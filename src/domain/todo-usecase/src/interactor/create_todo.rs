use todo_model::{ToDo, ToDoId};

use crate::error::Result;

use super::Interactor;

pub struct CreateTodoInput {
    pub content: String,
}

impl Interactor {
    pub async fn create_todo(input: CreateTodoInput) -> Result<ToDo> {
        let _todo = ToDo {
            id: ToDoId::generate(),
            content: input.content,
            ..Default::default()
        };
        todo!()
    }
}
