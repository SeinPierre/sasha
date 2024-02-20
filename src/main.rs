use std::process::{ExitCode, Termination};

pub enum LinuxExitCode { EOk, EErr(u8) }

impl Termination for LinuxExitCode {
   fn report(self) -> ExitCode {
     match self {
       LinuxExitCode::EOk => ExitCode::SUCCESS,
       LinuxExitCode::EErr(v) => ExitCode::from(v)
     }
   }
}

const HEADER_LENGTH_64:usize = 64;
const HEADER_LENGTH_32:usize = 32;


fn main() -> LinuxExitCode {
    let result:Vec<u8> = sasha::open_binary("../nth_prime/target/release/nth_prime").unwrap();
    let header_size:usize;
    match result[4] {
        1 => header_size = HEADER_LENGTH_32,
        2 => header_size = HEADER_LENGTH_64,
        _=> header_size = 0
    };
    let mut file_header: Vec<u8> = vec![0;header_size];
    file_header.clone_from_slice(&result[0..header_size]);
    
    // println!("Header :\n{:02x?}",&file_header);
    match file_header[..] {
      [0x7f, b'E', b'L', b'F', b, e, 1, os,..] => println!("This is an ELF{} file with {} endianess for {}", 
                                                              sasha::get_elf(b),
                                                              sasha::get_endianess(e),
                                                              sasha::get_os(os)
                                                            ),
      _ => print!("Not an ELF file")
    }
    LinuxExitCode::EOk
}

#[cfg(test)]
mod tests {

    #[test]
    fn ut_read_simple_program(){
        let result = sasha::open_binary("../nth_prime/target/release/nth_prime");        
        assert_eq!(result.unwrap().len(),4407792);
    }
}