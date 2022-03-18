use super::prelude::*;

#[derive(Debug)]
pub struct StrictVariableGetterBlock;

impl Block for StrictVariableGetterBlock {
    fn will_accept(&self, ctx: &Context) -> bool {
        ctx.response
            .lock()
            .unwrap()
            .variables
            .contains_key(ctx.verb.declaration.as_ref().unwrap())
    }

    fn process(&self, ctx: &mut Context) -> Result<Option<String>> {
        let response = ctx.response.lock().unwrap();
        let variable = ctx.verb.declaration.as_ref().unwrap();
        let adapter = response.variables.get(variable).unwrap();
        Ok(Some(adapter.get_value(ctx.verb.clone())?))
    }
}
