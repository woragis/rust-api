use chrono::Local;
use colored::*;
use fern::{log_file, Dispatch};

pub fn setup_logger() -> Result<(), fern::InitError> {
    let file = log_file("output.log");
    // Configure fern logger with various logging outputs and formats
    Dispatch::new()
        .level(log::LevelFilter::Off)
        .level_for("api", log::LevelFilter::Debug)
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}] - {}",
                Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                record.level().to_string().color(match record.level() {
                    log::Level::Error => "red",
                    log::Level::Warn => "yellow",
                    log::Level::Info => "green",
                    log::Level::Debug => "blue",
                    log::Level::Trace => "magenta",
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
