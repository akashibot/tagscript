use super::prelude::*;

pub struct AssignmentBlock;

impl Block for AssignmentBlock {
    fn will_accept(&self, ctx: &Context) -> bool {
        let dec = ctx.verb.declaration.as_ref().unwrap();
        ["=", "assign", "let", "var"].contains(&dec.to_ascii_lowercase().as_str())
    }

    fn process(&self, ctx: &mut Context) -> Result<Option<String>> {
        if let Some(parameter) = &ctx.verb.parameter {
            ctx.response.lock().unwrap().variables.insert(
                parameter.clone(),
                Adapter::String(
                    ctx.verb
                        .payload
                        .as_ref()
                        .unwrap_or(&String::from(""))
                        .clone(),
                ),
            );
            Ok(Some(String::from("")))
        } else {
            Ok(None)
        }
    }
}
