use std::env;
use std::fs;
use std::time::Instant;

mod disassembler;

fn main() {
	let start = Instant::now();
	let args: Vec<String> = env::args().collect();
	if args.len() == 2 {
		let filename = &args[1];
		let bytes: Vec<u8> = fs::read(filename).expect("Failed to read binary file.");
		
		let mut program: Vec<i64> = Vec::new();
		let mut byte_count = 0;
		let mut chunk_value: i64 = 0;
		for byte in bytes {
		    if byte_count == 0 {
		        chunk_value = (byte as i64) << 56;
		        byte_count += 1;
		    } else if byte_count == 1 {
		        chunk_value |= (byte as i64) << 48;
		        byte_count += 1;
		    } else if byte_count == 2 {
		        chunk_value |= (byte as i64) << 40;
		        byte_count += 1;
		    } else if byte_count == 3 {
		        chunk_value |= (byte as i64) << 32;
		        byte_count += 1;
		    } else if byte_count == 4 {
		        chunk_value |= (byte as i64) << 24;
		        byte_count += 1;
		    } else if byte_count == 5 {
		        chunk_value |= (byte as i64) << 16;
		        byte_count += 1;
		    } else if byte_count == 6 {
		        chunk_value |= (byte as i64) << 8;
		        byte_count += 1;
		    } else if byte_count == 7 {
		        chunk_value |= byte as i64;
		        program.push(chunk_value);
		        chunk_value = 0;
		        byte_count = 0;
		    }
		}

		let mut disassembler = disassembler::Disassembler::new(program);
		disassembler.disassemble();
	}
    let time: f64 = start.elapsed().as_micros() as f64 / 1000000 as f64;
    println!("Program completed in {} seconds.", time);
}
