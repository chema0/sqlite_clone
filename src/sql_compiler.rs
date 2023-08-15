use crate::database::Row;

pub trait Statement {
    fn execute(&self);
}

pub struct InsertStatement {
    pub row: Row,
}

impl Statement for InsertStatement {
    fn execute(&self) {
        println!("This is where we would do an insert.")
    }
}

pub struct SelectStatement {}

impl Statement for SelectStatement {
    fn execute(&self) {
        println!("This is where we would do a select.")
    }
}

fn tokenize(input: &str) -> Vec<&str> {
    let tokens: Vec<&str> = input.split(' ').collect();
    return tokens;
}

type Result = std::result::Result<Box<dyn Statement>, String>;

pub fn parse(input: &str) -> Result {
    let tokens = tokenize(input);
    match tokens.first() {
        Some(&"insert") => parse_insert(tokens),
        Some(&"select") => parse_select(tokens),
        _ => Err(format!("Unrecognized keyword at start of '{input}'.")),
    }
}

fn parse_insert(tokens: Vec<&str>) -> Result {
    if tokens.len() < 4 {
        return Err("'insert' statement is missing required arguments.".to_string());
    }

    let row: Row = Row {
        id: tokens[1].parse().expect("The 'id' must be a number!"),
        username: tokens[2].to_string(),
        email: tokens[3].to_string(),
    };

    Ok(Box::new(InsertStatement { row }))
}

fn parse_select(_tokens: Vec<&str>) -> Result {
    Ok(Box::new(SelectStatement {}))
}
