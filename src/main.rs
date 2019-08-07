extern crate clap;
extern crate itertools;
use clap::{Arg, App};
use std::io::{self, BufReader, BufRead};
use std::fs::{self};
use itertools::Itertools; // 0.8.0

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

#[derive(Debug)]
struct LineBuffer {
    before: Vec<String>,
    before_size: usize,
    after: Vec<String>,
    after_size: usize,
}

impl LineBuffer {
    fn add_before(&mut self, line: &str){
        self.before.insert(0,line.to_string());
        if self.before.len() > self.before_size {
            self.before.truncate(self.before_size);
        }
    }
    fn add_after(&mut self, line: &str){
        self.after.insert(0,line.to_string());
        if self.after.len() > self.after_size {
            self.after.truncate(self.after_size);
        }
    }
}

fn pattern_match(pattern: &str, contents: &str) -> bool {
    if contents.contains(pattern){
        true
    } else {
        false
    }
}

fn match_lines_with_buffer(reader: &Reader, pattern: &str){
    let before_size = 2;
    let after_size = 2;
    let mut before = Vec::with_capacity(before_size);
    let mut after = Vec::with_capacity(after_size);
    let mut buffer = LineBuffer{
        before: before,
        before_size: before_size,
        after_size: after_size,
        after: after};

    // main program loop
    for line in reader.get().lines() {
        match line {
            Ok(l) => {
                // buffer.before.push(l.clone());
                buffer.add_before(&l);
                if pattern_match(&pattern, &l) {
                    println!("{}", l);
                }
                println!("{:?}",buffer);
            }
            Err(e) => println!("error parsing line: {:?}", e),
        }
    }
}

fn match_lines(reader: &Reader, pattern: &str){
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

    match_lines(&reader, &pattern);

    // let stdin = std::io::stdin();
    // for lines in &stdin.lock().lines().chunks(3) {
    //     for (i, line) in lines.enumerate() {
    //         println!("Line {}: {:?}", i, line);
    //         println!("......")
    //     }
    // }
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
