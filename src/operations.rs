use serde::Deserialize;

use crate::{State, git, logging::*, sh};

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
    sh::get_cwd()
}

fn default_can_fail() -> bool {
    false
}

#[derive(Clone, Debug, Deserialize)]
pub struct Exec {
    // command to execute
    command: String,
    // where to do so (defaults to wherever the CLI is invoked)
    #[serde(default = "default_exec_at")]
    at: String,
    // allowed to fail (defaults to no)
    #[serde(default = "default_can_fail")]
    can_fail: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Script {
    // script to execute (eg a python script, thanks to shebang)
    file: String,
    #[serde(default = "default_can_fail")]
    can_fail: bool,
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
    pub fn apply(&self, state: &State) {
        match self {
            Operation::cp(ref cp) => {
                let orig = &cp.orig;
                let dest = &format!("{}/{}", &state.path, &cp.dest);

                info!("Applying cd: <blue>{}</> <yellow>-></> <blue>{}</>", orig, dest);

                let _ = sh::run_strict(format!("cp -r {orig} {dest}"));
            },
            Operation::diff(ref diff) => {
                info!("Applying patch: <blue>{}</>", diff.patch);

                let _ = sh::run_strict(format!("cp {} {}", diff.patch, state.cwd));
                git::apply(&diff.patch, state.path);
            },
            Operation::checkout(ref checkout) => {
                info!("Checking out <blue>{:?}</> from <blue>{}</> <yellow>@</> <blue>{}</>", checkout.files, checkout.repo, checkout.branch);

                git::remote_add(&checkout.repo, state.path);
                git::fetch(&checkout.repo, state.path);
                git::checkout(&checkout.repo, &checkout.branch, Some(&checkout.files), state.path);
            },
            Operation::exec(ref exec) => {
                let can_fail = if exec.can_fail {
                    ""
                } else {
                    " <red>not</>"
                };
                info!("Executing <blue>{}</> at <blue>{}</>. It can{} fail", &exec.command, &exec.at, can_fail);

                let command = exec.command.clone();
                if exec.can_fail {
                    sh::run_at(&exec.at, command);
                } else {
                    sh::run_strict_at(&exec.at, command);
                }
            },
            Operation::script(ref script) => {
                info!("Running script <blue>{}</>", &script.file);

                let file = script.file.clone();
                if script.can_fail {
                    sh::run(file);
                } else {
                    sh::run_strict(file);
                }
            }
        }
    }
}
