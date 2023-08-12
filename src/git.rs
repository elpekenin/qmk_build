use std::{
    collections::hash_map::DefaultHasher,
    ffi::OsStr,
    fmt::Display,
    hash::{Hash, Hasher},
    process::{exit, Output},
};

use crate::{logging::*, sh};

fn hash(repo: &String) -> String {
    let mut hasher = DefaultHasher::new();
    repo.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

pub struct GitRepo {
    // foler where the repo is
    pub path: String,
}

impl GitRepo {
    pub fn run<S: AsRef<OsStr> + Clone + Display>(&self, command: S, strict: bool) -> Output {
        sh::run(command, &self.path, strict)
    }

    pub fn init(path: impl Into<String>, repo: &String, branch: &String) -> Self {
        let path = path.into();
        let _self = Self { path };

        // clone repo if path doesnt exist yet
        if sh::run(format!("cd {}", _self.path), ".", false)
            .status
            .code()
            != Some(0)
        {
            info!("Cloning <blue>{repo}</>, this may take a while...");
            _self.clone(repo, branch);
        }

        info!("Repo at <blue>{}</>", _self.path);

        _self.remote_add(repo);
        _self.fetch(repo);
        _self.checkout(repo, branch, None);

        _self.restore_staged();
        _self.restore();
        _self.clean();

        info!("Working based on <blue>{repo}</> <green>@</> <blue>{branch}</>");

        info!("Synchronizing submodules, this may take a while...");
        let _ = _self.run("qmk git-submodule", true);

        _self
    }

    pub fn clone(&self, repo: &String, branch: &String) {
        let path = &self.path;
        let remote = hash(repo);

        let _ = sh::run(
            format!("git clone {repo} -b {branch} -o {remote} {path}"),
            ".",
            true,
        );
    }

    pub fn remote_add(&self, repo: &String) {
        let remote = hash(repo);

        let output = self.run(format!("git remote add {remote} {repo}"), false);

        if output.status.code() != Some(0) {
            let stderr = String::from_utf8(output.stderr).unwrap();
            if !stderr.contains("already exists") {
                error!("Adding remote failed with\n\t<red>{stderr}</>");
                exit(1);
            }
        }
    }

    pub fn fetch(&self, repo: &String) {
        let remote = hash(repo);
        let _ = self.run(format!("git fetch {remote}"), true);
    }

    pub fn checkout(&self, repo: &String, branch: &String, files: Option<&Vec<String>>) {
        let remote = hash(repo);

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

    pub fn restore_staged(&self) {
        let _ = self.run("git restore --staged .", true);
    }

    pub fn restore(&self) {
        let _ = self.run("git restore .", true);
    }

    pub fn clean(&self) {
        let _ = self.run("git clean -f -x", true);
    }
}
