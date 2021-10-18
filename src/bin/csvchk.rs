use clap::{App, Arg};
use csvchk::{Config, MyResult};

// --------------------------------------------------
fn main() {
    if let Err(err) = get_args().and_then(run) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("csvchk")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust cal")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input filename")
                .min_values(1)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("delimiter")
                .value_name("DELIM")
                .short("d")
                .long("delim")
                .help("Field delimiter")
                .default_value(","),
        )
        .arg(
            Arg::with_name("no_headers")
                .value_name("NO_HEADERS")
                .short("n")
                .long("no-headers")
                .help("Input file has no headers")
                .takes_value(false),
        )
        .get_matches();

    let delimiter = matches.value_of("delimiter").unwrap().as_bytes();
    if delimiter.len() != 1 {
        return Err("--delimiter must be a single byte/character".into());
    }

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        delimiter: delimiter[0],
        has_headers: !matches.is_present("no_headers"),
    })
}

// --------------------------------------------------
fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    for filename in config.files {
        csvchk::process(&filename, &config.delimiter, config.has_headers)?;
    }
    Ok(())
}
