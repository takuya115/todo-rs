use std::str::FromStr;

use uuid::Uuid;

use crate::ModelError;

#[derive(Debug)]
pub struct TaskId(Uuid);
impl TaskId {
    pub fn new<A: Into<String>>(value: A) -> Result<Self, ModelError> {
        let inner = |value: String| {
            let uuid = Uuid::from_str(&value)
                .map_err(|err| ModelError::Validation(format!("{:?}", err)))?;
            Ok(Self(uuid))
        };
        inner(value.into())
    }
    pub fn generate() -> Self {
        TaskId(Uuid::new_v4())
    }
}

impl FromStr for TaskId {
    type Err = ModelError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl From<Uuid> for TaskId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<TaskId> for Uuid {
    fn from(value: TaskId) -> Self {
        value.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ok_from_str() {
        let s = "550e8400-e29b-41d4-a716-446655440000";
        assert!(TaskId::from_str(s).is_ok())
    }

    #[test]
    fn err_from_str() {
        let s = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        assert!(TaskId::from_str(s).is_err())
    }
}
