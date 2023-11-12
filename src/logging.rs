use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, ConfigBuilder, TermLogger, TerminalMode};

mod macros {
    #[macro_export]
    macro_rules! _format {
        ($($args:tt)+) => {
            if std::env::var_os("NO_COLOR").is_some() {
                format!($($args)*)
                    .replace("<blue>", "")
                    .replace("<green>", "")
                    .replace("<red>", "")
                    .replace("<yellow>", "")
                    .replace("</>", "")
            } else {
                format!($($args)*)
            }
        };
    }

    #[macro_export]
    macro_rules! error {
        ($($args:tt)+) => {
            simplelog::error!("{}", &$crate::logging::_format!($($args)*));
        };
    }

    #[macro_export]
    macro_rules! _warn {
        ($($args:tt)+) => {
            simplelog::warn!("{}", &$crate::logging::_format!($($args)*));
        };
    }

    #[macro_export]
    macro_rules! info {
        ($($args:tt)+) => {
            simplelog::info!("{}", &$crate::logging::_format!($($args)*));
        };
    }

    #[macro_export]
    macro_rules! debug {
        ($($args:tt)+) => {
            simplelog::debug!("{}", &$crate::logging::_format!($($args)*));
        };
    }

    #[macro_export]
    macro_rules! trace {
        ($($args:tt)+) => {
            simplelog::trace!("{}", &$crate::logging::_format!($($args)*));
        };
    }

    pub use _format;
    pub use _warn;
    pub use debug;
    pub use error;
    pub use info;
    pub use trace;
}

pub use macros::{_format, _warn as warn, debug, error, info, trace};

pub fn init() {
    let logging_config = ConfigBuilder::new()
        .set_time_level(LevelFilter::Off)
        .build();

    let level = match std::env::var("RUST_LOG").unwrap_or_default().as_str() {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
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
