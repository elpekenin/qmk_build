use std::{
    error::Error,
    fmt::{Debug, Display},
    fs::File,
    io::BufReader,
    path::Path,
    process::exit,
};

use schemars::JsonSchema;
use serde::Deserialize;

use crate::{logging::*, operations::Operation};

fn default_path() -> String {
    String::from("$HOME/.__qmk_build__")
}

fn default_operations() -> Vec<Operation> {
    Vec::new()
}

fn _true() -> bool {
    true
}

/// Struct to define the contents expected on JSON file
#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct BuildFile {
    /// Required, initial state of the repo
    pub repo: String,
    pub branch: String,

    /// Optional, where the copy of the fork will be stored, defaults to $HOME/.__qmk_build__
    #[serde(default = "default_path")]
    pub path: String,

    /// Let user set a fixed value, otherwise let `qmk` infer from config
    pub keyboard: Option<String>,
    pub keymap: Option<String>,

    /// Set of changes to be performed
    #[serde(default = "default_operations")]
    pub operations: Vec<Operation>,

    /// Either you want a default compilation command (qmk compile)
    /// or not (provide your own command/script at a step)
    #[serde(default = "_true")]
    pub default_compilation: bool,
}

fn try_read_from<P: AsRef<Path>>(path: &P) -> Result<BuildFile, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let config = deser_hjson::from_reader(reader)?;

    Ok(config)
}

/// Parse the contents of the config file
pub fn read_from<P: AsRef<Path> + Display>(path: &P) -> BuildFile {
    match try_read_from(path) {
        Ok(config) => {
            info!("Loaded <blue>{path}</>",);
            config
        },
        Err(e) => {
            error!(
                "Parsing config file (<blue>{path}</>)\n\t<red>{}</>",
                e.to_string()
            );
            exit(1);
        }
    }
}
