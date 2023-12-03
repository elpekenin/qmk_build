use super::prelude::*;

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct Wget {
    url: String,  // file to download (raw file, not HTML :P)
    path: String, // where to save the file (relative to build::Settings.path), eg: users/elpekenin/downloads/rtc.c
}

impl OperationTrait for Wget {
    fn apply(&self, settings: &build::Settings, repository: &git::Repository) {
        let full_path = format!("./{}", self.path);
        let parent = std::path::Path::new(&full_path)
            .parent()
            .expect("Did not get a valid path");

        repository.run(
            format!(
                "mkdir -p {} && wget {} -O {}",
                parent.to_string_lossy(),
                self.url,
                self.path
            ),
            true,
        );
    }

    fn message(&self) -> String {
        format!(
            "Downloading <blue>{}</> <green>-></> <blue>{}</>",
            self.url, self.path
        )
    }
}
