use super::prelude::*;

#[derive(Debug)]
pub struct StopBlock;

impl Block for StopBlock {
    fn will_accept(&self, ctx: &Context) -> bool {
        let dec = ctx.verb.declaration.as_ref().unwrap().to_lowercase();
        ["stop", "halt"].contains(&dec.as_str())
    }

    fn process(&self, ctx: &mut Context) -> Result<Option<String>> {
        if let Some(parameter) = ctx.verb.parameter.clone() {
            if let Some(v) = helper_parse_if(parameter) {
                if v {
                    ctx.response
                        .lock()
                        .unwrap()
                        .actions
                        .insert("TSE_STOP".to_string(), Action::Stop(true));
                }
            }
            match ctx.verb.payload.clone() {
                Some(payload) => Ok(Some(payload)),
                None => Ok(Some(String::from(""))),
            }
        } else {
            Ok(Some(String::from("")))
        }
    }
}
