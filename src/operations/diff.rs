use super::prelude::*;

fn patch_folder() -> String {
    "patches".to_owned()
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct Diff {
    #[serde(default = "patch_folder")]
    folder: String, // where the patch file is stored (defaults to "./patches")
    file: String, // filename
}

impl OperationTrait for Diff {
    fn apply(&self, settings: &build::Settings, repository: &git::Repository) {
        let _ = sh::run(
            format!("cp {}/{} {}", self.folder, self.file, repository.path),
            ".",
            true,
        );
        repository.apply(&self.file);
    }

    fn message(&self) -> String {
        format!("Applying patch: <blue>{}/{}</>", self.folder, self.file)
    }
}
