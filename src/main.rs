extern crate clap;
use failure::{Error, ResultExt, format_err, bail};
use std::fs;

type Result<T> = std::result::Result<T, Error>;
type EResult = std::result::Result<(), Error>;

fn main() -> EResult {
    let matches = clap::App::new("placeholder_happy_app")
    .version("1.0")
    .about("Placeholder happy app")
    .arg(clap::Arg::with_name("path")
        .value_name("PATH")
        .help("Path to a file to try to open")
        .required(true))
    .arg(clap::Arg::with_name("gui")
        .short("g")
        .long("gui")
        .value_name("GUI")
        .help("Show the gui")
        .takes_value(false)
        .required(false))
    .get_matches();

    macro_rules! arg_value {
        ( $name:expr ) => {
            matches.value_of($name).ok_or_else(|| format_err!("missing {}", $name.to_uppercase()))?
        };
    }

    let path = arg_value!("path");
    let gui = matches.is_present("gui");
    if gui {
        bail!("sorry actually there's no gui");
    }
    fs::File::open(path).with_context(|_| "while opening file")?;
    println!("Ok, it works! Time to write your program.");
    Ok(())
}