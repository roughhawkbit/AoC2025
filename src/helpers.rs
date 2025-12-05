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

pub fn read_char_grid<P>(filename: P) -> io::Result<Vec<Vec<char>>>
where P: AsRef<Path>, {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    for line in reader.lines().map_while(Result::ok){
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }
    Ok(grid)
}