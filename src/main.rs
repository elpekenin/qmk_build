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

pub mod sh;

// Pack together any information that operations might need
pub struct BuildConfig {
    git_repo: Repository,
    build_file: BuildFile,
}

impl BuildConfig {
    fn new() -> Self {
        // Parse CLI args
        let cli_args = cli::Args::parse();

        info!("Welcome to <blue>QMK build (beta)</>");

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
        // create (if needed) and clear the output directory
        let binaries = "binaries/";
        let _ = sh::run(format!("mkdir -p {binaries}"), ".", true);
        let _ = sh::run(format!("rm {binaries}/*"), ".", true);

        // copy firmwares into output dir
        for ext in ["bin", "hex", "uf2"] {
            let _ = sh::run(
                format!("cp {}/*.{ext} {binaries}", self.git_repo.path),
                ".",
                false,
            );
        }

        info!("Copied into <blue>{binaries}</>");
    }

    fn set_config(&self, key: impl Into<String>, value: &Option<String>) -> Option<String> {
        let key = key.into();

        if let Some(value) = value {
            let stdout = self.git_repo.run(format!("qmk config {key} | cut -d '=' -f2"), true).stdout;

            let prev_value = match String::from_utf8_lossy(&stdout).to_string().as_str() {
                "None" => None,
                value => Some(String::from(value)),
            };

            let _ = self.git_repo.run(format!("qmk config {key}={value}"), true);

            prev_value
        } else {
            None
        }
    }

    fn default_compilation(&self) {
        // configure keyboard and keymap
        let prev_keyboard = self.set_config("user.keyboard", &self.build_file.keyboard);
        let prev_keymap = self.set_config("user.keymap", &self.build_file.keymap);

        // compile
        let _ = self.git_repo.run("qmk clean -a", true);
        let _ = self.git_repo.run("qmk compile", true);

        // restore
        let _ = self.set_config("user.keyboard", &prev_keyboard);
        let _ = self.set_config("user.keymap", &prev_keymap); 
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
