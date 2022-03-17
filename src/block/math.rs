use super::prelude::*;

pub struct MathBlock;

impl Block for MathBlock {
    fn will_accept(&self, ctx: &Context) -> bool {
        let dec = ctx.verb.declaration.as_ref().unwrap().to_lowercase();
        ["math", "m", "+", "calc"].contains(&dec.as_str())
    }

    fn process(&self, ctx: &mut Context) -> Result<Option<String>> {
        if let Some(payload) = ctx.verb.payload.as_ref() {
            Ok(match meval::eval_str(payload) {
                Ok(value) => Some(value.to_string()),
                Err(_) => None,
            })
        } else {
            Ok(None)
        }
    }
}
