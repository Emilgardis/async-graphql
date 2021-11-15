use crate::{InputType, InputValueError};

pub fn max_length<T: AsRef<str> + InputType>(
    value: &T,
    len: usize,
) -> Result<(), InputValueError<T>> {
    if value.as_ref().len() <= len {
        Ok(())
    } else {
        Err(format!(
            "the string length is {}, must be less than or equal to {}",
            value.as_ref().len(),
            len
        )
        .into())
    }
}
