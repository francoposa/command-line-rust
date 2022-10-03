use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type CatResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> CatResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut nonblank_line_num = 0;
                for (line_num, line_result) in file.lines().enumerate() {
                    let line = line_result?;
                    if config.number_lines {
                        println!("{:>6}\t{}", line_num + 1, line);
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            nonblank_line_num += 1;
                            println!("{:>6}\t{}", nonblank_line_num, line);
                        } else {
                            println!("{}", line);
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

const FILES_ID: &str = "files";
const NUMBER_ID: &str = "number";
const NUMBER_NONBLANK_ID: &str = "number_nonblank";

pub fn get_args() -> CatResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0a0")
        .author("francoposa <franco@francoposa.io>")
        .about("rust cat")
        .arg(
            Arg::with_name(FILES_ID)
                .value_name("FILES")
                .help("Input file(s)")
                .multiple_values(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name(NUMBER_ID)
                .short('n')
                .long(NUMBER_ID)
                .help("Number lines")
                .takes_value(false)
                .conflicts_with(NUMBER_NONBLANK_ID),
        )
        .arg(
            Arg::with_name(NUMBER_NONBLANK_ID)
                .short('b')
                .long("number-nonblank")
                .help("Number non-blank lines")
                .takes_value(false),
        )
        .get_matches();

    let config = Config {
        files: matches.get_many(FILES_ID).unwrap().cloned().collect(),
        number_lines: matches.is_present(NUMBER_ID),
        number_nonblank_lines: matches.is_present(NUMBER_NONBLANK_ID),
    };
    Ok(config)
}

fn open(filename: &str) -> CatResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
