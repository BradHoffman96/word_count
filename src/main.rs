use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
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

    let mut collection : Vec<_> = words.iter().collect();
    collection.sort_by(|a, b| b.1.cmp(a.1));

    let output = "output.dat";
    let mut file = File::create(output).unwrap();

    for (word, count) in collection {
        println!("{} {}", word, count);
        writeln!(&mut file, "{} {}", word, count).unwrap();
    }
}
