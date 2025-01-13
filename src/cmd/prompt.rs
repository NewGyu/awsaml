use anyhow::{Error, Result};
use std::{
    io::{self, Write},
    str::FromStr,
};

pub struct Prompt {
    pub msg: String,
    pub required: Required,
}

pub enum Required {
    Yes(Option<String>),
    No,
}

impl Prompt {
    pub fn get_value<T>(&self) -> Result<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    {
        let input = self.read_input()?;
        T::from_str(&input).map_err(|e| Error::new(e))
    }

    fn read_input(&self) -> Result<String> {
        self.print_input_msg();
        // Read input from the user
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input = input.trim().to_string();

        if input.is_empty() {
            match &self.required {
                Required::Yes(default) => {
                    if let Some(default) = default {
                        Ok(default.clone())
                    } else {
                        Err(Error::msg("Input is required"))
                    }
                }
                Required::No => return Ok(input),
            }
        } else {
            Ok(input)
        }
    }

    fn print_input_msg(&self) {
        print!(
            "{} ({}) >",
            self.msg,
            match &self.required {
                Required::Yes(default) => match default {
                    Some(ref default) => format!("default: {}", default),
                    None => "required".to_string(),
                },
                Required::No => "optional".to_string(),
            }
        );
        io::stdout().flush().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::prompt::{Prompt, Required};

    #[test]
    fn test_prompt() {
        assert_eq!(
            Prompt {
                msg: "Enter a number".to_string(),
                required: Required::Yes(None),
            }
            .get_value::<i32>()
            .unwrap(),
            123
        );
    }
}
