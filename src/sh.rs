use std::{
    ffi::OsStr,
    fmt::Display,
    process::{exit, Command, Output},
};

use crate::logging::{debug, error, log, paris};

// Run a command on the shell, at a given location, allowing it (or not) to fail
pub fn run<S: AsRef<OsStr> + Clone + Display>(
    command: S,
    at: impl Into<String>,
    strict: bool,
) -> Output {
    let final_command = format!("cd {} && {}", at.into(), command);

    debug!("Running command: {final_command}");

    let output = match Command::new("sh").arg("-c").arg(final_command).output() {
        Ok(output) => output,
        Err(e) => {
            error!(
                "Couldn't run command <red>{command}</>\n\t<red>{:?}</>",
                e.to_string().replace('\n', "\n\t")
            );
            exit(1);
        }
    };

    if strict && output.status.code() != Some(0) {
        error!(
            r#"Running command <yellow>{command}</>
    STDOUT
    ------
    <red>{}</>

    STDERR
    ------
    <red>{}</>"#,
            String::from_utf8(output.stdout)
                .unwrap()
                .replace('\n', "\n\t"),
            String::from_utf8(output.stderr)
                .unwrap()
                .replace('\n', "\n\t")
        );
        exit(1);
    }

    output
}
