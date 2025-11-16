use parser_of_json::{get_field, parse_json, validate_json};
use std::env;
use std::fs;

fn print_instructure() {
    println!(
        "\n
        JSON Parser CLI

        Comands:
            parse <file_path>        Parse JSON file
            validate <file_path>     Validate JSON file
            get <file_path> <key>    Extract a field from JSON

        Examples:
            parse data.json
            get data.json city
            validate input.json\n"
    );
}

fn run_validate(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Error: missing file path");
        return;
    }

    let path = &args[2];
    let content = fs::read_to_string(path).unwrap_or_else(|_| panic!("Cannot read file: {}", path));

    println!("Is JSON valid? {}", validate_json(&content));
}

fn run_parse(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Error: missing file path");
        return;
    }

    let path = &args[2];
    let content = fs::read_to_string(path).unwrap_or_else(|_| panic!("Cannot read file: {}", path));

    match parse_json(&content) {
        Ok(ast) => println!("JSON is: \n{:#?}", ast),
        Err(e) => eprintln!("Parse error: {:?}", e),
    }
}

fn run_get(args: &[String]) {
    if args.len() < 4 {
        eprintln!("Usage: get <file> <key>");
        return;
    }

    let path = &args[2];
    let key = &args[3];

    let content = fs::read_to_string(path).unwrap_or_else(|_| panic!("Cannot read file: {}", path));

    let root = parse_json(&content).unwrap();

    let result = get_field(&root, key);

    if let Some(v) = result {
        println!("{:#?}", v);
    } else {
        println!("Key '{}' not found", key);
    }
}

fn main() {
    print_instructure();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Missing commands");
        return;
    }

    let comm = &args[1];
    if comm == "validate" {
        run_validate(&args);
        return;
    }

    if comm == "parse" {
        run_parse(&args);
        return;
    }

    if comm == "get" {
        run_get(&args);
    }
}
