pub mod create_todo;

use enum_display::EnumDisplay;

#[derive(Debug, EnumDisplay)]
pub enum Operation {
    CreateTodo,
}
