use std::process::exit;

pub mod build;
mod cli;
pub mod git;
mod logging;
#[allow(unused_variables)]
mod operations;
mod self_update;
pub mod sh;

use clap::Parser;
use logging::{log, paris};
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

fn copy_binaries(git_repo: &git::Repository) {
    // create (if needed) and clear the output directory
    let binaries = "binaries/";
    let _ = sh::run(format!("mkdir -p {binaries}"), ".", true);
    let _ = sh::run(format!("rm -f {binaries}/*"), ".", true);

    // copy firmwares into output dir
    for ext in ["bin", "hex", "uf2"] {
        let _ = sh::run(
            format!("cp {}/*.{ext} {binaries}", git_repo.path),
            ".",
            false,
        );
    }

    logging::info!("Copied into <blue>{binaries}</>");
}

fn default_compilation(settings: &build::Settings, repository: &git::Repository) {
    logging::info!(
        "Compiling <blue>{:?}</> <green>:</> <blue>{:?}</>",
        settings.keyboard,
        settings.keymap,
    );

    // setup the command to be run
    let mut cmd = String::from("qmk compile");
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

    // parse CLI args
    let cli_args = cli::Args::parse();

    // recompile the tool if source was changed
    if self_update::detect_changes() {
        self_update::compile();
        log::warn!("Detected changes and re-compiled myself, try building your firmware now");
        exit(0); 
    }

    logging::info!("Welcome to <blue>QMK build (beta)</>");

    // (try) load build configuration
    let settings = read_settings(&cli_args.file);

    let repository = git::Repository::init(
        &settings.path, 
        &settings.repo,
        &settings.branch
    );

    // apply changes listed on the file
    for operation in &settings.operations {
        logging::info!("{}", operation.message());
        operation.apply(&settings, &repository);
    }

    // compile (if asked)
    if settings.default_compilation {
        default_compilation(&settings, &repository);
    }

    copy_binaries(&repository);

    // post-compile callback
    for operation in &settings.post_compilation {
        logging::info!("{}", operation.message());
        operation.apply(&settings, &repository);
    }

    logging::info!("<green>Finished</>");
    exit(0);
}
