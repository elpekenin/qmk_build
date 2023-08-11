use std::{path::Path, fs::File, io::BufReader, error::Error, process::exit};

use serde::Deserialize;

use crate::logging::*;

/// Different patches to be applied to initial state of the repo
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "operation")]
pub enum Operations {
    /// Can be used on files or folders, copy whatever contents
    CmdCp {orig: String, dest: String},

    /// Apply diff on a file
    CmdDiff {orig: String, dest: String},
    
    /// Checkout a file
    GitCheckout {repo: String, branch: String, path: String},
}

fn default_workdir() -> String {
    String::from("$HOME/.__qmk_build__")
}

fn default_operations() -> Vec<Operations> {
    Vec::new()
}

/// Struct to define the contents expected on JSON file
#[derive(Clone, Debug, Deserialize)]
pub struct JsonConfig {
    /// Required, initial state of the repo
    pub repo: String,
    pub branch: String,

    /// Optional, where the copy of the fork will be stored, defaults to $HOME/.__qmk_build__
    #[serde(default = "default_workdir")]
    pub workdir: String,

    /// Let user set a fixed value, otherwise let `qmk` infer from config
    pub keyboard: Option<String>,
    pub keymap: Option<String>,

    /// Set of changes to be performed
    #[serde(default = "default_operations")]
    pub operations: Vec<Operations>
}

fn try_read_from<P: AsRef<Path>>(path: P) -> Result<JsonConfig, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let config = serde_json::from_reader(reader)?;

    Ok(config)
}

/// Parse the contents of the config file
pub fn read_from<P: AsRef<Path>>(path: P) -> JsonConfig {
    match try_read_from(path) {
        Ok(config) => config,
        Err(e) => {
            error!("Parsing config file\n\t<red>{}</>", e.to_string());
            exit(1);
        }
    }
}