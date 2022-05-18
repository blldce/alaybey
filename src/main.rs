pub mod endian;
pub mod helper;
use crate::endian::write_u32;
use crate::{endian::read_i32, helper::load_text_from_file};
use std::{
    fs::File,
    io::{Read, Write},
};

use helper::FileError;
use lib::{compiler::language, vm::AlaybeyVM};

pub mod lib;

// cargo run build program.alaybey
// cargo run run program.alaybeyvm
fn main() -> Result<(), FileError> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("please enter source file..");
        println!("USAGE : stack-vm build or run filename");
        println!("Example :");
        println!("stack-vm build myprogram.alaybey");
        println!("stack-vm run myprogram.alaybeyvm");
        return Ok(());
    }
    let command = args[1].as_str();
    let file_name = args[2].as_str();
    match helper::get_command(command) {
        helper::Operation::Buid => {
            let is_file_ext = match helper::check_file_ext(file_name, "alaybey") {
                Ok(is_file_ext) => is_file_ext,
                Err(e) => return Err(e),
            };

            if is_file_ext {
                let mut language_text = String::new();
                if let Err(err) = load_text_from_file(file_name, &mut language_text) {
                    return Err(err);
                }
                let tokens = language::lexer(language_text.as_str());
                let parsed_data = language::parser(tokens);
                let output_file = match helper::create_file(file_name) {
                    Ok(output_file) => output_file,
                    Err(e) => return Err(e),
                };
                let mut file = match File::create(&output_file) {
                    Ok(file) => file,
                    Err(_) => return Err(FileError::CreateFile),
                };
                let mut buf = [0; 4];
                for data in parsed_data {
                    write_u32(&mut buf, data.to_le() as u32);
                    if file.write(&buf).is_err() {
                        return Err(FileError::WriteFile);
                    }
                }
                println!("Build success! Path : {}", &output_file);
            } else {
                return Err(FileError::FetchFile);
            }
        }
        helper::Operation::Run => {
            let is_file_ext = match helper::check_file_ext(file_name, "alaybeyvm") {
                Ok(is_file_ext) => is_file_ext,
                Err(e) => return Err(e),
            };
            if is_file_ext {
                let mut file = match File::open(args[2].clone()) {
                    Ok(file) => file,
                    Err(_) => return Err(FileError::OpenFile),
                };
                let file_len = match file.metadata() {
                    Ok(meta) => meta.len(),
                    Err(_) => return Err(FileError::Metadata),
                };
                let mut endian_next_instuction = 0; // ie: 0700 0040 0900 ...
                let mut program_instructions: Vec<i32> = Vec::new();
                let mut buffer_arr = [0; 4];
                while endian_next_instuction < file_len {
                    if file.read_exact(&mut buffer_arr).is_err() {
                        return Err(FileError::ReadFile);
                    }
                    let int = match read_i32(&buffer_arr[..]) {
                        Ok(int) => int,
                        Err(_) => return Err(FileError::ReadFile),
                    };

                    program_instructions.push(int);
                    endian_next_instuction += 4;
                }
                let mut virtual_machine = AlaybeyVM::new(helper::DEFAULT_RUNTME_STACK_SZE);
                virtual_machine.load_program(&program_instructions);

                if virtual_machine.run().is_err() {
                    return Err(FileError::Run);
                }
            } else {
                return Err(FileError::FetchFile);
            }
        }
        helper::Operation::Unknown => return Err(FileError::Unknown),
    }
    Ok(())
}
