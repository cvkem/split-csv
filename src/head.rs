use std::{
    fs::File,
    io::{BufRead, BufReader}
};

/// Show the 'num' headlines of the file 'file_name'.
pub fn head_lines(file_name: String, num: usize) {
    
    let file = File::open(&file_name[..]).expect("Failed to open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .take(num)
        .for_each(|l| println!("{}", l.expect("Could not read line")));
}