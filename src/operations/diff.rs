use super::prelude::*;

fn patch_folder() -> String {
    "patches".to_owned()
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct Diff {
    #[serde(default = "patch_folder")]
    folder: String, // where the patch file is stored (defaults to "./patches")
    file: String,  // filename 
}

impl OperationTrait for Diff {
    fn apply(&self,state: &BuildConfig) {
        let _ = sh::run(
            format!(
                "cp {}/{} {}",
                self.folder,
                self.file,
                state.git_repo.path
            ),
            ".",
            true,
        );
        state.git_repo.apply(&self.file);
    }

    fn message(&self) -> String {
        format!("Applying patch: <blue>{}/{}</>", self.folder, self.file)
    }
}