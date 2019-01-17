#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::super::hash_session;
    use super::super::HashSessionStore;
    use super::super::SessionPair;
    use super::super::SessionStore;
    #[test]
    fn test_session() {
        // let hs: HashSessionStore<String> = HashSessionStore::new();
        let hs2 = hash_session();
        let mut map = HashMap::new();
        map.insert("id".to_string(), "1".to_string());
        hs2.set(&"123".into(), map);
        assert_eq!(1, 1);
    }
}
