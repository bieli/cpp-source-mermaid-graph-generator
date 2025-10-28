mod parser;

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <source.cpp> [skip_list]", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];
    let skip_list: Vec<String> = if args.len() >= 3 {
        args[2].split(',').map(|s| s.trim().to_string()).collect()
    } else {
        vec!["cout".to_string(), "cin".to_string()]
    };

    let input = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("Error reading '{}': {}", file_path, err);
        process::exit(2);
    });

    let mermaid_graph = parser::generate_mermaid_graph(&input, file_path, &skip_list);
    println!("{}", mermaid_graph);
}

