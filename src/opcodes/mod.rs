use std::io::{Read, Error, ErrorKind, Result};
use std::fmt::Write;

mod opcode_gen;

fn read_bytes<T: Read>(mut stream: T, bytes: u8, mut asm: &mut String)
    -> Result<u16> {
    let mut narg : u16 = 0;
    for i in 0..bytes {
        let mut arg_buffer = [0; 1];
        let n = try!(stream.read(&mut arg_buffer));
        if n == 0 {
            return Err(Error::new(ErrorKind::Other, "Unexpected EOF"))
        }
        narg |= (arg_buffer[0] as u16)  << (i * 8);
        write!(&mut asm, " {:02x}", arg_buffer[0]).ok();
    }
    Ok(narg)
}

pub fn print_opcode<T: Read>(mut stream: T, index: &mut u64) -> Result<()> {
    let mut buffer = [0; 1];
    let n = try!(stream.read(&mut buffer));
    if n == 0 {
        return Err(Error::new(ErrorKind::Other, "Unexpected EOF"))
    }

    let (instr, size, args) = opcode_gen::lookup_opcode(buffer[0]);

    let mut formatted_op = String::new();
    write!(&mut formatted_op, "{:4}", instr).ok();

    let mut asm = String::new();
    write!(&mut asm, "{:02x}", buffer[0]).ok();

    let mut byte_args = 0;
    for arg in args {
        if arg.starts_with("D") && arg != "D" {
            let bytes = arg[1..].parse::<u8>().ok().expect("parse error") / 8;
            byte_args += bytes;
            let narg = try!(read_bytes(&mut stream, bytes, &mut asm));
            write!(&mut formatted_op, " #${:02x}", narg).ok();
        } else if arg == "adr" {
            byte_args += 2;
            let narg = try!(read_bytes(&mut stream, 2, &mut asm));
            write!(&mut formatted_op, " ${:02x}", narg).ok();
        } else {
            write!(&mut formatted_op, " {}", arg).ok();
        }
    }
    assert_eq!(byte_args, size - 1);

    *index += size as u64;
    println!("{:07} {:8} {}", *index, asm, formatted_op);
    Ok(())
}
