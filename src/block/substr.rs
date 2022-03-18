use super::prelude::*;

pub struct SubstringBlock;

impl Block for SubstringBlock {
    fn will_accept(&self, ctx: &Context) -> bool {
        let dec = ctx.verb.declaration.as_ref().unwrap().to_lowercase();
        ["substr", "substring"].contains(&dec.as_str())
    }

    fn process(&self, ctx: &mut Context) -> Result<Option<String>> {
        if let Some(parameter) = &ctx.verb.parameter {
            if let Some(payload) = ctx.verb.payload.as_ref() {
                if parameter.contains("-") {
                    let mut parts = parameter.split("-");
                    let start = parts.next().unwrap().parse::<usize>()?;
                    let end = parts.next().unwrap().parse::<usize>()?;
                    let mut result = String::new();
                    for (i, c) in payload.chars().enumerate() {
                        if i >= start && i <= end {
                            result.push(c);
                        }
                    }
                    return Ok(Some(result));
                } else {
                    let start = parameter.parse::<usize>()?;
                    let mut result = String::new();
                    for (i, c) in payload.chars().enumerate() {
                        if i >= start {
                            result.push(c);
                        }
                    }
                    return Ok(Some(result));
                }
            }
        }
        Ok(None)
    }
}
