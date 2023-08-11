pub extern crate log;
pub extern crate simplelog;
pub use simplelog::*;

pub fn init() {
    let logging_config = ConfigBuilder::new()
        .set_time_level(LevelFilter::Off)
        .build();

    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, logging_config, TerminalMode::Mixed, ColorChoice::Auto),
        ]
    ).unwrap();
}
