use super::prelude::*;
use crate::Verb;

#[derive(Debug)]
pub struct ShortCutRedirectBlock {
    pub redirect_name: String,
}

impl Block for ShortCutRedirectBlock {
    fn will_accept(&self, ctx: &Context) -> bool {
        let dec = ctx.verb.declaration.as_ref().unwrap();
        dec.parse::<u64>().is_ok()
    }

    fn process(&self, ctx: &mut Context) -> Result<Option<String>> {
        let blank = Verb::new(&format!(
            "{{{}:{}}}",
            self.redirect_name,
            ctx.verb.payload.as_ref().unwrap()
        ));
        ctx.verb = blank;
        Ok(None)
    }
}

impl Default for ShortCutRedirectBlock {
    fn default() -> Self {
        ShortCutRedirectBlock {
            redirect_name: String::from("args"),
        }
    }
}
