use super::{ToDoId, model::ToDo};

pub trait ToDoUseCase {
    fn create(todo: String) -> ToDoId;
    fn delete(id: ToDoId) -> bool;
}

pub trait DBService {
    fn create_todo(message: &str) -> ToDo;
    fn delete_todo(id: ToDoId) -> bool;
}

pub struct Interactor {
    pub todo_usecase: dyn ToDoUseCase
}

