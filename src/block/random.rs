use super::prelude::*;

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
                Some(opts) => Ok(Some(opts[rand::random::<usize>() % opts.len()].to_string())),
                None => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}
