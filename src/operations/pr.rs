use super::prelude::*;

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct PullRequest {
    repo: Option<String>, // repo where PR was raised (defaults to config file's repo field)
    id: u16               // PR id. FIXME: May not work on anything other than GitHub
}

impl OperationTrait for PullRequest {
    fn apply(&self,state: &BuildConfig) {
        let local_branch_name = format!("PR_{}", self.id);
        let fetch_remote_branch = &format!("pull/{}/head:{}", self.id, local_branch_name);

        state.git_repo.fetch(
            self.repo.as_ref().unwrap_or(&state.build_file.repo),
            Some(fetch_remote_branch),
        );
        state.git_repo.merge(None, &[local_branch_name], None);
    }

    fn message(&self) -> String {
        format!("Merging changes from <blue>#{}</>", self.id)
    }
}