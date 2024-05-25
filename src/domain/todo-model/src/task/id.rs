use std::{fmt::Display, str::FromStr};

use uuid::Uuid;

use crate::{typed::ValidationError, ModelError};

#[derive(Debug)]
pub struct TaskId(Uuid);
impl TaskId {
    pub fn new<A: Into<String>>(value: A) -> Result<Self, ValidationError> {
        let inner = |value: String| {
            let uuid = Uuid::from_str(&value).map_err(|err| ValidationError::InvalidFormat {
                src: "TaskId".into(),
                input: value,
                detail: format!("{:?}", err),
            })?;
            Ok(Self(uuid))
        };
        inner(value.into())
    }
    pub fn generate() -> Self {
        TaskId(Uuid::new_v4())
    }
}

impl Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl FromStr for TaskId {
    type Err = ValidationError;
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
        let result = TaskId::from_str(s);
        assert!(result.is_err());
        println!("{:#?}", result)
    }
}
