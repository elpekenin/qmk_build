use super::prelude::*;

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct Wget {
    url: String,      // file to download (raw file, not HTML :P)
    filename: String, // to save the file (located at build::Settings.path / DOWNLOADS)
}

const DOWNLOADS: &str = "downloads";

impl OperationTrait for Wget {
    fn apply(&self, settings: &build::Settings, repository: &git::Repository) {
        repository.run(format!("mkdir -p {DOWNLOADS}"), true);
        repository.run(
            format!("wget {} -O {DOWNLOADS}/{}", self.url, self.filename),
            true,
        );
    }

    fn message(&self) -> String {
        format!(
            "Downloading: <blue>{}</> <green>-></> <blue>{DOWNLOADS}/{}</>",
            self.url, self.filename
        )
    }
}
