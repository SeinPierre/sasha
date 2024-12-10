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


pub fn get_endianess(n: u8) -> &'static str {
    match n {
      1 => "little",
      2 => "big",
      _ => "Wrong"
    }
  }
  
pub fn get_elf(n: u8) -> &'static str {
    match n {
      1 => "32",
      2 => "64",
      _ => "Wrong"
    }
  }
  
pub fn get_os(n: u8) -> &'static str {
    match n {
      0x00 => "System V",
      0x01 => "HP-UX",
      0x02 => "NetBSD",
      0x03 => "Linux",
      0x04 => "Gnu Hurd",
      0x06 => "Solaris",
      0x07 => "AIX",
      0x08 => "IRIX",
      0x09 => "FreeBSD",
      0x0A => "Tru64",
      0x0B => "Novell Modesto",
      0x0C => "OpenBSD",
      0x0D => "OpenVMS",
      0x0E => "NonStop Kernel",
      0x0F => "AROS",
      0x10 => "FenixOS",
      0x11 => "Nuxi CloudABI",
      0x12 => "Stratus Technologies OpenVOS",
      _ => "Unkown"
    }
  }

// Byte to Instruction : http://ref.x86asm.net/geek-abc.html / http://ref.x86asm.net/coder.htm / https://sandpile.org/

pub fn get_opcodes_lines(vec: Vec<u8>) -> Vec<Vec<u8>> {
    let mut res: Vec<Vec<u8>> = vec![];
    let mut it = vec.iter();
    while let Some(&i) = it.next() {
      let mut v: Vec<u8> = vec![0];
      v[0]=i;
      // If 2 bytes opcodes
      if i == 0x0F {       
        let &i = it.next().unwrap();
        v.push(i);
      }
      res.push(v);
    }

    res
}