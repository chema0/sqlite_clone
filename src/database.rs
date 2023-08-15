// use std::mem;

/* Limited schema, users table represented by:
    ----------------------
    id: integer
    username: varchar(32)
    email: varchar(255)
    ----------------------
*/

// const COLUMN_USERNAME_SIZE: u8 = 32;
// const COLUMN_EMAIL_SIZE: u8 = 255;

pub struct Row {
    pub id: u32,
    pub username: String,
    pub email: String,
}

// const ID_SIZE: usize = mem::size_of::<u32>();
// const USERNAME_SIZE: usize = mem::size_of::<String>();
// const EMAIL_SIZE: usize = mem::size_of::<String>();

/* FIXME: need to deal with ambiguous associated type */
// fn field_size<T>() -> usize {
//     mem::size_of::<T>()
// }
