pub mod compiler;
pub mod database;
pub mod repl;

fn main() {
    repl::start_repl();
}
