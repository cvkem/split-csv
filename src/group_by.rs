use std::{
    collections::HashMap,
    fs::File,
    io::{self, prelude::*, BufReader},
    path::Path
};

use super::output_file::OutputFile;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    // only accept full column matches
    static ref LABEL: Regex = Regex::new(r"^\s*([\w\d\-\+_]+)\s*$").unwrap();
}

struct Acc<'a> {
    num_lines: u64,
    stem: &'a str,
    ext: &'a str,
    header_line: &'a str,
    writers: HashMap<String, OutputFile>}

impl<'a> Acc<'a> {

    /// get the output-file for the current 'label'
    fn append_line(&mut self, label: String, line: &str) {
        // Create the file if is does not exist.
        self.writers.entry(label.clone()).or_insert_with(|| {
            let mut output_file = OutputFile::new(self.stem, &label, self.ext).unwrap();
            match output_file.write(&self.header_line) {
                Ok(_) => (),
                Err(err) => panic!("Failed to write headerline for file with label '{label}': {err:?}")
            };
            output_file
        });
        match self.writers.get_mut(&label).unwrap().write(line) {
            Ok(_) => (),
            Err(err) => panic!("Writing failed with error: {err:?}")
        };
    }
}


/// split a csv-file in a numbered set of CSV-files each containing at most 'max_num_lines' data-rows.
pub fn group_by(file_name: String, separator: char, group_by_column: usize) -> Result<(), io::Error> {
    println!("Performing a group-by for column {group_by_column}  based on separator: '{separator}'.");

    let file = File::open(&file_name).expect("Failed to open file: '{file_name}'");
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let first_line = lines.next().expect("First line missing").unwrap();

    let path = Path::new(&file_name);
    let acc = Acc{
        num_lines: 0, 
        stem: path.file_stem().expect("Could not find stem of path").to_str().unwrap(),
        ext: path.extension().expect("Could not find extension of path (file).").to_str().unwrap(),
        header_line: &first_line,
        writers: HashMap::new()};

    lines.fold(acc, |mut acc: Acc, s: Result<String, io::Error>| {

            // write contents to the current file
            let s = s.unwrap();
            acc.num_lines += 1;

            let inserted = match s.split(separator).skip(group_by_column - 1).next() {
                Some(column) => if let Some(caps) = LABEL.captures(column) {
                                        acc.append_line(caps[1].to_string(), &s);
                                        true
                                    } else {
                                        println!("No capture for column '{column}'. Full line: {s}");
                                        false
                                    },
                None => {
                    println!("Column {group_by_column} does not exist");
                    false
                }
            };
            if !inserted {
                acc.append_line(String::new(), &s);
            }

            acc
        });

    Ok(())
}