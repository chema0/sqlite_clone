use std::{
    io::{self, Write},
    process::exit,
};

use crate::sql_compiler::{parse, Statement};

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
            None => continue,
        };

        execute_statement(statement);
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

fn prepare_statement(input: &String) -> Option<Box<dyn Statement>> {
    match parse(input) {
        Ok(statement) => Some(statement),
        Err(err) => {
            eprintln!("{}", err);
            None
        }
    }
}

fn execute_statement(statement: Box<dyn Statement>) {
    statement.execute();
}
