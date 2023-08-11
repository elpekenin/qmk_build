use clap::Parser;
mod cli;

mod json_config;

mod logging;
use logging::*;

mod sh;

use std::{process::exit, collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

/// Entrypoint for the app
fn main() {
    logging::init();

    let args = cli::Args::parse(); 
    info!("Welcome to <blue>QMK build (alpha)</>");
    
    // Read config file
    let pwd = String::from_utf8(
            sh::run_strict("pwd").stdout
        )
        .unwrap()
        .replace('\n', "");
    let file = format!("{pwd}/{}", args.file);
    let user_config = json_config::read_from(file.clone());
    info!("Loaded <blue>{file}</>",);

    // Setup git
    let repo = user_config.repo;
    let branch = user_config.branch;
    let workdir = &user_config.workdir;

    // unique remote name by using a hash
    let mut hasher = DefaultHasher::new();
    repo.hash(&mut hasher);
    let remote = format!("{:x}", hasher.finish());

    // cd into repo, clone beforehand if folder doesn't exist
    if sh::run(format!("cd {workdir}")).status.code() != Some(0) {
        info!("Cloning <blue>{repo}</>, this may take a while...");

        // clone with a name for the remote instead of the default "origin"
        let _ = sh::run_strict(format!("git clone {repo} -b {branch} -o {remote} {workdir}"));
    }
    info!("Repo at <blue>{workdir}</>");

    // configure initial state of the repo, before any patches
    let output = sh::run_at(workdir, format!("git remote add {remote} {repo}"));
    if output.status.code() != Some(0) {
        let stderr = String::from_utf8(output.stderr).unwrap();
        if !stderr.contains("already exists") { 
            error!("Adding remote failed with\n\t<red>{stderr}</>");
            exit(1);
        }
    }
    let _ = sh::run_strict_at(workdir, format!("git fetch {remote}"));
    let _ = sh::run_strict_at(workdir, format!("git checkout {remote}/{branch}"));
    info!("Working based on <blue>{repo}</> <yellow>/</> <blue>{branch}</>");

    info!("Synchronizing submodules, this may take a while...");
    let _ = sh::run_strict_at(workdir, "qmk git-submodule".to_owned());


    // Apply operations
    for operation in user_config.operations {
        info!("{operation:?}");
    }


    // #######
    // Compile
    // #######

    let mut command = String::from("qmk compile");

    if let Some(keyboard) = user_config.keyboard {
        command.push_str(&format!(" -kb {keyboard}"));
    } 
    
    if let Some(keymap) = user_config.keymap {
        command.push_str(&format!(" -km {keymap}"));
    } 

    info!("Compiling");
    let _ = sh::run_strict_at(workdir, "qmk clean -a".to_owned());
    let _ = sh::run_strict_at(workdir, command);

    let binaries = "binaries/";
    info!("Copying into <blue>{binaries}</>");
    let _ = sh::run(format!("mkdir -p {binaries}"));
    for ext in ["bin", "hex", "uf2"] {
        let _ = sh::run(format!("cp {workdir}/*.{ext} {binaries}"));
    }

    info!("<green>Finished</>");
    exit(0);
}
