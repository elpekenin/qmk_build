use crate::git::Strategy;

use super::prelude::*;

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct Merge {
    repo: String,               // repo to check from
    branch: String,             // branch of such repo
    strategy: Option<Strategy>, // how to solve conflicts
}

impl OperationTrait for Merge {
    fn apply(&self, settings: &build::Settings, repository: &git::Repository) {
        repository.fetch(&self.repo, Some(&self.branch));
        repository.merge(
            Some(&self.repo),
            &[self.branch.clone()],
            self.strategy.clone(),
        );
    }

    fn message(&self) -> String {
        format!(
            "Merging <blue>{}</> <green>@</> <blue>{}</> using {}",
            self.repo,
            self.branch,
            self.strategy.clone().unwrap_or_default().to_string()
        )
    }
}
