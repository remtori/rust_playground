pub mod bytes;
pub mod flystring;
pub mod objectpool;
pub mod prelude;
pub mod string;
pub mod threadguard;
pub mod threadpool;

pub fn init_logger() -> Result<(), fern::InitError> {
    use fern::*;
    use std::ffi::*;
    use std::*;

    let log_file_path = {
        let bin_path = env::current_exe()?;
        let mut bin_name = bin_path.file_stem();
        let log_file_name = bin_name.get_or_insert(OsStr::new("unknown"));
        let mut log_file_path = OsString::from("logs/");
        log_file_path.push(log_file_name);
        log_file_path.push(".log");
        log_file_path
    };

    Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{:5}][{}][{}]: {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.level(),
                record.target(),
                thread::current().name().get_or_insert("unnamed"),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(log_file(log_file_path)?)
        .apply()?;

    Ok(())
}

pub fn ascii_parse<T: std::str::FromStr>(bytes: &[u8]) -> Option<T> {
    if let Ok(Ok(value)) = std::str::from_utf8(bytes).map(|s| s.parse::<T>()) {
        Some(value)
    } else {
        None
    }
}
