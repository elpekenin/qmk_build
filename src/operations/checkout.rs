use super::prelude::*;

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct Checkout {
    repo: String,       // repo to check from
    branch: String,     // branch of such repo
    files: Vec<String>, // files/folders being pulled
}

impl OperationTrait for Checkout {
    fn apply(&self,state: &BuildConfig) {
        state.git_repo.remote_add(&self.repo);
        state.git_repo.fetch(&self.repo, None);
        state.git_repo.checkout(
            &self.repo,
            &self.branch,
            Some(&self.files)
        );
    }

    fn message(&self) -> String {
        format!(
            "Checking out <blue>{:?}</> from <blue>{}</> <green>@</> <blue>{}</>",
            self.files,
            self.repo,
            self.branch
        )
    }
}