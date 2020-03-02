use std::io::Read;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Lines, Write};
use std::io::prelude;

fn main() {
    let path = "./foo.txt";
    println!("Writing some data to {}", path);
    write_file(path, "Hello World!\n").expect("Failed to write to file")

    // let content = read_file(path).expect("Failed to read file");

}

fn read_file(path: &str) -> io::Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    Ok(content)
}

fn read_line_iterator(path: &str) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);
    Ok(buf_reader.lines())
}

fn write_file(path: &str, content: &str) -> io::Result<()> {
    let file = File::create(path)?;
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write_all(content.as_bytes())?;
    Ok(())
}

fn append_file(path: &str, content: &str) -> io::Result<()> {
    let file = OpenOptions::new().append(true).open(path);
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write_all(content.as_bytes())?;
    Ok(())
}

fn append_and_read(path: &str, content: &str) -> io::Result<()> {
    let file = OpenOptions::new().read(true).append(true).open(path)?;
    let mut buf_reader = BufReader::new(&file);
    let mut buf_writer = BufWriter::new(&file);

    let mut file_content = String::new();
    buf_reader.read_to_string(&mut file_content)?;
}