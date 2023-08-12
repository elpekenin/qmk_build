pub use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// File where custom building steps to be applied are listed
    #[arg(default_value_t = String::from("build.json"))]
    pub file: String,
}
