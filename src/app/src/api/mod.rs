pub mod create_todo;

use strum::Display;

#[derive(Debug, Display)]
pub enum Operation {
    CreateTodo,
}
