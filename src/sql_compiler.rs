use crate::database::Row;

pub trait Statement {}

pub struct InsertStatement<'a> {
    row: &'a Row,
}

impl<'a> Statement for InsertStatement<'a> {}

pub struct SelectStatement {}

impl Statement for SelectStatement {}

fn tokenize(input: &str) -> Vec<&str> {
    let tokens: Vec<&str> = input.split(' ').collect();
    return tokens;
}

type Result<'a> = std::result::Result<&'a dyn Statement, String>;

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
        id: tokens[0].parse().expect("The 'id' must be a number!"),
        username: tokens[1].to_string(),
        email: tokens[2].to_string(),
    };

    Ok(&InsertStatement { row: &row })
}

fn parse_select(_tokens: Vec<&str>) -> Result {
    Ok(&SelectStatement {})
}

/* FIXME: need to deal with ambiguous associated type */
// fn field_size<T>() -> usize {
//     mem::size_of::<T>()
// }
