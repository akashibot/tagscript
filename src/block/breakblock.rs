use super::prelude::*;

#[derive(Debug)]
pub struct BreakBlock;

impl Block for BreakBlock {
    fn will_accept(&self, ctx: &Context) -> bool {
        let dec = ctx.verb.declaration.as_ref().unwrap();
        ["break", "short", "shortcircuit"].contains(&dec.to_ascii_lowercase().as_str())
    }

    fn process(&self, ctx: &mut Context) -> Result<Option<String>> {
        if let Some(parameter) = &ctx.verb.parameter {
            if let Some(condition) = helper_parse_if(parameter.clone()) {
                if condition {
                    ctx.response.lock().unwrap().body = Some(match &ctx.verb.parameter {
                        Some(parameter) => parameter.clone(),
                        None => String::from(""),
                    });
                }
            }
        }
        Ok(Some(String::from("")))
    }
}
