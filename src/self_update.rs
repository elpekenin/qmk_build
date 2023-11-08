// Tiny logic so the tool re-compiles itself

use crate::sh;

pub fn detect_changes() -> bool {
    // injected by build.rs
    let build_timestamp = String::from(env!("BUILD_TIMESTAMP"));

    let output = sh::run(r#"find src/ -printf "%Ts\n" | sort -nr | head -n 1"#, env!("CARGO_MANIFEST_DIR"), true);
    let last_src_change = String::from_utf8(output.stdout).expect("Couldn't convert last change to string");

    last_src_change > build_timestamp
}

pub fn compile() {
    sh::run("cargo install --path .", env!("CARGO_MANIFEST_DIR"), true);
}