use std::marker::PhantomData;

#[derive(Debug)]
pub struct StringValidator<T> {
    upper_limit: usize,
    lower_limit: usize,
    _handle: PhantomData<T>,
}

impl<T> Default for StringValidator<T> {
    fn default() -> Self {
        Self {
            upper_limit: usize::default(),
            lower_limit: usize::default(),
            _handle: PhantomData,
        }
    }
}

impl<T> StringValidator<T> {
    pub fn set_upper_limit(mut self, limit: usize) -> Self {
        self.upper_limit = limit;
        self
    }

    pub fn set_lower_limit(mut self, limit: usize) -> Self {
        self.lower_limit = limit;
        self
    }

    pub fn validation<A: Into<String>>(&self, value: A) -> Result<String, ValidationError> {
        let validate = |v: String| {
            let name = std::any::type_name::<T>()
                .split("::")
                .last()
                .unwrap_or("Some value")
                .to_string();

            let v = v.trim();
            if v.is_empty() {
                return Err(ValidationError::Empty { name });
            }
            match v.chars().count() {
                // 下限未満
                c if c < self.lower_limit => Err(ValidationError::UnderLowerLimit {
                    name,
                    input: c,
                    limit: self.lower_limit,
                }),
                // 上限より大きい
                c if self.upper_limit < c => Err(ValidationError::OverUpperLimit {
                    name,
                    input: c,
                    limit: self.upper_limit,
                }),
                _ => Ok(v.into()),
            }
        };
        validate(value.into())
    }
}

#[derive(Debug)]
pub enum ValidationError {
    Empty {
        name: String,
    },
    UnderLowerLimit {
        name: String,
        input: usize,
        limit: usize,
    },
    OverUpperLimit {
        name: String,
        input: usize,
        limit: usize,
    },
}
