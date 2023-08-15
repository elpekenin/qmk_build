use super::prelude::*;

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct Copy {
    orig: String, // origin path
    dest: String, // destination path
}

impl OperationTrait for Copy {
    fn apply(&self, state: &BuildConfig) {
        let orig = &self.orig;
        let dest = &format!("{}/{}", &state.git_repo.path, &self.dest);
        let _ = sh::run(format!("cp -r {orig} {dest}"), ".", true);
    }

    fn message(&self) -> String {
        format!(
            "Copying: <blue>{}</> <green>-></> <blue>{}</>",
            self.orig,
            self.dest,
        )
    }
}
