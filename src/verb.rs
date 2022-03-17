#[derive(Debug, Clone)]
pub struct Verb {
    pub declaration: Option<String>,
    pub parameter: Option<String>,
    pub payload: Option<String>,
}

impl Verb {
    pub fn new(verb_string: &String) -> Self {
        let len = verb_string.len();
        let parsed_string = &verb_string[1..len - 1];

        let bytes = parsed_string.as_bytes();

        let mut dec_depth = 0;
        let mut dec_start = 0;

        let mut declaration = None;
        let mut payload = None;
        let mut parameter = None;

        for (i, &item) in bytes.iter().enumerate() {
            if item == b":"[0] && dec_depth == 0 {
                let res: Vec<&str> = parsed_string.splitn(2, ":").collect();
                let payload = if res.len() == 2 {
                    Some(res[1].to_string())
                } else {
                    None
                };

                return Self {
                    payload,
                    declaration: Some(res[0].to_string()),
                    parameter: None,
                };
            } else if item == b"("[0] {
                dec_depth += 1;
                if dec_start == 0 {
                    dec_start = i;
                    declaration = Some(parsed_string[..i].to_string());
                }
            } else if item == b")"[0] && dec_depth != 0 {
                dec_depth -= 1;
                if dec_depth == 0 {
                    parameter = Some(parsed_string[dec_start + 1..i].to_string());
                }
                if parsed_string.len() >= i + 2 {
                    if &parsed_string[i + 1..i + 2] == ":" {
                        payload = Some(parsed_string[i + 2..].to_string());
                    }
                }
                return Verb {
                    declaration,
                    parameter,
                    payload,
                };
            }
        }
        let res = parsed_string.splitn(2, ":").collect::<Vec<&str>>();
        Verb {
            declaration: Some(res[0].to_string()),
            parameter: None,
            payload: if res.len() == 2 {
                Some(res[1].to_string())
            } else {
                None
            },
        }
    }
}

impl ToString for Verb {
    fn to_string(&self) -> String {
        let mut res = String::from("{");

        if let Some(declaration) = &self.declaration {
            res.push_str(declaration);
        }
        if let Some(parameter) = &self.parameter {
            res.push_str(format!("({})", parameter).as_str());
        }
        if let Some(payload) = &self.payload {
            res.push_str(":");
            res.push_str(payload);
        }
        res.push_str("}");
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verb_new() {
        let verb_string = String::from("{verb(parameter):payload}");
        let verb = Verb::new(&verb_string);
        assert_eq!(verb.declaration, Some("verb".to_string()));
        assert_eq!(verb.parameter, Some("parameter".to_string()));
        assert_eq!(verb.payload, Some("payload".to_string()));
    }

    #[test]
    fn test_verb_to_string() {
        let verb_string = String::from("{verb(parameter):payload}");
        let verb = Verb::new(&verb_string);
        assert_eq!(verb.to_string(), verb_string);
    }

    #[test]
    fn test_verb_new_with_no_parameter() {
        let verb_string = String::from("{verb:payload}");
        let verb = Verb::new(&verb_string);
        assert_eq!(verb.declaration, Some("verb".to_string()));
        assert_eq!(verb.parameter, None);
        assert_eq!(verb.payload, Some("payload".to_string()));
    }

    #[test]
    fn test_verb_new_with_no_payload() {
        let verb_string = String::from("{verb(parameter)}");
        let verb = Verb::new(&verb_string);
        assert_eq!(verb.declaration, Some("verb".to_string()));
        assert_eq!(verb.parameter, Some("parameter".to_string()));
        assert_eq!(verb.payload, None);
    }

    #[test]
    fn test_only_verb() {
        let verb_string = String::from("{verb}");
        let verb = Verb::new(&verb_string);
        assert_eq!(verb.declaration, Some("verb".to_string()));
        assert_eq!(verb.parameter, None);
        assert_eq!(verb.payload, None);
    }
}
