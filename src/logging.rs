use log::LevelFilter;
use simplelog::{ConfigBuilder, CombinedLogger, TermLogger, TerminalMode, ColorChoice};

extern crate simplelog;
pub use simplelog::{debug, error, info, warn};

pub fn init() {
    let logging_config = ConfigBuilder::new()
        .set_time_level(LevelFilter::Off)
        .build();

    let level = match std::env::var("RUST_LOG").unwrap_or_default().as_str() {
        "debug" => LevelFilter::Debug,
        "error" => LevelFilter::Error,
        "trace" => LevelFilter::Trace,
        "warn" => LevelFilter::Warn,
        "off" => LevelFilter::Off,
        _ => LevelFilter::Info,
    };

    #[allow(clippy::unwrap_used)]
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
