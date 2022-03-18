use crate::{interpreter::Context, Result};

pub trait Block: std::fmt::Debug + Send + Sync {
    fn will_accept(&self, ctx: &Context) -> bool;

    fn process(&self, ctx: &mut Context) -> Result<Option<String>>;
}
