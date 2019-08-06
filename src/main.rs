extern crate clap;
use clap::{Arg, App};
use std::io::{self, BufReader, BufRead};
use std::fs::{self};

struct Reader {
    input: String
}

impl Reader {
    fn get(&self) -> Box<BufRead> {
        let reader: Box<BufRead> = match self.input.as_str() {
        "-" => Box::new(BufReader::new(io::stdin())),
        _ => Box::new(BufReader::new(fs::File::open(self.input.as_str()).unwrap()))
        };
        return reader
    }
}

fn main() {
    let matches = App::new("grep")
                        .about("GNU grep clone")
                        .arg(Arg::with_name("pattern")
                            .takes_value(true)
                            .required(true)
                            .help("Pattern to search for")
                            .index(1))
                        .arg(Arg::with_name("inputFile")
                            .help("The input file to use")
                            .index(2))
                        .get_matches();

    let inputFile = matches.value_of("inputFile").unwrap_or("-");
    let pattern = matches.value_of("pattern").unwrap();
    let reader = Reader { input: inputFile.to_string() };

    // main program loop
    for line in reader.get().lines() {
        match line {
            Ok(l) => {
                if l.contains(pattern){
                    println!("{}", l);
                }
            }
            Err(e) => println!("error parsing line: {:?}", e),
        }
    }
}
