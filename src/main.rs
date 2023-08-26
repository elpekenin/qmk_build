use std::process::exit;

use clap::Parser;
mod cli;

mod config;

mod git;

mod logging;
use config::BuildFile;
use git::Repository;
use logging::{info, log, paris, error};

mod operations;
use operations::prelude::OperationTrait;

mod sh;

// Pack together any information that operations might need
pub struct BuildConfig {
    git_repo: Repository,
    build_file: BuildFile,
}

impl BuildConfig {
    fn new() -> Self {
        // Parse CLI args
        let cli_args = cli::Args::parse();

        info!("Welcome to <blue>QMK build (alpha)</>");

        // (try) Load build configuration
        let file = &cli_args.file;
        let build_file = match BuildFile::load(file) {
            Ok(config) => {
                info!("Loaded <blue>{file}</>",);
                config
            }
            Err(e) => {
                error!(
                    "Parsing config file (<blue>{file}</>)\n\t<red>{}</>",
                    e.to_string()
                );
                exit(1);
            }
        };

        let git_repo = git::Repository::init(
            &build_file.path,
            &build_file.repo,
            &build_file.branch
        );

        Self {
            git_repo,
            build_file,
        }
    }

    fn copy_binaries(&self) {
        let binaries = "binaries/";
        let _ = sh::run(format!("mkdir -p {binaries}"), ".", true);
        for ext in ["bin", "hex", "uf2"] {
            let _ = sh::run(
                format!("cp {}/*.{ext} {binaries}", self.git_repo.path),
                ".",
                false,
            );
        }
        info!("Copied into <blue>{binaries}</>");
    }

    fn default_compilation(&self) {
        let mut command = String::from("qmk compile");

        if let Some(keyboard) = &self.build_file.keyboard {
            command.push_str(&format!(" -kb {keyboard}"));
        }

        if let Some(keymap) = &self.build_file.keymap {
            command.push_str(&format!(" -km {keymap}"));
        }

        let _ = self.git_repo.run("qmk clean -a", true);
        let _ = self.git_repo.run(command, true);
    }
}

// Entrypoint for the app
fn main() {
    logging::init();

    // Parse CLI args
    //   - Early exit after handling some flag
    //   - Read build settings + configure git repo otherwise
    let config = BuildConfig::new();

    // Apply changes listed on the file
    for operation in &config.build_file.operations {
        info!("{}", operation.message());
        operation.apply(&config);
    }

    // Compile (if asked)
    if config.build_file.default_compilation {
        info!("Compiling");
        config.default_compilation();
    }

    config.copy_binaries();

    info!("<green>Finished</>");
    exit(0);
}
