use std::str::FromStr;

use uuid::Uuid;

#[derive(Debug)]
pub struct ToDoId(Uuid);
impl ToDoId {
    pub fn generate() -> Self {
        ToDoId(Uuid::new_v4())
    }
}

// impl FromStr for ToDoId {
//     type Err = ;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {}
// }
