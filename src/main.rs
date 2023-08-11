use std::process::exit;

use clap::Parser;
mod cli;

mod git;

mod json_config;

mod logging;
use logging::*;

mod operations;

mod sh;


/// Shared app state
struct State<'a> {
    // Location where CLI was invoked
    cwd: &'a String,
    // Location of the repo
    path: &'a String,
}

/// Entrypoint for the app
fn main() {
    logging::init();

    let args = cli::Args::parse();
    info!("Welcome to <blue>QMK build (alpha)</>");

    // Read config file
    let cwd = sh::get_cwd();
    let file = format!("{cwd}/{}", args.file);
    let user_config = json_config::read_from(&file);
    info!("Loaded <blue>{file}</>",);

    // Shared app state
    let state = State {
        cwd: &cwd, 
        path: &user_config.path,
    };

    let repo = &user_config.repo;
    let branch = &user_config.branch;
    let path = &user_config.path;

    // clone repo if path doesnt exist yet
    if sh::run(format!("cd {path}")).status.code() != Some(0) {
        info!("{path:?}");
        info!("Cloning <blue>{repo}</>, this may take a while...");
        git::clone(repo, branch, path);
    }
    info!("Repo at <blue>{path}</>");


    // clone initial repo @ branch
    git::remote_add(repo, path);
    git::fetch(repo, path);
    git::checkout(repo, branch, None, path);

    // restore potential diff's
    git::restore_staged(path);
    git::restore(path);
    git::clean(path); // remove untracked files

    info!("Working based on <blue>{repo}</> <yellow>@</> <blue>{branch}</>");


    info!("Synchronizing submodules, this may take a while...");
    let _ = sh::run_strict_at(path, "qmk git-submodule".to_owned());


    // Apply operations
    for operation in user_config.operations {
        operation.apply(&state);
    }


    // Compile
    let mut command = String::from("qmk compile");

    if let Some(keyboard) = user_config.keyboard {
        command.push_str(&format!(" -kb {keyboard}"));
    }

    if let Some(keymap) = user_config.keymap {
        command.push_str(&format!(" -km {keymap}"));
    }

    info!("Compiling");
    let _ = sh::run_strict_at(path, "qmk clean -a".to_owned());
    let _ = sh::run_strict_at(path, command);

    let binaries = "binaries/";
    let _ = sh::run(format!("mkdir -p {binaries}"));
    for ext in ["bin", "hex", "uf2"] {
        let _ = sh::run(format!("cp {path}/*.{ext} {binaries}"));
    }
    info!("Copied into <blue>{binaries}</>");


    info!("<green>Finished</>");
    exit(0);
}
