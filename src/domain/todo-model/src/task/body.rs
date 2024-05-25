use std::{fmt::Display, str::FromStr};

use crate::typed::{StringValidator, ValidationError};

#[derive(Debug)]
pub struct TaskBody(String);

impl TaskBody {
    const MAX_LENGTH: usize = 1000;
    pub fn new<A: Into<String>>(input: A) -> Result<Self, ValidationError> {
        let value = StringValidator::<TaskBody>::default()
            .set_upper_limit(Self::MAX_LENGTH)
            .validation(input)?;
        Ok(Self(value))
    }

    pub fn new_unchecked(s: &str) -> Self {
        Self(s.into())
    }
}

impl Display for TaskBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for TaskBody {
    type Err = ValidationError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest(input)]
    #[case::normal_text("nomal contents. これは普通の文章です。")]
    #[case::max_length(&"max文字数@123".repeat(100))]
    #[case::trimed("\nmax文字数@123\n")]
    fn ok_from_str(input: &str) {
        let text = TaskBody::from_str(input).expect("error");
        assert_eq!(text.to_string(), input.trim().to_string())
    }

    #[rstest(input)]
    #[case::empty("")]
    #[case::max_length(&format!("{}a", "max文字数@123".repeat(100)))]
    fn err_from_str(input: &str) {
        let text = TaskBody::from_str(input);
        assert!(text.is_err())
    }
}
