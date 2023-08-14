use schemars::JsonSchema;
use serde::Deserialize;

use crate::{logging::{info, log, paris}, sh, BuildConfig};

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct Cp {
    description: Option<String>,
    // origin path
    orig: String,
    // destination path
    dest: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct Diff {
    description: Option<String>,
    // path file to be applied
    patch: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct Checkout {
    description: Option<String>,
    // repo to check from
    repo: String,
    // branch of such repo
    branch: String,
    // files/folders being pulled
    files: Vec<String>,
}

fn default_exec_at() -> String {
    ".".to_owned()
}

fn default_strict() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct Exec {
    description: Option<String>,
    // command to execute
    command: String,
    // where to do so (defaults to wherever the CLI is invoked)
    #[serde(default = "default_exec_at")]
    at: String,
    // allowed to fail (defaults to no)
    #[serde(default = "default_strict")]
    strict: bool,
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct Script {
    description: Option<String>,
    // script to execute (eg a python script, thanks to shebang)
    file: String,
    #[serde(default = "default_strict")]
    strict: bool,
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct PullRequest {
    description: Option<String>,
    // repo where the PR was made (default to file's repo field)
    repo: Option<String>,
    // PR id. FIXME: May not work on anything other than GitHub
    id: u16
}

// #########
// Aggregate
// #########

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(tag = "operation", rename_all = "snake_case")]
/// Different patches to be applied to initial state of the repo
pub enum Operation {
    /// Can be used on files or folders, copy whatever contents
    Cp(Cp),

    /// Apply diff on a file
    Diff(Diff),

    /// Grab files/folders from another branch (and repo?)
    Checkout(Checkout),

    /// Execute a command
    Exec(Exec),

    /// Execute a file
    Script(Script),

    /// Grab diff from a PR
    Pr(PullRequest),
}

impl Operation {
    fn description(&self) -> &Option<String> {
        match self {
            Operation::Cp(op) => &op.description,
            Operation::Diff(op) => &op.description,
            Operation::Checkout(op) => &op.description,
            Operation::Exec(op) => &op.description,
            Operation::Script(op) => &op.description,
            Operation::Pr(op) => &op.description,
        }
    }

    fn message(&self) -> String {
        match self {
            Operation::Cp(op) => format!("Applying cd: <blue>{}</> <green>-></> <blue>{}</>", op.orig, op.dest),
            Operation::Diff(op) => format!("Applying patch: <blue>{}</>", op.patch),
            Operation::Checkout(op) => format!("Checking out <blue>{:?}</> from <blue>{}</> <green>@</> <blue>{}</>", op.files, op.repo, op.branch),
            Operation::Exec(op) => {
                let can_fail = if op.strict { " <red>not</>" } else { "" };
                format!("Executing <blue>{}</> at <blue>{}</> -- It can{} fail", &op.command, &op.at, can_fail)
            },
            Operation::Script(op) => format!("Running script <blue>{}</>", &op.file),
            Operation::Pr(op) => format!("Merging changes from <blue>#{}</>", op.id),
        }
    }

    pub fn apply(&self, state: &BuildConfig) {
        let mut message = self.message();

        if let Some(description) = &self.description() {
            message.push_str(&format!(" <cyan>[{description}]</>"));
        }

        info!("{}", message);

        match self {
            Operation::Cp(ref op) => {
                let orig = &op.orig;
                let dest = &format!("{}/{}", &state.git_repo.path, &op.dest);
                let _ = sh::run(format!("cp -r {orig} {dest}"), ".", true);
            }
            Operation::Diff(ref op) => {
                let _ = sh::run(
                    format!("cp {} {}", op.patch, state.git_repo.path),
                    ".",
                    true,
                );
                state.git_repo.apply(&op.patch);
            }
            Operation::Checkout(ref op) => {
                state.git_repo.remote_add(&op.repo);
                state.git_repo.fetch(&op.repo, None);
                state
                    .git_repo
                    .checkout(&op.repo, &op.branch, Some(&op.files));
            }
            Operation::Exec(ref op) => {
                let command = op.command.clone();
                sh::run(command, &op.at, op.strict);
            }
            Operation::Script(ref op) => {
                let file = op.file.clone();
                sh::run(file, ".", op.strict);
            },
            Operation::Pr(ref op) => {
                let local_branch_name = &format!("PR_{}", op.id);
                let fetch_remote_branch = &format!("pull/{}/head:{}", op.id, local_branch_name);
                
                state.git_repo.fetch(op.repo.as_ref().unwrap_or(&state.build_file.repo), Some(fetch_remote_branch));
                state.git_repo.merge_local_branch(local_branch_name);
            }
        }   
    }
}
