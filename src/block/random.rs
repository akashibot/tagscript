use super::prelude::*;
use fastrand;

#[derive(Debug)]
pub struct RandomBlock;

impl Block for RandomBlock {
    fn will_accept(&self, ctx: &Context) -> bool {
        let dec = ctx.verb.declaration.as_ref().unwrap();
        ["random", "rand", "#"].contains(&dec.to_ascii_lowercase().as_str())
    }

    fn process(&self, ctx: &mut Context) -> Result<Option<String>> {
        if let Some(payload) = ctx.verb.payload.clone() {
            match helper_split(payload, true) {
                Some(opts) => {
                    let index = fastrand::usize(..opts.len());
                    Ok(Some(opts[index].to_string()))
                }
                None => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}