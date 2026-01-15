use std::env;
use std::fs;
use std::process;
use std::path::Path;

mod codegen;
mod lexer;
mod parser;
mod ast;

use lexer::Lexer;
use parser::Parser;
use codegen::{generate, GenMode};

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
        eprintln!("Usage: redline-core <file.rl> [--json-ast | --gen <hpp|cpp>]");
        process::exit(1);
    }

    let file_path_arg = &args[1];
    let module_name = Path::new(file_path_arg).file_stem().unwrap().to_str().unwrap();

    let mut gen_mode = GenMode::Cpp; // Default to Cpp
    let mut dump_json_ast = false;

    if let Some(gen_flag_pos) = args.iter().position(|arg| arg == "--gen") {
        if let Some(mode_str) = args.get(gen_flag_pos + 1) {
            gen_mode = match mode_str.as_str() {
                "hpp" => GenMode::Hpp,
                "cpp" => GenMode::Cpp,
                _ => {
                    eprintln!("Invalid value for --gen flag. Use 'hpp' or 'cpp'.");
                    process::exit(1);
                }
            };
        } else {
            eprintln!("Missing value for --gen flag. Use 'hpp' or 'cpp'.");
            process::exit(1);
        }
    } else if args.iter().any(|arg| arg == "--json-ast") {
        dump_json_ast = true;
    }

    let content = match fs::read_to_string(file_path_arg) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file [{}]: {}", file_path_arg, e);
            process::exit(1);
        }
    };

    let tokens = match Lexer::new(content.clone()).tokenize() {
        Ok(t) => t,
        Err(e) => {
            report_error(file_path_arg, &content, &e.message, e.line, e.column);
            process::exit(1);
        }
    };

    let program = match Parser::new(&tokens).parse() {
        Ok(p) => p,
        Err(e) => {
            report_error(file_path_arg, &content, &e.message, e.line, e.column);
            process::exit(1);
        }
    };

    if dump_json_ast {
        match serde_json::to_string_pretty(&program) {
            Ok(json_str) => println!("{}", json_str),
            Err(e) => {
                eprintln!("Error serializing AST to JSON: {}", e);
                process::exit(1);
            }
        }
    } else {
        match generate(&program, gen_mode, module_name) {
            Ok(code) => println!("{}", code),
            Err(e) => {
                eprintln!("Codegen Error: {}", e);
                process::exit(1);
            }
        }
    }
}
