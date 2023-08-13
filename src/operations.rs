use schemars::JsonSchema;
use serde::Deserialize;

use crate::{logging::{info, log, paris}, sh, BuildConfig};

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct Cp {
    // origin path
    orig: String,
    // destination path
    dest: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct Diff {
    // path file to be applied
    patch: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct Checkout {
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
    // script to execute (eg a python script, thanks to shebang)
    file: String,
    #[serde(default = "default_strict")]
    strict: bool,
}

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
}

impl Operation {
    pub fn apply(&self, state: &BuildConfig) {
        match self {
            Operation::Cp(ref cp) => {
                let orig = &cp.orig;
                let dest = &format!("{}/{}", &state.git_repo.path, &cp.dest);

                info!(
                    "Applying cd: <blue>{}</> <green>-></> <blue>{}</>",
                    orig, dest
                );

                let _ = sh::run(format!("cp -r {orig} {dest}"), ".", true);
            }
            Operation::Diff(ref diff) => {
                info!("Applying patch: <blue>{}</>", diff.patch);

                let _ = sh::run(
                    format!("cp {} {}", diff.patch, state.git_repo.path),
                    ".",
                    true,
                );
                state.git_repo.apply(&diff.patch);
            }
            Operation::Checkout(ref checkout) => {
                info!(
                    "Checking out <blue>{:?}</> from <blue>{}</> <green>@</> <blue>{}</>",
                    checkout.files, checkout.repo, checkout.branch
                );

                state.git_repo.remote_add(&checkout.repo);
                state.git_repo.fetch(&checkout.repo);
                state
                    .git_repo
                    .checkout(&checkout.repo, &checkout.branch, Some(&checkout.files));
            }
            Operation::Exec(ref exec) => {
                let can_fail = if exec.strict { " <red>not</>" } else { "" };
                info!(
                    "Executing <blue>{}</> at <blue>{}</> -- It can{} fail",
                    &exec.command, &exec.at, can_fail
                );

                let command = exec.command.clone();
                sh::run(command, &exec.at, exec.strict);
            }
            Operation::Script(ref script) => {
                info!("Running script <blue>{}</>", &script.file);

                let file = script.file.clone();
                sh::run(file, ".", script.strict);
            }
        }
    }
}
