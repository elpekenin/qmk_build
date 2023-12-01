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

const fn default_operations() -> Vec<Operation> {
    Vec::new()
}

// Struct to define the contents expected on JSON file
#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct Settings {
    // Required, initial state of the repo
    pub repo: String,
    pub branch: String,

    // Optional, where the copy of the fork will be stored, defaults to $HOME/.__qmk_build__
    #[serde(default = "default_path")]
    pub path: String,

    // Can be used to override the command used to compile, defaults to `qmk config`
    pub compile_command: Option<String>,
    // Let user set a fixed value, otherwise let `qmk compile` infer from config
    pub keyboard: Option<String>,
    pub keymap: Option<String>,

    // Set of changes to be performed
    #[serde(default = "default_operations")]
    pub operations: Vec<Operation>,

    // Do stuff after compiling
    #[serde(default = "default_operations")]
    pub post_compilation: Vec<Operation>,
}

impl Settings {
    /// # Errors
    ///
    /// Will return `Err` if `path` does not exist or the content does not follow
    /// the expected format (check schema file)
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

    use crate::{build, logging};

    #[test]
    fn schema() {
        let schema = schema_for!(build::Settings);

        #[allow(clippy::unwrap_used)]
        let schema_str = serde_json::to_string_pretty(&schema).unwrap();

        #[allow(clippy::unwrap_used)]
        let mut file = File::create("schema").unwrap();
        let _ = file.write_all(schema_str.as_bytes());

        logging::info!("Schema generated");
    }
}
