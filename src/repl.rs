use std::{
    io::{self, Write},
    process::exit,
};

const SUCCESS_EXIT: i32 = 1;

pub fn start_repl() {
    loop {
        print_prompt();
        let input = read_string();

        if is_meta_command(&input) {
            match do_meta_command(&input) {
                MetaCommandResult::UnrecognizedCommand => {
                    println!("Unrecognized command '{input}'.");
                    continue;
                }
                _ => continue,
            }
        }

        let statement = match prepare_statement(&input) {
            Some(x) => x,
            None => {
                println!("Unrecognized keyword at start of '{input}'");
                continue;
            }
        };

        execute_statement(&statement);
        println!("Executed.")
    }
}

fn read_string() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("can not read user input");
    let cleaned_input = input.trim().to_string();
    cleaned_input
}

fn print_prompt() {
    print!("db > ");
    io::stdout().flush().expect("failed to write into stdout");
}

fn is_meta_command(command: &String) -> bool {
    command.starts_with(".")
}

enum MetaCommandResult {
    _Success,
    UnrecognizedCommand,
}

fn do_meta_command(input: &String) -> MetaCommandResult {
    if input == ".exit" {
        exit(SUCCESS_EXIT)
    }
    MetaCommandResult::UnrecognizedCommand
}

enum Statement {
    Insert,
    Select,
}

fn prepare_statement(input: &String) -> Option<&Statement> {
    match input.as_str() {
        "insert" => Some(&Statement::Insert),
        "select" => Some(&Statement::Select),
        _ => None,
    }
}

fn execute_statement(statement: &Statement) {
    match statement {
        Statement::Insert => {
            println!("This is where we would do an insert.")
        }
        Statement::Select => {
            println!("This is where we would do a select.")
        }
    }
}
