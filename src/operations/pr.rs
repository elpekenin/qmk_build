use super::prelude::*;

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct PullRequest {
    repo: Option<String>, // repo where PR was raised (defaults to config file's repo field)
    id: u16,              // PR id. FIXME: May not work on anything other than GitHub
}

impl OperationTrait for PullRequest {
    fn apply(&self, settings: &build::Settings, repository: &git::Repository) {
        let local_branch_name = format!("PR_{}", self.id);
        let fetch_remote_branch = &format!("pull/{}/head:{}", self.id, local_branch_name);

        repository.fetch(
            self.repo.as_ref().unwrap_or(&settings.repo),
            Some(fetch_remote_branch),
        );
        repository.merge(None, &[local_branch_name], None);
    }

    fn message(&self) -> String {
        format!("Merging changes from <blue>#{}</>", self.id)
    }
}
