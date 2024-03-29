use std::str::FromStr;

use uuid::Uuid;

use crate::ModelError;

#[derive(Debug)]
pub struct TodoId(Uuid);
impl TodoId {
    pub fn generate() -> Self {
        TodoId(Uuid::new_v4())
    }
}

impl FromStr for TodoId {
    type Err = ModelError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::from_str(s).map_err(|err| ModelError::Validation(format!("{:?}", err)))?;
        Ok(Self(uuid))
    }
}

impl From<Uuid> for TodoId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<TodoId> for Uuid {
    fn from(value: TodoId) -> Self {
        value.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ok_from_str() {
        let s = "550e8400-e29b-41d4-a716-446655440000";
        assert!(TodoId::from_str(s).is_ok())
    }

    #[test]
    fn err_from_str() {
        let s = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        assert!(TodoId::from_str(s).is_err())
    }
}
