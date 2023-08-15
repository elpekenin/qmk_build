pub mod prelude {
    #[allow(unused_imports)]
    pub(crate) use crate::{
        logging::{info, log, paris},
        sh, BuildConfig,
    };

    pub use enum_dispatch::enum_dispatch;
    pub use schemars::JsonSchema;
    pub use serde::Deserialize;

    pub fn default_strict() -> bool {
        true
    }

    pub fn default_run_at() -> String {
        ".".to_owned()
    }

    // common behaviour on all operations
    #[enum_dispatch]
    pub trait OperationTrait {
        fn apply(&self, state: &BuildConfig);
        fn message(&self) -> String;
    }
}

use prelude::*;

mod checkout;
mod copy;
mod diff;
mod exec;
mod merge;
mod pr;
mod script;

use checkout::Checkout;
use copy::Copy;
use diff::Diff;
use exec::Exec;
use merge::Merge;
use pr::PullRequest;
use script::Script;


#[enum_dispatch(OperationTrait)]
#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(tag = "operation", rename_all = "snake_case")]
// Different patches to be applied to initial state of the repo
pub enum Operation {
    Checkout(Checkout), // Grab files/folders from another branch (and repo?)
    Cp(Copy),           // Copy files/folders
    Diff(Diff),         // Apply diff file
    Exec(Exec),         // Run a command
    Merge(Merge),       // Merge a branch
    Pr(PullRequest),    // Grab PR changes
    Script(Script),     // Run a file
}
