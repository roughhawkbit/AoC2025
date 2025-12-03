use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_comma_list<P>(filename: P) -> io::Result<io::Split<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).split(b','))
}