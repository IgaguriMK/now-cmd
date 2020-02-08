use anyhow::Error;
use clap::{App, Arg};

fn main() {
    if let Err(e) = execute() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn execute() -> Result<(), Error> {
    let matches = App::new("now")
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .default_value("iso8601")
                .help("Output format"),
        )
        .get_matches();

    Ok(())
}
