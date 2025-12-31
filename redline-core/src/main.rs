use std::env;
use std::fs;
use std::process;

mod codegen;
mod lexer;
mod parser;
mod ast;

use lexer::Lexer;
use parser::Parser;
use codegen::generate;

fn report_error(file_path: &str, input: &str, message: &str, line: usize, column: usize) {
    eprintln!("\nError: {}", message);
    eprintln!("  --> {}:{}:{}", file_path, line, column);

    if line > 0 {
        if let Some(line_str) = input.lines().nth(line - 1) {
            let line_num_str = line.to_string();
            let line_num_width = line_num_str.len();

            eprintln!("{:>width$} |", "", width = line_num_width);
            eprintln!("{} | {}", line, line_str);
            eprintln!("{:>width$} | {:>col$}", "", "^", width = line_num_width, col = column);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: redline-core <file.rl>");
        process::exit(1);
    }

    let file_path = &args[1];
    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file [{}]: {}", file_path, e);
            process::exit(1);
        }
    };

    let tokens = match Lexer::new(content.clone()).tokenize() {
        Ok(t) => t,
        Err(e) => {
            report_error(file_path, &content, &e.message, e.line, e.column);
            process::exit(1);
        }
    };

    let program = match Parser::new(&tokens).parse() {
        Ok(p) => p,
        Err(e) => {
            report_error(file_path, &content, &e.message, e.line, e.column);
            process::exit(1);
        }
    };

    match generate(&program) {
        Ok(cpp_code) => {
            print!("{}", cpp_code);
        }
        Err(e) => {
            // For now, CodegenError does not have location info.
            // This could be a future improvement.
            eprintln!("Codegen Error: {}", e);
            process::exit(1);
        }
    }
}
