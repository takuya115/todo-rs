use super::ToDoId;

pub trait ToDoUsecase {
    fn create(todo: String) -> ToDoId;
}
