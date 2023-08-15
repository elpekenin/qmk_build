pub use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "elpekenin")]
#[command(long_about = r#"
Little CLI tool written in Rust.
It allows customizing the original repo before compiling.
These changes are defined in a json file, with actions like copying local files, or grabbing from another repo/branch."#)]
#[command(about = "CLI to ease custom QMK compilations")]
#[command(version)]
pub struct Args {
    #[arg(default_value_t = String::from("build.json"))]
    pub file: String,          // File where custom building steps to be applied are listed
    #[arg(short, long)]
    pub generate_schema: bool, // To generate the jsonschema
}
