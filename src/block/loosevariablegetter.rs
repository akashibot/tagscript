use super::prelude::*;

pub struct LooseVariableGetterBlock;

impl Block for LooseVariableGetterBlock {
    fn will_accept(&self, _ctx: &Context) -> bool {
        true
    }

    fn process(&self, ctx: &mut Context) -> Result<Option<String>> {
        let response = ctx.response.lock().unwrap();
        if let Some(declaration) = ctx.verb.declaration.as_ref() {
            Ok(match response.variables.get(declaration) {
                Some(adapter) => Some(adapter.get_value(ctx.verb.clone())?),
                _ => None,
            })
        } else {
            Ok(None)
        }
    }
}
