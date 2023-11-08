use std::process::Command;

fn main() {
    let output = Command::new("date").args(["+%s"]).output().unwrap();
    let build_timestamp = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=BUILD_TIMESTAMP={build_timestamp}");
}