// Tiny logic so the tool re-compiles itself

use crate::sh;

const PROJECT_DIR: &str = env!("CARGO_MANIFEST_DIR");

pub fn detect_changes() -> bool {
    // injected by build.rs
    let build_timestamp = String::from(env!("BUILD_TIMESTAMP"));

    // checking at MANIFEST_DIR we can detect changes on src, build, and TOML
    // exclude target, otherwise new binary would be seen as a change, and we always land here
    let output = sh::run(
        r#"find -not -path "./target/*" -printf "%Ts\n" | sort -nr | head -n 1"#,
        PROJECT_DIR,
        true,
    );

    let last_change =
        String::from_utf8(output.stdout).expect("Couldn't convert last change to string");

    last_change > build_timestamp
}

pub fn compile() -> std::process::ExitStatus {
    // format + lint code
    sh::run("cargo fmt", PROJECT_DIR, true);
    sh::run(
        "cargo clippy -- -Wclippy::pedantic -Dwarnings",
        PROJECT_DIR,
        true,
    );

    // re-gen schema
    sh::run("cargo test", PROJECT_DIR, true);

    // compile + install executable
    sh::run("cargo install --path .", PROJECT_DIR, false).status
}
