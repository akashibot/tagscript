use super::prelude::*;

fn parse_into_output(payload: String, result: Option<bool>) -> Option<String> {
    if let Some(result) = result {
        match helper_split(payload.clone(), false) {
            Some(output) if output.len() == 2 => {
                if result {
                    return Some(output[0].clone());
                } else {
                    return Some(output[1].clone());
                }
            }
            _ => {
                if result {
                    return Some(payload);
                } else {
                    return Some(String::from(""));
                }
            }
        }
    }
    None
}

#[derive(Debug)]
pub struct AnyBlock;

impl Block for AnyBlock {
    fn will_accept(&self, ctx: &Context) -> bool {
        let dec = ctx.verb.declaration.clone().unwrap().to_lowercase();
        ["any", "or"].contains(&dec.as_str())
    }

    fn process(&self, ctx: &mut Context) -> Result<Option<String>> {
        Ok(
            if let (Some(payload), Some(parameter)) =
                (ctx.verb.payload.clone(), ctx.verb.parameter.clone())
            {
                let result = helper_parse_list_if(parameter.clone())
                    .into_iter()
                    .any(|i| i.unwrap_or(false));

                parse_into_output(payload, Some(result))
            } else {
                None
            },
        )
    }
}

#[derive(Debug)]
pub struct AllBlock;

impl Block for AllBlock {
    fn will_accept(&self, ctx: &Context) -> bool {
        let dec = ctx.verb.declaration.clone().unwrap().to_lowercase();
        ["all", "and"].contains(&dec.as_str())
    }

    fn process(&self, ctx: &mut Context) -> Result<Option<String>> {
        Ok(
            if let (Some(payload), Some(parameter)) =
                (ctx.verb.payload.clone(), ctx.verb.parameter.clone())
            {
                let result = helper_parse_list_if(parameter.clone())
                    .into_iter()
                    .all(|i| i.unwrap_or(false));

                parse_into_output(payload, Some(result))
            } else {
                None
            },
        )
    }
}

#[derive(Debug)]
pub struct IfBlock;

impl Block for IfBlock {
    fn will_accept(&self, ctx: &Context) -> bool {
        let dec = ctx.verb.declaration.clone().unwrap().to_lowercase();
        ["if"].contains(&dec.as_str())
    }

    fn process(&self, ctx: &mut Context) -> Result<Option<String>> {
        Ok(
            if let (Some(payload), Some(parameter)) =
                (ctx.verb.payload.clone(), ctx.verb.parameter.clone())
            {
                let result = helper_parse_if(parameter.clone());

                parse_into_output(payload, result)
            } else {
                None
            },
        )
    }
}
