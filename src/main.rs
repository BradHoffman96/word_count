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
    collection.sort_by(|a, b| {
        if a.1 == b.1 {
            a.0.cmp(b.0)
        } else {
            b.1.cmp(a.1)
        }
    });

    let output = "output.txt";
    let mut file = File::create(output).unwrap();

    for (word, count) in collection {
        //println!("{} {}", word, count);
        writeln!(&mut file, "{} {}", word, count).unwrap();
    }

    compare();
}

fn compare() {
    let file = File::open("output.txt").unwrap();
    let metadata = file.metadata().unwrap();

    let correct = File::open("THEGODFATHER_CORRECT_WC.dat").unwrap();
    let correct_metadata = correct.metadata().unwrap();

    if metadata.len() == correct_metadata.len() {
        println!("Files match!");
    } else {
        println!("Correct size: {}, Output size: {}", correct_metadata.len(), metadata.len());
    }
}