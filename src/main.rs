mod formatter;

use anyhow::Error;
use chrono::Local;
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
                .default_value("human")
                .help("Output format"),
        )
        .get_matches();

    let fmt_name = matches.value_of("format").unwrap();
    let formatter = formatter::parse(fmt_name)?;

    //////////////

    let now = Local::now().naive_local();

    let s = formatter.format(now);
    println!("{}", s);

    Ok(())
}
