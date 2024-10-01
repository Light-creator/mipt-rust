#![forbid(unsafe_code)]

use std::{fs::File, io::BufRead, io::BufReader};
use std::collections::HashSet;
use std::io;
use std::path::Path;


fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let mut hash = HashSet::<String>::new();
    
    if let Ok(lines) = read_lines(args[1].clone()) {
        for line in lines.flatten() {
            hash.insert(line);
        }
    }

    if let Ok(lines) = read_lines(args[2].clone()) {
        for line in lines.flatten() {
            if let Some(success) = hash.take(&line) {
                println!("{success}");
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
