use std::process::Command;

fn main() {
    // save timestamp of build, to detect changes
    let output = Command::new("date").args(["+%s"]).output().unwrap();
    let build_timestamp = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=BUILD_TIMESTAMP={build_timestamp}");

    // build pretty version string
    let pkg_version = String::from(std::env!("CARGO_PKG_VERSION"));

    let curr_time = String::from_utf8(
        Command::new("date")
            .args(["+%e %b %Y %H:%M"])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
    .replace('\n', "");

    let binding =
        String::from_utf8(Command::new("rustc").args(["-V"]).output().unwrap().stdout).unwrap();
    let rustc_version = binding.split(' ').collect::<Vec<_>>()[1];

    let final_version = format!("{pkg_version}. Built at {curr_time} with Rust {rustc_version}");
    println!("cargo:rustc-env=VERSION={final_version}");
}
