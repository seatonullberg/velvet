use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use clap::{arg_enum, value_t, App, Arg, ArgMatches, SubCommand};
use ron::ser::{to_string_pretty, PrettyConfig};

use velvet_convert::load_poscar;

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum FileFormat {
        Poscar,
    }
}

fn main() {
    let matches = App::new("Velvet CLI")
        .version("0.1.0")
        .author("Seaton Ullberg <seatonullberg@gmail.com>")
        .about("Command line tool built on top of the Velvet API")
        .subcommand(
            SubCommand::with_name("convert")
                .about("convert external data formats into Velvet's internal format")
                .arg(
                    Arg::with_name("src")
                        .index(1)
                        .takes_value(true)
                        .required(true)
                        .help("source filepath"),
                )
                .arg(
                    Arg::with_name("dst")
                        .index(2)
                        .takes_value(true)
                        .required(true)
                        .help("destination filepath"),
                )
                .arg(
                    Arg::with_name("format")
                        .short("f")
                        .long("format")
                        .takes_value(true)
                        .possible_values(&FileFormat::variants())
                        .case_insensitive(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("convert") {
        handle_convert(matches)
    }
}

fn handle_convert(matches: &ArgMatches) {
    let file = File::open(matches.value_of("src").unwrap()).unwrap();
    let buf = BufReader::new(file);
    let fmt = value_t!(matches, "format", FileFormat).unwrap();
    let system = match fmt {
        FileFormat::Poscar => load_poscar(buf),
    };
    let pretty = PrettyConfig::new().with_decimal_floats(true);
    let res = to_string_pretty(&system, pretty).unwrap();
    let mut file = File::create(matches.value_of("dst").unwrap()).unwrap();
    file.write_all(res.as_bytes()).unwrap();
}
