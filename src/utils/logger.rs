use chrono::Local;
use colored::*;
use fern::{log_file, Dispatch, InitError};
use log::{Level, LevelFilter};

pub fn setup_logger() -> Result<(), InitError> {
    let file = log_file("output.log");
    // Configure fern logger with various logging outputs and formats
    Dispatch::new()
        .level(LevelFilter::Off)
        .level_for("api", LevelFilter::Debug)
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}] - {}",
                Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                record.level().to_string().color(match record.level() {
                    Level::Error => "red",
                    Level::Warn => "yellow",
                    Level::Info => "green",
                    Level::Debug => "blue",
                    Level::Trace => "magenta",
                }),
                // record.target(),
                message
            ))
        })
        .chain(std::io::stdout()) // Log to standard output
        .chain(file?)
        .apply()
        .unwrap();

    Ok(())
}
