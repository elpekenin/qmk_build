use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, ConfigBuilder, TermLogger, TerminalMode};

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

    // init will only fail if we call it a second time (at which point logger is setup and we dont mind it failing)
    let _ = CombinedLogger::init(vec![TermLogger::new(
        level,
        logging_config,
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )]);
}
