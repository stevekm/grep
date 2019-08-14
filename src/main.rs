extern crate clap;
use clap::{Arg, App};
use std::io::{self, BufReader, BufRead, BufWriter};
use std::fs::{self, File};
use std::path::Path;
use std::io::Write;
use itertools::multipeek;

struct Config {
    before: usize,
    after: usize,
    pattern: String,
}

struct Reader {
    input: String
}

impl Reader {
    fn get(&self) -> Box<BufRead> {
        match self.input.as_str() {
        "-" => Box::new(BufReader::new(io::stdin())),
        "STDIN" => Box::new(BufReader::new(io::stdin())),
        _ => Box::new(BufReader::new(fs::File::open(self.input.as_str()).unwrap()))
        }
    }
}

struct Writer {
    output: String
}

impl Writer {
    fn get(&self) -> BufWriter<Box<dyn std::io::Write>> {
        match self.output.as_str() {
            "STDOUT" => BufWriter::new(Box::new(io::stdout())),
            _ => BufWriter::new(Box::new(File::create(Path::new(&self.output)).unwrap()))
        }
    }
}

#[derive(Debug)]
struct LineBuffer {
    contents: Vec<String>,
    size: usize,
}

impl LineBuffer {
    fn add(&mut self, line: &str){
        self.contents.insert(0,line.to_string());
        if self.contents.len() > self.size {
            self.contents.truncate(self.size);
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

fn match_lines_with_buffer(reader: &Reader, writer: &Writer, pattern: &str){
    let mut write_handle = writer.get();
    let before_size = 2;
    let after_size = 2;
    let mut before = Vec::with_capacity(before_size);
    let mut buffer = LineBuffer{ contents: before, size: before_size };
    let mut mp = multipeek(reader.get().lines());

    // main program loop
    loop {
        let line = mp.next();
        match line {
            Some(l) => {
                match l {
                    Ok(l_val) => {
                        if pattern_match(&pattern, &l_val) {
                            // first write out the previous lines buffer
                            for item in buffer.contents.iter().rev(){
                                writeln!(write_handle, "{}", item);
                            }
                            // write the current line
                            writeln!(write_handle, "{}", l_val);

                            // write out the following lines
                            if after_size > 0 {
                                for _ in 0..after_size {
                                    let peek = mp.peek();
                                    match peek {
                                        Some(peek_res) => {
                                            match peek_res {
                                                Ok(peek_val) => writeln!(write_handle, "{}", peek_val),
                                                Err(e) => writeln!(write_handle, "error peek value line: {:?}", e),
                                            };
                                        }
                                        None => ()
                                    }
                                }
                                writeln!(write_handle, "{}", "--");
                            }
                        }
                        // add the current line to the buffer
                        buffer.add(&l_val);
                    },
                    Err(e) => println!("error parsing line: {:?}", e),
                }
            }
            None => break,
        }
    }
}

fn match_lines(reader: &Reader, writer: &Writer, pattern: &str){
    let mut write_handle = writer.get();
    // main program loop
    for line in reader.get().lines() {
        match line {
            Ok(l) => {
                if pattern_match(&pattern, &l) {
                    writeln!(write_handle, "{}", l);
                }
            }
            Err(e) => println!("error parsing line: {:?}", e),
        }
    }
}


fn string_to_int(val: String) -> Result<(), String> {
    match val.parse::<usize>() {
      Ok(n) => { return Ok(()); },
      Err(e) => {
          return Err(format!("Value {:?} cannot be coerced to integer", val))
      },
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
                            .index(2)
                            .default_value("STDIN"))
                        .arg(Arg::with_name("outputFile")
                            .takes_value(true)
                            .help("Output file to write to")
                            .short("o")
                            .default_value("STDOUT"))
                        .arg(Arg::with_name("after")
                            .takes_value(true)
                            .help("Number of lines to print after each match")
                            .short("A")
                            .default_value("0")
                            .validator(string_to_int))
                        .arg(Arg::with_name("before")
                            .takes_value(true)
                            .help("Number of lines to print before each match")
                            .short("B")
                            .default_value("0")
                            .validator(string_to_int))
                        .get_matches();

    let inputFile = matches.value_of("inputFile").unwrap();
    let outputFile = matches.value_of("outputFile").unwrap();
    let pattern = matches.value_of("pattern").unwrap();
    let after = matches.value_of("after").unwrap().parse::<usize>();
    let before = matches.value_of("before").unwrap().parse::<usize>();
    // let config = Config { before: before, after: after, pattern: pattern }
    let reader = Reader { input: inputFile.to_string() };
    let writer = Writer { output: outputFile.to_string() };

    // match_lines(&reader, &writer, &pattern);
    match_lines_with_buffer(&reader, &writer, &pattern);
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

    // #[test]
    // fn test_stdin_stdout_mock(){
    //     let input = "foo\nbar\nbaz\n";
    //     let mut output = Vec::new();
    // }
}
