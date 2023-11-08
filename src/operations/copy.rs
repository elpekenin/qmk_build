use super::prelude::*;

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct Copy {
    orig: String, // origin path
    dest: String, // destination path
}

impl OperationTrait for Copy {
    fn apply(&self, settings: &build::Settings, repository: &git::Repository) {
        let orig = &self.orig;
        let dest = &format!("{}/{}", &repository.path, &self.dest);
        let _ = sh::run(format!("mkdir -p {dest}"), ".", true);
        let _ = sh::run(format!("cp -r {orig} {dest}"), ".", true);
    }

    fn message(&self) -> String {
        format!(
            "Copying: <blue>{}</> <green>-></> <blue>{}</>",
            self.orig, self.dest,
        )
    }
}
