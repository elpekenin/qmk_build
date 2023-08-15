use super::prelude::*;

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct Script {
    file: String, // script to execute (eg a python script, thanks to shebang)
    #[serde(default = "default_strict")]
    strict: bool, // is it allowed to fail (defaults to false)
}

impl OperationTrait for Script {
    fn apply(&self, _state: &BuildConfig) {
        sh::run(self.file.clone(), ".", self.strict);
    }

    fn message(&self) -> String {
        format!("Running script <blue>{}</>", &self.file)
    }
}