use std::{fmt::Display, str::FromStr};

use crate::ModelError;

#[derive(Debug)]
pub struct Text(String);

impl Text {
    const MAX_LENGTH: usize = 1000;
    pub fn from_str_unchecked(s: &str) -> Self {
        Self(s.into())
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Text {
    type Err = ModelError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        match s.chars().count() {
            c if c == 0 => Err(ModelError::Validation("Text is empty".into())),
            c if c > Self::MAX_LENGTH => Err(ModelError::Validation("Over upper limit".into())),
            _ => Ok(Self(s.into())),
        }
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
        let text = Text::from_str(input).expect("error");
        assert_eq!(text.to_string(), input.trim().to_string())
    }

    #[rstest(input)]
    #[case::empty("")]
    #[case::max_length(&format!("{}a", "max文字数@123".repeat(100)))]
    fn err_from_str(input: &str) {
        let text = Text::from_str(input);
        assert!(text.is_err())
    }
}
