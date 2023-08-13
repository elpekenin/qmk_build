use std::{fs::File, io::Write, process::exit};

use clap::Parser;
mod cli;

mod config;

mod git;

mod logging;
use config::BuildFile;
use git::Repository;
use logging::{info, log, paris};
use schemars::schema_for;

mod operations;

mod sh;

/// Pack together any information that operations might need
pub struct BuildConfig {
    git_repo: Repository,
    build_file: BuildFile,
}

impl BuildConfig {
    fn new() -> Self {
        let cli_args = cli::Args::parse();

        if cli_args.generate_schema {
            let schema = schema_for!(BuildFile);
            let schema_str = serde_json::to_string_pretty(&schema).unwrap();

            let mut file = File::create("schema").unwrap();
            let _ = file.write_all(schema_str.as_bytes());

            info!("Schema generated");
            exit(0)
        }

        let build_file = config::read_from(&cli_args.file);
        let git_repo =
            git::Repository::init(&build_file.path, &build_file.repo, &build_file.branch);

        info!("Welcome to <blue>QMK build (alpha)</>");
        Self {
            git_repo,
            build_file,
        }
    }

    pub fn setup(&self) {
        for operation in &self.build_file.operations {
            operation.apply(self);
        }
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

    pub fn compile(&self) {
        if self.build_file.default_compilation {
            self.default_compilation();
        }
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
