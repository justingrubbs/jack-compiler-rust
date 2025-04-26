mod ast {
    pub mod asm;
    pub mod jack;
    pub mod token;
    pub mod vm;
}
mod compiler {
    pub mod assembler;
    pub mod jack_to_vm;
    pub mod lexer;
    pub mod parser;
    pub mod vm_to_asm;
}
mod pretty_printer {
    pub mod asm;
    pub mod jack;
    pub mod lexer;
    pub mod vm;
}
mod test {
    pub mod asm_parser;
    pub mod tests;
    pub mod vm_parser;
}

use std::env;
use std::fs;
use std::io::{self, Error};

use chumsky::Parser;
use std::path::Path;

fn main() -> Result<(), Error> {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    let path = &args[1];

    // Check if the path is a file or directory
    let metadata = fs::metadata(path)?;

    if metadata.is_file() {
        let hack = jack_to_hack(path)?;
        let file_name = path.trim_end_matches(".jack");
        let hack_string = hack.join("\n");
        let output_path = format!("{}.hack", file_name);
        fs::write(output_path, hack_string)?;
    } else if metadata.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_path = entry.path();
            if file_path.extension().and_then(|s| s.to_str()) == Some("jack") {
                let hack = jack_to_hack(file_path.to_str().unwrap())?;
                let hack_string = hack.join("\n");
                let file_name = file_path.with_extension("hack");
                fs::write(file_name, hack_string)?;
            }
        }
    } else {
        eprintln!("Provided path is neither a file nor a directory.");
    }
    Ok(())
}

// Tokenize a single Jack file
pub fn tokenize_jack_file(file_path: &str) -> Result<Vec<crate::ast::token::Token>, Error> {
    let contents = fs::read_to_string(file_path)?;
    crate::compiler::lexer::tokenize()
        .parse(contents)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{:#?}", e)))
}

// Parse a single Jack file
pub fn parse_jack_file(file_path: &str) -> Result<crate::ast::jack::Class, Error> {
    let tokens = tokenize_jack_file(file_path)?;
    crate::compiler::parser::parse_class()
        .parse(tokens)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{:#?}", e)))
}

pub fn jack_to_hack(file_path: &str) -> Result<Vec<String>, Error> {
    let file_name = Path::new(file_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(file_path)
        .to_string();
    parse_jack_file(file_path)
        .map(|class| crate::compiler::jack_to_vm::JackToVm::compile(file_name.clone(), class))
        .map(|vm| crate::compiler::vm_to_asm::VmToAsm::compile(file_name, vm))
        .map(|asm| crate::compiler::assembler::Assembler::assemble(asm))
}

// Compile a single Jack file into VM
pub fn jack_to_vm(file_path: &str) -> Result<Vec<crate::ast::vm::Command>, Error> {
    let file_name = Path::new(file_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(file_path)
        .to_string();
    parse_jack_file(file_path)
        .map(|class| crate::compiler::jack_to_vm::JackToVm::compile(file_name.to_string(), class))
}

pub fn parse_asm_file(file_path: &str) -> Result<Vec<crate::ast::asm::Assembly>, Error> {
    let contents = fs::read_to_string(file_path)?;
    crate::test::asm_parser::parse_assembly()
        .parse(contents)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{:#?}", e)))
}

// Compile a single ASM file into hack
pub fn assembler(file_path: &str) -> Result<Vec<String>, Error> {
    parse_asm_file(file_path).map(|v_asm| crate::compiler::assembler::Assembler::assemble(v_asm))
}

pub fn parse_vm_file(file_path: &str) -> Result<Vec<crate::ast::vm::Command>, Error> {
    let contents = fs::read_to_string(file_path)?;
    crate::test::vm_parser::parse_vm()
        .parse(contents)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{:#?}", e)))
}

// Compile a VM file or directory containing multiple VM files to a single ASM file
pub fn vm_to_asm(path: &str) -> Result<Vec<crate::ast::asm::Assembly>, Error> {
    let metadata = fs::metadata(path)?;
    if metadata.is_file() {
        let file_name = Path::new(path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(path)
            .to_string();
        parse_vm_file(path).map(|commands| {
            crate::compiler::vm_to_asm::VmToAsm::compile(file_name.to_string(), commands)
        })
    } else {
        let mut vm_files: Vec<(String, Vec<crate::ast::vm::Command>)> = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_path = entry.path();
            if file_path.extension().and_then(|s| s.to_str()) == Some("vm") {
                let file_name = file_path.file_stem().unwrap().to_str().unwrap().to_string();
                let vm = parse_vm_file(file_path.to_str().unwrap())?;
                vm_files.push((file_name, vm));
            }
        }
        let asm: Vec<_> = vm_files
            .into_iter()
            .flat_map(|(file_name, commands)| {
                crate::compiler::vm_to_asm::VmToAsm::compile(file_name, commands)
            })
            .collect();
        Ok(asm)
    }
}
