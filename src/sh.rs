use std::{
    ffi::OsStr,
    fmt::Display,
    process::{exit, Command, Output},
};

use crate::logging::*;

/// Run a command on the shell
pub fn run<S: AsRef<OsStr> + Clone + Display>(command: S) -> Output {
    match Command::new("sh").arg("-c").arg(command.clone()).output() {
        Ok(output) => output,
        Err(e) => {
            error!(
                "Couldn't run command <red>{command}</>\n\t<red>{:?}</>",
                e.to_string().replace('\n', "\n\t")
            );
            exit(1);
        }
    }
}

/// Run a command on the shell, ensuring it returns success
pub fn run_strict<S: AsRef<OsStr> + Clone + Display>(command: S) -> Output {
    let output = run(command.clone());

    if output.status.code() != Some(0) {
        error!(
            "Running command <yellow>{command}</>\n\t<red>{}</>",
            String::from_utf8(output.stderr)
                .unwrap()
                .replace('\n', "\n\t")
        );
        exit(1);
    }

    output
}

pub fn run_at<S: AsRef<OsStr> + Clone + Display>(path: &S, command: S) -> Output {
    run(format!("cd {path} && {command}"))
}

pub fn run_strict_at<S: AsRef<OsStr> + Clone + Display>(path: &S, command: S) -> Output {
    run_strict(format!("cd {path} && {command}"))
}

pub fn get_cwd() -> String {
    String::from_utf8(run_strict("pwd").stdout)
        .unwrap()
        .replace('\n', "")
}