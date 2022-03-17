#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use tagscript_rs::{block, Adapter, Block, Interpreter, Result, Verb};

    fn set_up() -> Interpreter {
        let blocks: Vec<Box<dyn Block>> = vec![Box::from(block::StrictVariableGetterBlock {})];
        Interpreter::new(blocks)
    }

    fn dummy_function(_: &Verb) -> Result<String> {
        Ok(String::from("dummy"))
    }

    #[test]
    fn test_string_adapter() {
        let interpreter = set_up();

        let mut seed_variables: HashMap<String, Adapter> = HashMap::new();

        seed_variables.insert(
            "test".to_string(),
            Adapter::String("Hello World, How are you".to_string()),
        );

        let result =
            interpreter.process(String::from("{test}"), Some(seed_variables.clone()), None);

        assert_eq!(
            result.unwrap().body,
            Some("Hello World, How are you".to_string()),
        );

        let result = interpreter.process(
            String::from("{test(1)}"),
            Some(seed_variables.clone()),
            None,
        );

        assert_eq!(result.unwrap().body, Some("Hello".to_string()));

        let result = interpreter.process(
            String::from("{test(3+)}"),
            Some(seed_variables.clone()),
            None,
        );

        assert_eq!(result.unwrap().body, Some("How are you".to_string()));

        let result = interpreter.process(
            String::from("{test(+2)}"),
            Some(seed_variables.clone()),
            None,
        );

        assert_eq!(result.unwrap().body, Some("Hello World,".to_string()));
    }

    #[test]
    fn test_function_adapter() {
        let interpreter = set_up();

        let mut seed_variables: HashMap<String, Adapter> = HashMap::new();

        seed_variables.insert("test".to_string(), Adapter::Function(dummy_function));

        let result =
            interpreter.process(String::from("{test}"), Some(seed_variables.clone()), None);

        assert_eq!(result.unwrap().body, Some("dummy".to_string()));
    }
}
