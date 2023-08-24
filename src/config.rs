use std::{
    error::Error,
    fmt::{Debug, Display},
    fs::File,
    io::BufReader,
    path::Path,
};

use schemars::JsonSchema;
use serde::Deserialize;

use crate::operations::Operation;

fn default_path() -> String {
    String::from("$HOME/.__qmk_build__")
}

fn default_operations() -> Vec<Operation> {
    Vec::new()
}

fn _true() -> bool {
    true
}

// Struct to define the contents expected on JSON file
#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct BuildFile {
    // Required, initial state of the repo
    pub repo: String,
    pub branch: String,

    // Optional, where the copy of the fork will be stored, defaults to $HOME/.__qmk_build__
    #[serde(default = "default_path")]
    pub path: String,

    // Let user set a fixed value, otherwise let `qmk` infer from config
    pub keyboard: Option<String>,
    pub keymap: Option<String>,

    // Set of changes to be performed
    #[serde(default = "default_operations")]
    pub operations: Vec<Operation>,

    // Either you want a default compilation command (qmk compile)
    // or not (provide your own command/script at a step)
    #[serde(default = "_true")]
    pub default_compilation: bool,
}

impl BuildFile {
    pub fn load<P: AsRef<Path> + Display>(path: &P) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
    
        let config = deser_hjson::from_reader(reader)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write};

    use schemars::schema_for;

    use crate::{logging::{info, log, paris}, config::BuildFile};

    #[test]
    fn schema() {
        let schema = schema_for!(BuildFile);
        let schema_str = serde_json::to_string_pretty(&schema).unwrap();

        let mut file = File::create("schema").unwrap();
        let _ = file.write_all(schema_str.as_bytes());

        info!("Schema generated");
    }
}