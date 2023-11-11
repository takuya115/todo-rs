use super::ToDoId;

pub trait ToDoUseCase {
    fn create(todo: String) -> ToDoId;
    fn delete(todo: ToDoId) -> bool;
}
