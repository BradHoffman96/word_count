use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/THEGODFATHER.dat";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for word in line.split_whitespace() {
            println!("{}", word);
        }
    }
}
