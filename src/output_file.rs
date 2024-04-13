use std::{
    fs::File,
    io::{self, Write, BufWriter}
};


pub struct OutputFile (BufWriter<File>);

impl OutputFile {

    pub fn new(stem: &str, part: &str, ext: &str) -> Result<Self, io::Error> {
        let part_file_str = format!("{stem}-{part}.{ext}");
        println!("Create file: {part_file_str}");
        let part_file = File::create(part_file_str)?;
        let writer = BufWriter::new(part_file);
        Ok(OutputFile(writer))
    }

    pub fn write(&mut self, line: &str) -> Result<(), io::Error> {
        let _ = self.0.write(line.as_bytes())?;
        let _ = self.0.write(b"\n")?;
        Ok(())
    }
}
