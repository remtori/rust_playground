pub mod bytes;
pub mod prelude;
pub mod string;
pub mod threadguard;
pub mod threadpool;

pub fn init_logger() -> Result<(), fern::InitError> {
    use fern::colors::*;
    use fern::*;
    use std::*;

    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Magenta)
        .trace(Color::White);

    Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{:5}][{}][{}]: {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                colors.color(record.level()),
                record.target(),
                thread::current().name().get_or_insert("unnamed"),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;

    Ok(())
}
