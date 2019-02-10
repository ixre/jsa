use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use std::sync::Mutex;

use sha1::Sha1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub pwd: String,
    pub flag: usize,
    pub email: String,
    pub api_tokens: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserToml {
    user: Vec<User>
}

lazy_static! {
    static ref USERS: Arc<Mutex<HashMap<String,User>>> = Arc::new(Mutex::new(users_init()));
}

/// Generate user's pwd. It's use SHA1 algorithm
fn user_pwd<P: Into<String>>(p: P) -> String {
    let mut s = Sha1::new();
    s.update(p.into().as_bytes());
    s.digest().to_string()
}

// If users.toml not exists, create the default user.
fn flush_defaults(_p: &str) -> UserToml {
    let mut u = UserToml {
        user: vec![]
    };
    u.user.push(User {
        name: "admin".to_string(),
        pwd: user_pwd("123456"),
        flag: 0,
        email: "".to_string(),
        api_tokens: vec![],
    });
    u
}

fn users_init() -> HashMap<String, User> {
    let mut file_path = super::CONF_PATH.lock().unwrap().clone();
    file_path.push_str("/users.toml");
    let mut file = match File::open(&file_path) {
        Ok(f) => f,
        Err(err) => {
            let _u = flush_defaults(&file_path);
            panic!("Open {} except:{}", &file_path, err);
        }
    };
    let mut buf: String = String::new();
    match file.read_to_string(&mut buf) {
        Ok(s) => s,
        Err(err) => panic!("Read {} except:{}", &file_path, err)
    };
    let mut users = HashMap::new();
    let dst: Result<UserToml, toml::de::Error> = toml::from_str(&buf);
    match dst {
        Ok(arr) => arr.user.into_iter().for_each(|u| { users.insert(u.name.clone(), u); }),
        Err(err) => panic!("Deserialize users except:{}", err)
    };
    return users;
}

pub fn get_users() -> Vec<User> {
    let lock = USERS.lock().unwrap();
    let s: Vec<User> = lock.iter().map(|(_, v)| v.clone()).collect();
    s
}

#[test]
fn test_load_users() {
    let v = get_users();
    println!("{:#?}", v);
}

#[test]
fn test_get_users() {
    let clone = (USERS.clone(), USERS.clone());
    {
        let mut map0 = clone.0.lock().unwrap();
        map0.insert("tom".to_string(), User {
            name: "tom".to_string(),
            pwd: "".to_string(),
            flag: 0,
            email: "".to_string(),
            api_tokens: vec![],
        });
    }
    //assert_eq!(get_users()[0].name, "tom".to_owned());
    {
        let mut map1 = clone.1.lock().unwrap();
        map1.insert("william".to_string(), User {
            name: "tom".to_string(),
            pwd: "".to_string(),
            flag: 0,
            email: "".to_string(),
            api_tokens: vec![],
        });
        map1.insert("jack".to_string(), User {
            name: "tom".to_string(),
            pwd: "".to_string(),
            flag: 0,
            email: "".to_string(),
            api_tokens: vec![],
        });
    }
    assert_eq!(get_users().len(), 3);
}