pub mod database;
pub mod repl;
pub mod sql_compiler;

fn main() {
    repl::start_repl();
}
