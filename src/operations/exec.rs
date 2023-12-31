use super::prelude::*;

const fn default_strict() -> bool {
    true
}

fn default_run_at() -> String {
    String::from(".")
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct Exec {
    command: String, // command to execute
    #[serde(default = "default_run_at")]
    at: String, // where to do so (defaults to wherever the CLI is invoked)
    #[serde(default = "default_strict")]
    strict: bool, // is it allowed to fail (defaults to false)
}

impl OperationTrait for Exec {
    fn apply(&self, settings: &build::Settings, repository: &git::Repository) {
        let command = self.command.clone();
        sh::run(command, &self.at, self.strict);
    }

    fn message(&self) -> String {
        let can_fail = if self.strict { " <red>not</>" } else { "" };
        format!(
            "Executing <blue>{}</> at <blue>{}</> -- It can{} fail",
            &self.command, &self.at, can_fail
        )
    }
}
