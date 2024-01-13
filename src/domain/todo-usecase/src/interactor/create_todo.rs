use std::str::FromStr;

use todo_model::{Text, Todo, TodoId};

use crate::error::{Error, Result};

use super::Interactor;

pub struct CreateTodoInput {
    pub content: String,
}

impl Interactor {
    pub async fn create_todo(&self, input: CreateTodoInput) -> Result<Todo> {
        let content =
            Text::from_str(&input.content).map_err(|err| Error::InvalidInput(Box::new(err)))?;
        self.gateway
            .db_service()
            .create_todo(TodoId::generate(), content)
            .await
    }
}
