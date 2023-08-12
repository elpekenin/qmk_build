use serde::Deserialize;

use crate::{logging::*, sh, BuildConfig};

#[derive(Clone, Debug, Deserialize)]
pub struct Cp {
    // origin path
    orig: String,
    // destination path
    dest: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Diff {
    // path file to be applied
    patch: String,
}

#[derive(Clone, Debug, Deserialize)]
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

#[derive(Clone, Debug, Deserialize)]
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

#[derive(Clone, Debug, Deserialize)]
pub struct Script {
    // script to execute (eg a python script, thanks to shebang)
    file: String,
    #[serde(default = "default_strict")]
    strict: bool,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "operation")]
/// Different patches to be applied to initial state of the repo
pub enum Operation {
    /// Can be used on files or folders, copy whatever contents
    cp(Cp),

    /// Apply diff on a file
    diff(Diff),

    /// Grab files/folders from another branch (and repo?)
    checkout(Checkout),

    /// Execute a command
    exec(Exec),

    /// Execute a file
    script(Script),
}
#[warn(non_camel_case_types)]

impl Operation {
    pub fn apply(&self, state: &BuildConfig) {
        match self {
            Operation::cp(ref cp) => {
                let orig = &cp.orig;
                let dest = &format!("{}/{}", &state.git_repo.path, &cp.dest);

                info!(
                    "Applying cd: <blue>{}</> <green>-></> <blue>{}</>",
                    orig, dest
                );

                let _ = sh::run(format!("cp -r {orig} {dest}"), ".", true);
            }
            Operation::diff(ref diff) => {
                info!("Applying patch: <blue>{}</>", diff.patch);

                let _ = sh::run(
                    format!("cp {} {}", diff.patch, state.git_repo.path),
                    ".",
                    true,
                );
                state.git_repo.apply(&diff.patch);
            }
            Operation::checkout(ref checkout) => {
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
            Operation::exec(ref exec) => {
                let can_fail = if exec.strict { " <red>not</>" } else { "" };
                info!(
                    "Executing <blue>{}</> at <blue>{}</> -- It can{} fail",
                    &exec.command, &exec.at, can_fail
                );

                let command = exec.command.clone();
                sh::run(command, &exec.at, exec.strict);
            }
            Operation::script(ref script) => {
                info!("Running script <blue>{}</>", &script.file);

                let file = script.file.clone();
                sh::run(file, ".", script.strict);
            }
        }
    }
}
