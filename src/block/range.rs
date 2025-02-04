use super::prelude::*;
use fastrand;

#[derive(Debug)]
pub struct RangeBlock;

impl Block for RangeBlock {
    fn will_accept(&self, ctx: &Context) -> bool {
        let dec = ctx.verb.declaration.as_ref().unwrap();
        ["rangef", "range"].contains(&dec.to_ascii_lowercase().as_str())
    }

    fn process(&self, ctx: &mut Context) -> Result<Option<String>> {
        if let Some(payload) = ctx.verb.payload.as_ref() {
            let parts = payload.split("-").collect::<Vec<_>>();

            match ctx.verb.declaration.as_ref().map(|p| p.as_str()) {
                Some("rangef") => {
                    if parts.len() == 2 {
                        let start = parts[0].parse::<f32>()?;
                        let end = parts[1].parse::<f32>()?;
                        Ok(Some(format!(
                            "{}",
                            fastrand::f32() * (end - start) + start
                        )))
                    } else {
                        Ok(None)
                    }
                }
                Some("range") => {
                    if parts.len() == 2 {
                        let start = parts[0].parse::<i32>()?;
                        let end = parts[1].parse::<i32>()?;
                        Ok(Some(format!(
                            "{}",
                            fastrand::i32(start..end)
                        )))
                    } else {
                        Ok(None)
                    }
                }
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}
