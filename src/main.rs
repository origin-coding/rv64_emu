mod bus;
mod cpu;
mod dram;
mod instruction;
mod register;

use std::{env, io};
use std::fs::File;
use std::io::Read;

use crate::cpu::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!(
            "Usage:\n\
            - cargo run <filename>"
        );
    }

    let mut file = File::open(&args[1])?;
    let mut code = Vec::new();
    file.read_to_end(&mut code)?;

    let mut cpu = Cpu::new(code);

    loop {
        // 1. Fetch.
        let inst = match cpu.fetch() {
            // Break the loop if an error occurs.
            Ok(inst) => inst,
            Err(_) => break,
        };

        // 2. Add 4 to the program counter.
        cpu.pc += 4;

        // 3. Decode.
        // 4. Execute.
        match cpu.execute(inst) {
            // Break the loop if an error occurs.
            Ok(_) => {}
            Err(_) => break,
        }

        // This is a workaround for avoiding an infinite loop.
        // TODO avoid counting pc manually
        if cpu.pc == 0 || cpu.pc - 0x8000_0000 >= 12 {
            break;
        }
    }

    cpu.dump_registers();
    Ok(())
}
