use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};


// #[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("rcat")
        .version("0.1.0")
        .author("Luke Milby <luke.milby@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("Number lines")
                .takes_value(false)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank")
                .help("Number non-blank lines")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    let mut contents = String::new();
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let buf_reader = BufReader::new(file);

                let mut count = 1;
                for l in buf_reader.lines() {
                    let line = l.unwrap();
                    if config.number_lines {
                        contents.push_str(format!("\t{} {}\n", count, line).as_str());
                    } else if config.number_nonblank_lines {
                        if  line.is_empty() {
                            count -= 1;
                            contents.push_str(format!("{}\n", &line).as_str());
                        } else {
                            contents.push_str(format!("\t{} {}\n", count, line).as_str());
                        }
                    } else {
                        contents.push_str(format!("{}\n", line).as_str())
                    };
                    count += 1
                };
                // buf_reader.read_to_string(&mut contents)?;
            }
        }
    }
    print!("{}", contents);
    Ok(())
}