pub extern crate log;
pub extern crate simplelog;
use std::env;

pub use simplelog::*;

pub fn init() {
    let logging_config = ConfigBuilder::new()
        .set_time_level(LevelFilter::Off)
        .build();

    let level = match env::var("RUST_LOG").unwrap_or_default().as_str() {
        "debug" => LevelFilter::Debug,
        "error" => LevelFilter::Error,
        "trace" => LevelFilter::Trace,
        "warn" => LevelFilter::Warn,
        "off" => LevelFilter::Off,
        _ => LevelFilter::Info,
    };

    CombinedLogger::init(
        vec![
            TermLogger::new(
                level,
                logging_config,
                TerminalMode::Mixed,
                ColorChoice::Auto
            ),
        ]
    ).unwrap();
}
