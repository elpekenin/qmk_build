use serde::Deserialize;

use crate::{State, git, logging::*, sh};

#[derive(Clone, Debug, Deserialize)]
pub struct CpOperation {
    orig: String,
    dest: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DiffOperation {
    patch: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CheckoutOperation {
    repo: String,
    branch: String,
    files: Vec<String>,
}


#[allow(non_camel_case_types)]

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "operation")]
/// Different patches to be applied to initial state of the repo
pub enum Operation {
    /// Can be used on files or folders, copy whatever contents
    cp(CpOperation),

    /// Apply diff on a file
    diff(DiffOperation),

    /// Grab files/folders from another branch (and repo?)
    checkout(CheckoutOperation),
}
#[warn(non_camel_case_types)]


impl Operation {
    pub fn apply(&self, state: &State) {
        match self {
            Operation::cp(ref operation) => {
                let orig = &operation.orig;
                let dest = &format!("{}/{}",&state.path, &operation.dest);

                info!("Applying cd: <blue>{}</> <yellow>-></> <blue>{}</>", orig, dest);

                let _ = sh::run_strict(format!("cp -r {orig} {dest}"));
            },
            Operation::diff(ref operation) => {
                info!("Applying patch: <blue>{}</>", operation.patch);

                let _ = sh::run_strict(format!("cp {} {}", operation.patch, state.cwd));
                git::apply(&operation.patch, state.path);
            },
            Operation::checkout(ref operation) => {
                info!("Checking out <blue>{:?}</> from <blue>{}</> <yellow>@</> <blue>{}</>", operation.files, operation.repo, operation.branch);

                git::remote_add(&operation.repo, state.path);
                git::fetch(&operation.repo, state.path);
                git::checkout(&operation.repo, &operation.branch, Some(&operation.files), state.path);
            },
        }
    }
}
