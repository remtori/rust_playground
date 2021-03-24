fn crate_name(inp: &str) -> &str {
    if let Some(idx) = inp.find(':') {
        &inp[..idx]
    } else {
        inp
    }
}

pub fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                crate_name(record.target()),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file(format!(
            "logs/server.{}.log",
            chrono::Local::now().format("%Y-%m-%d-%H-%M")
        ))?)
        .apply()?;

    Ok(())
}
