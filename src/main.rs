use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let filename = "src/THEGODFATHER.dat";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut words = HashMap::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for word in line.split_whitespace() {
            let key = String::from(word);
            let count = words.entry(key).or_insert(0);
            *count += 1;
        }
    }

    println!("{:?}", words);
}
