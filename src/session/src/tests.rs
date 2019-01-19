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
        let session_id = "1".into();
        // Create session storage
        let hs2 = hash_session();
        // Insert new session
        let mut map = HashMap::new();
        map.insert("user_id".to_string(), "1".to_string());
        let mut map_mut = map.clone();
        hs2.set(&session_id, map);
        // Update existed session
        map_mut.insert("user_name".to_string(),"jarrysix".to_string());
        hs2.set(&session_id, map_mut);
        // Get session
        let map :HashMap<String,String> = hs2.get(&session_id).unwrap();
        println!("{:#?}",map);
        assert_eq!(1, 1);
    }
}
