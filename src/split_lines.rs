use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
    path::Path
};

use super::output_file::OutputFile;


struct Acc<'a> {
    num_lines: u64,
    stem: &'a str,
    ext: &'a str,
    header_line: &'a str,
    part_nr: u32,  
    writer: Option<OutputFile>}

impl<'a> Acc<'a> {
    fn next_file(mut self) -> Self {
        self.part_nr += 1;
        let mut writer = OutputFile::new(self.stem, &self.part_nr.to_string(), self.ext).unwrap();
        match writer.write(self.header_line) {
            Ok(_) => (),
            Err(err) => panic!("Error while writing header-line: {err:?}")
        };
        self.writer = Some(writer);
        self
    }
}


/// split a csv-file in a numbered set of CSV-files each containing at most 'max_num_lines' data-rows.
pub fn split_lines(file_name: String, max_num_lines: u64) -> Result<(), io::Error> {

    let file = File::open(&file_name).expect("Failed to open file: '{file_name}'");
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let first_line = lines.next().expect("First line missing").unwrap();

    let path = Path::new(&file_name);
    let acc = Acc{
        num_lines: 0, 
        part_nr: 0,
        stem: path.file_stem().expect("Could not find stem of path").to_str().unwrap(),
        ext: path.extension().expect("Could not find extension of path (file).").to_str().unwrap(),
        header_line: &first_line,
        writer: None};

    lines.fold(acc, |acc: Acc, s: Result<String, io::Error>| {
            // check whether we need to move to the next file
            let mut acc = if acc.num_lines % max_num_lines == 0 || acc.writer.is_none() {
                acc.next_file()
            } else {
                acc
            };

            // write contents to the current file
            let s = s.unwrap();
            acc.num_lines += 1;

            match acc.writer.as_mut().unwrap().write(&s) {
                Ok(_) => (),
                Err(err) => panic!("Error during write: {err:?}")
            };

            acc
        });

    Ok(())
}