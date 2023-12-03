use super::prelude::*;

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct Copy {
    orig: String,        // origin path
    destination: String, // destination path
}

impl OperationTrait for Copy {
    fn apply(&self, settings: &build::Settings, repository: &git::Repository) {
        let orig = &self.orig;
        let destination = &format!("{}/{}", &repository.path, &self.destination);
        let _ = sh::run(format!("mkdir -p {destination}"), ".", true);
        let _ = sh::run(format!("cp -r {orig} {destination}"), ".", true);
    }

    fn message(&self) -> String {
        format!(
            "Copying: <blue>{}</> <green>-></> <blue>{}</>",
            self.orig, self.destination,
        )
    }
}
