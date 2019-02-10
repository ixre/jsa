use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub pwd: Option<String>,
    pub flag: usize,
    pub email: Option<String>,
    pub api_tokens: Vec<String>,
}

lazy_static! {
    static ref USERS: Arc<Mutex<HashMap<String,User>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub fn get_users() -> Vec<User> {
    let lock = USERS.lock().unwrap();
    let s: Vec<User> = lock.iter().map(|(_, v)| v.clone()).collect();
    s
}


#[test]
fn test_get_users() {
    let clone = (USERS.clone(),USERS.clone());
    {
        let mut map0 = clone.0.lock().unwrap();
        map0.insert("tom".to_string(), User {
            name: "tom".to_string(),
            pwd: None,
            flag: 0,
            email: None,
            api_tokens: vec![],
        });
    }
    assert_eq!(get_users()[0].name, "tom".to_owned());
    {
        let mut map1 = clone.1.lock().unwrap();
        map1.insert("william".to_string(), User {
            name: "tom".to_string(),
            pwd: None,
            flag: 0,
            email: None,
            api_tokens: vec![],
        });
        map1.insert("jack".to_string(), User {
            name: "tom".to_string(),
            pwd: None,
            flag: 0,
            email: None,
            api_tokens: vec![],
        });
    }
    assert_eq!(get_users().len(),3);
}