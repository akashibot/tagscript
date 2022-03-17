use crate::{verb::Verb, Result};
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub enum Adapter {
    String(String),
    Int(i32),
    Function(fn(&Verb) -> Result<String>),
}

impl Adapter {
    pub fn get_value(&self, ctx: Verb) -> Result<String> {
        Ok(match self {
            Adapter::String(string) => {
                if let Some(parameter) = ctx.parameter {
                    if !parameter.contains("+") {
                        let index = match parameter.parse::<usize>() {
                            Ok(index) if index > 0 => index - 1,
                            _ => return Ok(string.clone()),
                        };
                        let splitter = ctx.payload.unwrap_or_else(|| String::from(" "));
                        string
                            .split(splitter.as_str())
                            .nth(index)
                            .unwrap_or(string.as_ref())
                            .to_string()
                    } else {
                        let index = match parameter.replace("+", "").parse::<usize>() {
                            Ok(index) => index - 1,
                            Err(_) => return Ok(string.clone()),
                        };
                        let splitter = ctx.payload.unwrap_or_else(|| String::from(" "));
                        if parameter.starts_with("+") {
                            string
                                .split(splitter.as_str())
                                .enumerate()
                                .take_while(|(i, _)| *i <= index)
                                .map(|(_, s)| s)
                                .collect::<Vec<&str>>()
                                .join(splitter.as_str())
                        } else if parameter.ends_with("+") {
                            string
                                .split(splitter.as_str())
                                .skip(index)
                                .collect::<Vec<&str>>()
                                .join(splitter.as_str())
                        } else {
                            string
                                .split(splitter.as_str())
                                .nth(index)
                                .unwrap_or(string.as_ref())
                                .to_string()
                        }
                    }
                } else {
                    string.clone()
                }
            }
            Adapter::Int(number) => number.to_string(),
            Adapter::Function(function) => function(&ctx)?,
        })
    }
}

impl Debug for Adapter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Adapter::String(string) => write!(f, "{}", string),
            Adapter::Int(number) => write!(f, "{}", number),
            Adapter::Function(_) => write!(f, "Function"),
        }
    }
}
