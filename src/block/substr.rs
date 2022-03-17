use super::prelude::*;

struct SubstringBlock;

impl Block for SubstringBlock {
    fn will_accept(&self, ctx: &Context) -> bool {
        let dec = ctx.verb.declaration.as_ref().unwrap().to_lowercase();
        ["substr", "substring"].contains(&dec.as_str())
    }

    fn process(&self, ctx: &mut Context) -> Result<Option<String>> {
        if let Some(parameter) = &ctx.verb.parameter {
            if let Some(payload) = ctx.verb.payload.as_ref() {
                if parameter.contains("-") {}
            }
        }
        Ok(None)
    }
}
