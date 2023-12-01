pub mod build;
mod cli;
pub mod git;
#[macro_use]
pub mod logging;
#[allow(unused_variables)]
mod operations;
mod self_update;
pub mod sh;

use std::process::exit;

use clap::Parser;
use operations::prelude::OperationTrait;

fn read_settings(file: &String) -> build::Settings {
    match build::Settings::load(file) {
        Ok(config) => {
            logging::info!("Loaded <blue>{file}</>",);
            config
        }
        Err(e) => {
            logging::error!(
                "Parsing config file (<blue>{file}</>)\n\t<red>{}</>",
                e.to_string()
            );
            exit(1);
        }
    }
}

fn is_wsl() -> bool {
    let stdout = match String::from_utf8(sh::run("uname -a", ".", false).stdout) {
        Ok(str) => str,
        Err(_) => {
            return false;
        }
    };

    stdout.to_lowercase().contains("wsl")
}

fn copy_binaries(settings: &build::Settings, git_repo: &git::Repository) {
    // create (if needed) and clear the output directory
    let mut dest_folder = String::from("binaries/");

    if is_wsl() && settings.wsl.is_some() {
        dest_folder = settings.wsl.clone().unwrap().destination;
    }

    let _ = sh::run(format!("mkdir -p {dest_folder}"), ".", true);

    // copy firmwares into output dir
    for ext in ["bin", "hex", "uf2"] {
        let _ = sh::run(
            format!("cp {}/*.{ext} {dest_folder}", git_repo.path),
            ".",
            false,
        );
    }

    logging::info!("Copied into <blue>{dest_folder}</>");
}

fn compile(settings: &build::Settings, repository: &git::Repository) {
    logging::info!(
        "Compiling <blue>{:?}</> <green>:</> <blue>{:?}</>",
        settings.keyboard,
        settings.keymap,
    );

    // setup the command to be run
    let mut cmd = settings
        .compile_command
        .clone()
        .unwrap_or(String::from("qmk compile"));

    if let Some(kb) = &settings.keyboard {
        cmd.push_str(&format!(" -kb {kb}"));
    }

    if let Some(km) = &settings.keymap {
        cmd.push_str(&format!(" -km {km}"));
    }

    // compile
    let _ = repository.run("qmk clean -a", true);
    let _ = repository.run(cmd, true);
}

// Entrypoint for the app
fn main() {
    logging::init();

    // recompile the tool if source was changed
    if self_update::detect_changes() {
        logging::warn!("Detected changes on my source code, attempting to re-compile myself...");
        self_update::compile();
    }

    // parse CLI args
    let cli_args = cli::Args::parse();

    logging::info!("Welcome to <blue>QMK build (beta)</>");

    // (try) load build configuration
    let settings = read_settings(&cli_args.file);

    let repository = git::Repository::init(&settings.path, &settings.repo, &settings.branch);

    // apply changes listed on the file
    for operation in &settings.operations {
        logging::info!("{}", operation.message());
        operation.apply(&settings, &repository);
    }

    compile(&settings, &repository);

    copy_binaries(&settings, &repository);

    // post-compile callback
    for operation in &settings.post_compilation {
        logging::info!("{}", operation.message());
        operation.apply(&settings, &repository);
    }

    logging::info!("<green>Finished</>");
    exit(0);
}
