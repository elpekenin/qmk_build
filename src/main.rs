use std::process::exit;

use clap::Parser;
mod cli;

mod config;

mod git;

mod logging;
use cli::CliArgs;
use config::BuildFile;
use git::GitRepo;
use logging::*;

mod operations;

mod sh;

/// Pack together any information that operations might need
pub struct BuildConfig {
    cli_args: CliArgs,
    git_repo: GitRepo,
    build_file: BuildFile,
}

impl BuildConfig {
    fn new() -> Self {
        info!("Welcome to <blue>QMK build (alpha)</>");

        let cli_args = cli::CliArgs::parse();
        let build_file = config::read_from(&cli_args.file);
        let git_repo = git::GitRepo::init(&build_file.path, &build_file.repo, &build_file.branch);

        Self {
            cli_args,
            git_repo,
            build_file,
        }
    }

    pub fn setup(&self) {
        for operation in &self.build_file.operations {
            operation.apply(self);
        }
    }

    pub fn compile(&self) {
        // Compile
        let mut command = String::from("qmk compile");

        if let Some(keyboard) = &self.build_file.keyboard {
            command.push_str(&format!(" -kb {keyboard}"));
        }

        if let Some(keymap) = &self.build_file.keymap {
            command.push_str(&format!(" -km {keymap}"));
        }

        let _ = self.git_repo.run("qmk clean -a", true);
        let _ = self.git_repo.run(command, true);

        let binaries = "binaries/";
        let _ = sh::run(format!("mkdir -p {binaries}"), ".", true);
        for ext in ["bin", "hex", "uf2"] {
            let _ = sh::run(format!("cp {}/*.{ext} {binaries}", self.git_repo.path), ".", false);
        }
        info!("Copied into <blue>{binaries}</>");
    }
}

/// Entrypoint for the app
fn main() {
    logging::init();

    // Setup everything based on CLI args + build file
    let config = BuildConfig::new();

    config.setup();

    info!("Compiling");
    config.compile();

    info!("<green>Finished</>");
    exit(0);
}
