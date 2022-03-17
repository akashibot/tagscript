use tagscript_rs::Verb;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        let parsed = Verb::new(&String::from("{hello:world}"));
        assert_eq!(parsed.declaration, Some("hello".to_string()));
        assert_eq!(parsed.parameter, None);
        assert_eq!(parsed.payload, Some("world".to_string()));

        let bare = Verb::new(&"{user}".to_string());
        assert_eq!(bare.declaration, Some("user".to_string()));
        assert_eq!(bare.parameter, None);
        assert_eq!(bare.payload, None);

        let bare = Verb::new(&"{user(hello)}".to_string());
        assert_eq!(bare.declaration, Some("user".to_string()));
        assert_eq!(bare.parameter, Some("hello".to_string()));
        assert_eq!(bare.payload, None);
    }
}
