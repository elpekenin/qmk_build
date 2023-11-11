use std::process::Command;

fn main() {
    let output = Command::new("date").args(["+%s"]).output().unwrap();
    let build_timestamp = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=BUILD_TIMESTAMP={build_timestamp}");

    let output = Command::new("date")
        .args(["+%e %b %Y %X"])
        .output()
        .unwrap();
    let curr_time = String::from_utf8(output.stdout).unwrap().replace('\n', "");
    let pkg_version = String::from(std::env!("CARGO_PKG_VERSION"));
    let version = format!("{pkg_version} (built at {curr_time})");
    println!("cargo:rustc-env=VERSION={version}");
}
