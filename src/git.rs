use std::{
    collections::hash_map::DefaultHasher,
    ffi::OsStr,
    fmt::Display,
    hash::{Hash, Hasher},
    process::{exit, Output},
};

use schemars::JsonSchema;
use serde::Deserialize;

use crate::{
    logging::{error, info, log, paris},
    sh,
};

pub struct Repository {
    // foler where the repo is
    pub path: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Strategy {
    Octopus,
    Ours,
    Recursive,
    Resolve,
    Subtree,
}

impl Default for Strategy {
    fn default() -> Self {
        Self::Recursive
    }
}
impl ToString for Strategy {
    fn to_string(&self) -> String {
        format!("{self:?}").to_lowercase()
    }
}

impl Repository {
    pub fn run<S: AsRef<OsStr> + Clone + Display>(&self, command: S, strict: bool) -> Output {
        sh::run(command, &self.path, strict)
    }

    fn remote(repo: &String) -> String {
        let mut hasher = DefaultHasher::new();
        repo.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    pub fn init(path: impl Into<String>, repo: &String, branch: &String) -> Self {
        let path = path.into();
        let self_ = Self { path };

        // clone repo if path doesnt exist yet
        if sh::run(format!("cd {}", self_.path), ".", false)
            .status
            .code()
            != Some(0)
        {
            info!("Cloning <blue>{repo}</>, this may take a while...");
            self_.clone(repo, branch);
        }

        info!("Repo at <blue>{}</>", self_.path);

        // Get into desired branch
        // Remove any stray change
        // Delete branches other than working one
        self_.reset_hard();
        self_.remote_add(repo);
        self_.fetch(repo, None);
        self_.checkout(repo, branch, None);
        self_.clean();
        self_.run(
            format!("git branch | grep -v '{branch}' | xargs --no-run-if-empty git branch -D"),
            true,
        );

        info!("Working based on <blue>{repo}</> <green>@</> <blue>{branch}</>");

        info!("Synchronizing submodules, this may take a while...");
        let _ = self_.run("qmk git-submodule", true);

        self_
    }

    pub fn clone(&self, repo: &String, branch: &String) {
        let path = &self.path;
        let remote = Self::remote(repo);

        let _ = sh::run(
            format!("git clone {repo} -b {branch} -o {remote} {path}"),
            ".",
            true,
        );
    }

    pub fn remote_add(&self, repo: &String) {
        let remote = Self::remote(repo);

        let output = self.run(format!("git remote add {remote} {repo}"), false);

        if output.status.code() != Some(0) {
            let stderr = String::from_utf8(output.stderr).unwrap();
            if !stderr.contains("already exists") {
                error!("Adding remote failed with\n\t<red>{stderr}</>");
                exit(1);
            }
        }
    }

    pub fn fetch(&self, repo: &String, branch: Option<&String>) {
        let remote = Self::remote(repo);

        let mut command = format!("git fetch {remote}");
        if let Some(branch) = branch {
            command.push_str(&format!(" {branch}"));
        }

        let _ = self.run(command, true);
    }

    pub fn checkout(&self, repo: &String, branch: &String, files: Option<&Vec<String>>) {
        let remote = Self::remote(repo);

        let mut command = format!("git checkout {remote}/{branch}");
        if let Some(files) = files {
            command.push_str(" --");

            for file in files {
                command.push_str(&format!(" {file}"));
            }
        }

        let _ = self.run(command, true);
    }

    pub fn apply(&self, file: &String) {
        let _ = self.run(format!("git apply {file}"), true);
    }

    pub fn reset_hard(&self) {
        let _ = self.run("git reset --hard", true);
    }

    pub fn clean(&self) {
        let _ = self.run("git clean -dfx", true);
    }

    pub fn merge(&self, repo: Option<&String>, branches: &[String], strategy: Option<Strategy>) {
        let repo = match repo {
            Some(repo) => format!("{}/", Repository::remote(repo)),
            None => String::new(),
        };

        let branches = branches.join(" ");

        let strategy = strategy.unwrap_or_default();
        let strategy = format!(" -s {}", strategy.to_string());

        let command = format!("git merge {repo}{branches}{strategy}");
        let _ = self.run(command, true);
    }
}
