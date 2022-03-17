use super::prelude::*;

pub struct StrfBlock;

impl Block for StrfBlock {
    fn will_accept(&self, ctx: &Context) -> bool {
        let dec = ctx.verb.declaration.as_ref().unwrap().to_lowercase();
        ["strf"].contains(&dec.as_str())
    }

    fn process(&self, ctx: &mut Context) -> Result<Option<String>> {
        if let Some(parameter) = ctx.verb.parameter.clone() {
            if parameter.parse::<u64>().is_ok() {
                // fuck this
            }
        }
    }
}
