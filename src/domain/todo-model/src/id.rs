use std::str::FromStr;

use uuid::Uuid;

use crate::ModelError;

#[derive(Debug)]
pub struct ToDoId(Uuid);
impl ToDoId {
    pub fn generate() -> Self {
        ToDoId(Uuid::new_v4())
    }
}

impl FromStr for ToDoId {
    type Err = ModelError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::from_str(s).map_err(|err| ModelError::Validation(format!("{:?}", err)))?;
        Ok(Self(uuid))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ok_from_str() {
        let s = "550e8400-e29b-41d4-a716-446655440000";
        let id = ToDoId::from_str(s);
        assert!(id.is_ok())
    }
}
