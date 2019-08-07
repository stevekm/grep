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

fn pattern_match(pattern: &str, contents: &str) -> bool {
    if contents.contains(pattern){
        true
    } else {
        false
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
                if pattern_match(&pattern, &l) {
                    println!("{}", l);
                }
            }
            Err(e) => println!("error parsing line: {:?}", e),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_pattern_match1(){
        let pattern = "foo";
        let contents = "foobarbaz";
        let output = pattern_match(&pattern, &contents);
        let expected_output = true;
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_pattern_match2(){
        let pattern = "foo";
        let contents = "barbaz";
        let output = pattern_match(&pattern, &contents);
        let expected_output = false;
        assert_eq!(output, expected_output);
    }
}
