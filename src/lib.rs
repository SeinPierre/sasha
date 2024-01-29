use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Read;

pub fn really_complicated_code(a: u8, b: u8) -> Result<u8, Box<dyn Error>> {
    Ok(a + b)
}


pub fn open_binary(filename: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    Ok(buffer)
}
