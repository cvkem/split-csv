
use std::{
    fs::File,
    io::{BufRead, BufReader}
};

/// count the number of lines in a datafiles (the header-line is included in the count)
pub fn count_lines(file_name: String) {
    
    let file = File::open(&file_name[..]).expect("Failed to open file");
    let reader = BufReader::new(file);

    let count = reader
        .lines()
        .fold(0, |acc, _| acc + 1);

    println!(" The file '{file_name}' has {count} lines");
}
