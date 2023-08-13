use std::mem;

pub enum StatementType {
    Insert,
    Select,
}

pub struct Statement {
    r#type: StatementType,
    row_to_insert: Row,
}

/* Limited schema, users table represented by:
    ----------------------
    id: integer
    username: varchar(32)
    email: varchar(255)
    ----------------------
*/

const COLUMN_USERNAME_SIZE: u8 = 32;
const COLUMN_EMAIL_SIZE: u8 = 255;

pub struct Row {
    id: u32,
    username: String,
    email: String,
}

const ID_SIZE: usize = field_size::<Row::id>();
// const USERNAME_SIZE: usize = field_size!(Row::username);

type Result<'a> = std::result::Result<&'a Statement, String>;

fn tokenize(input: &str) -> Vec<&str> {
    let tokens: Vec<&str> = input.split(' ').collect();
    return tokens;
}

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
    Ok(&Statement::Insert)
}

fn parse_select(_tokens: Vec<&str>) -> Result {
    Ok(&Statement::Select)
}

fn field_size<T>() -> usize {
    mem::size_of::<T>()
}

// trait FieldSize {
//     fn size() -> usize;
// }

// impl FieldSize for u32 {
//     fn size() -> usize {
//         size_of::<u32>()
//     }
// }

// impl FieldSize for String {
//     fn size() -> usize {
//         size_of::<String>()
//     }
// }
