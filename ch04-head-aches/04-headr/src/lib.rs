use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};

use clap::{value_parser, App, Arg};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

type HeadResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> HeadResult<()> {
    for (file_num, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {
                if config.files.len() > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    );
                }

                if let Some(num_bytes) = config.bytes {
                    // let mut window = file.take(num_bytes as u64);
                    // let mut buffer = vec![0; num_bytes];
                    // let bytes_read = window.read(&mut buffer)?;
                    // print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                    let bytes = file
                        .bytes()
                        .take(num_bytes)
                        .collect::<Result<Vec<_>, _>>()?;
                    print!("{}", String::from_utf8_lossy(&bytes[..]));
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
            }
        }
    }
    Ok(())
}

const FILES_ID: &str = "files";
const FILES_DEFAULT: &str = "-";
const LINES_ID: &str = "lines";
const BYTES_ID: &str = "bytes";

pub fn get_args() -> HeadResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0a0")
        .author("francoposa <franco@francoposa.io>")
        .about("rust head")
        .arg(
            Arg::with_name(FILES_ID)
                .value_name("FILES")
                .help("Input file(s)")
                .multiple_values(true)
                .default_value(FILES_DEFAULT),
        )
        .arg(
            Arg::with_name(LINES_ID)
                .short('n')
                .long(LINES_ID)
                .value_name("LINES")
                .help("Number of lines")
                .default_value("10"),
        )
        .arg(
            Arg::with_name(BYTES_ID)
                .short('c')
                .long(BYTES_ID)
                .value_name("BYTES")
                .takes_value(true)
                .conflicts_with(LINES_ID)
                .help("Number of bytes"),
        )
        .get_matches();

    let files = matches.get_many(FILES_ID).unwrap().cloned().collect();
    let lines = matches
        .get_one(LINES_ID)
        .cloned()
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("invalid line count -- {}", e))?
        .unwrap();
    let bytes = matches
        .get_one(BYTES_ID)
        .cloned()
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("invalid byte count -- {}", e))?;

    Ok(Config {
        files,
        lines,
        bytes,
    })
}

fn open(filename: &str) -> HeadResult<Box<dyn BufRead>> {
    match filename {
        FILES_DEFAULT => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn parse_positive_int(s: String) -> HeadResult<usize> {
    match s.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(s)),
    }
}

#[test]
fn test_parse_positive_int() {
    let result = parse_positive_int(String::from("3"));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 3);

    let result = parse_positive_int(String::from("foo"));
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "foo".to_string());

    let result = parse_positive_int(String::from("0"));
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "0".to_string());

    let result = parse_positive_int(String::from("-1"));
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "-1".to_string());
}
