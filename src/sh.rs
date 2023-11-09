use std::{
    ffi::OsStr,
    fmt::Display,
    process::{exit, Command, Output},
};

use crate::logging;

fn format_buf(buf: &[u8]) -> String {
    String::from_utf8_lossy(buf).replace('\n', "\n    ")
}

// Run a command on the shell, at a given location, allowing it (or not) to fail
pub fn run<S: AsRef<OsStr> + Clone + Display>(
    command: S,
    at: impl Into<String>,
    strict: bool,
) -> Output {
    let final_command = format!("cd {} && {}", at.into(), command);

    logging::debug!("Running command: {final_command}");

    let output = match Command::new("sh").arg("-c").arg(final_command).output() {
        Ok(output) => output,
        Err(e) => {
            logging::error!(
                "Couldn't run command <red>{command}</>\n\t<red>{:?}</>",
                e.to_string().replace('\n', "\n\t")
            );
            exit(1);
        }
    };

    if strict && output.status.code() != Some(0) {
        logging::error!(
            r#"Running command <yellow>{command}</>
    STDOUT
    ------
    <red>{}</>

    STDERR
    ------
    <red>{}</>"#,
            format_buf(&output.stdout),
            format_buf(&output.stderr),
        );
        exit(1);
    }

    output
}
