// Tiny logic so the tool re-compiles itself

use std::process::{exit, Command, Output, Stdio};

use crate::logging;

const PROJECT_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn run_cmd(cmd: &str, redirect: bool) -> Output {
    let mut command = Command::new("sh");

    command.arg("-c").arg(format!("cd {PROJECT_DIR} && {cmd}"));

    if redirect {
        command.stdin(Stdio::null());
        command.stdout(Stdio::null());
        command.stderr(Stdio::inherit());
    }

    command
        .output()
        .unwrap_or_else(|_| panic!("Couldn't run the command: {cmd}"))
}

pub fn detect_changes() -> bool {
    // injected by build.rs
    let build_timestamp = String::from(env!("BUILD_TIMESTAMP"));

    // checking at MANIFEST_DIR we can detect changes on src, build, and TOML
    // exclude target, otherwise new binary would be seen as a change, and we always land here
    let output = run_cmd(
        r#"find -path . -not -path ./target -not -path ./.git -printf "%Ts\n" | sort -nr | head -n 1"#,
        false,
    );

    let last_change =
        String::from_utf8(output.stdout).expect("Couldn't convert last change to string");

    last_change > build_timestamp
}

pub fn compile() {
    // format + lint code
    run_cmd("cargo fmt", false);
    run_cmd("cargo clippy -- -Wclippy::pedantic -Dwarnings", false);

    // re-gen schema
    run_cmd("cargo test", false);

    // compile + install executable
    //   true => cargo output redirected to parent (qmk_build)
    //           only warnings/errors, info messages silenced with --quiet
    let status = run_cmd("cargo install --quiet --path .", true).status;
    if status.success() {
        logging::warn!("Done. Can compile firmware now ^^");
    } else {
        logging::warn!("Source code is broken. Please fix me :(");
    }

    exit(
        status
            .code()
            .expect("How did self-compile end by signal???"),
    );
}
