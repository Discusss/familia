use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;
use crate::color::*;

/**
 * Set up the logger with the default configuration.
 *
 * # Example
 *
 * ```
 * use core::logger::setup_logger;
 *
 * setup_logger().unwrap();
 * ```
 */
pub fn setup_logger() -> Result<(), fern::InitError> {

    std::fs::create_dir_all("logs")?;

    let mut colors = ColoredLevelConfig::new();
    colors.warn = Color::Yellow;
    colors.info = Color::Green;
    colors.error = Color::Red;
    colors.debug = Color::BrightBlack;
    colors.trace = Color::Magenta;

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}: {} {} - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                format_target(record.target(), 26),
                colors.color(record.level()),
                message
            ))
        })
        .level(LevelFilter::Info)
        //.level_for("rocket", LevelFilter::Warn)
        .level_for("tracing", LevelFilter::Warn)
        .chain(std::io::stdout())
        .chain(fern::log_file("logs/latest.log")?)
        .chain(fern::log_file(format!("logs/{}.log", chrono::Local::now().format("%Y-%m-%d_%H-%M-%S")).as_str())?)
        .apply()?;
    Ok(())
}

/**
 * Format the target of a log message.
 *
 * # Arguments
 *
 * * `target` - The target of the log message.
 * * `ind` - The number of characters to pad the target with.
 *
 * # Example
 *
 * ```
 * use core::logger::format_target;
 *
 * let formatted = format_target("core::logger", 20);
 * assert_eq!(formatted, "core::logger        ");
 * ```
 */
pub fn format_target(target: &str, ind: usize) -> String {
    let mut formatted = target.to_string();

    if formatted.len() > ind {
        formatted.truncate(ind - 3);
        formatted.push_str("...");
        formatted
    } else {
        formatted.push_str(&" ".repeat(ind - formatted.len()));
        formatted
    }
}

/**
 * Log a banner to the console.
 *
 * # Example
 *
 * ```
 * use core::logger::log_banner;
 *
 * log_banner();
 * ```
 */
pub fn log_banner() {
    println!(
        r#"
{color_red}      __            ______      __
{color_red}     / /   ____ _  / ____/___ _/ /_  _________ _
{color_red}    / /   / __ `/ / /   / __ `/ __ \/ ___/ __ `/   {color_yellow}Image API for
{color_red}   / /___/ /_/ / / /___/ /_/ / /_/ / /  / /_/ /    {color_yellow}La Cabra
{color_red}  /_____/\__,_/  \____/\__,_/_.___/_/   \__,_/
        "#);

    println!("{color_magenta}  Version:     {color_cyan} {}", env!("CARGO_PKG_VERSION"));
    println!("{color_magenta}  Author(s):   {color_cyan} {}", env!("CARGO_PKG_AUTHORS"));
    println!("{color_magenta}  License:     {color_cyan} {}", env!("CARGO_PKG_LICENSE"));
    println!("{color_magenta}  Repository:  {color_cyan} {}", env!("CARGO_PKG_REPOSITORY"));
    println!("{bg_reset}{color_reset}");
}