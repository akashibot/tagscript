use super::prelude::*;

pub struct FiftyFiftyBlock;

impl Block for FiftyFiftyBlock {
    fn will_accept(&self, ctx: &Context) -> bool {
        let dec = ctx.verb.declaration.as_ref().unwrap();
        ["50", "5050", "?"].contains(&dec.to_ascii_lowercase().as_str())
    }

    fn process(&self, ctx: &mut Context) -> Result<Option<String>> {
        if let Some(payload) = ctx.verb.payload.clone() {
            Ok(Some(match rand::random::<f32>() < 0.5 {
                true => payload,
                false => String::from(""),
            }))
        } else {
            Ok(None)
        }
    }
}
