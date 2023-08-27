pub mod prelude {
    #[allow(unused_imports)]
    pub use crate::{
        logging::{info, log, paris},
        sh, BuildConfig,
    };

    pub use enum_dispatch::enum_dispatch;
    pub use schemars::JsonSchema;
    pub use serde::Deserialize;

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

use checkout::Checkout;
use copy::Copy;
use diff::Diff;
use exec::Exec;
use merge::Merge;
use pr::PullRequest;


#[enum_dispatch(OperationTrait)]
#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(tag = "operation", rename_all = "snake_case")]
// Different patches to be applied to initial state of the repo
pub enum Operation {
    Checkout(Checkout), // Grab files/folders from another branch (and repo?)
    Cp(Copy),           // Copy files/folders
    Diff(Diff),         // Apply diff file
    Exec(Exec),         // Run a command/script
    Merge(Merge),       // Merge a branch
    Pr(PullRequest),    // Grab PR changes
}
