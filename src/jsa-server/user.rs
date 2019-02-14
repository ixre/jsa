use std::collections::HashMap;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use sha1::Sha1;

/// User flag
pub enum UserFlag {
    Enabled = 1,
    SuperUser = 2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub pwd: String,
    pub flag: i8,
    pub email: String,
    pub api_tokens: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserToml {
    user: Vec<User>,
}

lazy_static! {
    static ref USERS: Arc<Mutex<HashMap<String, User>>> = Arc::new(Mutex::new(users_init()));
}

fn users_init() -> HashMap<String, User> {
    let file_path = User::path();
    let mut file = match File::open(&file_path) {
        Ok(f) => f,
        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                User::flush_defaults(&file_path);
                File::open(&file_path).unwrap()
            } else {
                panic!("Open {} except:{}", &file_path, err);
            }
        }
    };
    let mut buf: String = String::new();
    match file.read_to_string(&mut buf) {
        Ok(s) => s,
        Err(err) => panic!("Read {} except:{}", &file_path, err),
    };
    let mut users = HashMap::new();
    let dst: Result<UserToml, toml::de::Error> = toml::from_str(&buf);
    match dst {
        Ok(arr) => arr.user.into_iter().for_each(|u| {
            users.insert(u.name.clone(), u);
        }),
        Err(err) => panic!("Deserialize users except:{}", err),
    };
    return users;
}

impl User {
    /// Generate user's pwd. It's use SHA1 algorithm
    pub fn pwd<P: Into<String>>(p: P) -> String {
        let mut s = Sha1::new();
        s.update(p.into().as_bytes());
        s.digest().to_string()
    }

    // If users.toml not exists, create the default user.
    fn flush_defaults(p: &str) -> UserToml {
        let mut u = UserToml { user: vec![] };
        u.user.push(User {
            name: "admin".to_string(),
            pwd: Self::pwd("123456"),
            flag: UserFlag::Enabled as i8 | UserFlag::SuperUser as i8,
            email: "".to_string(),
            api_tokens: vec![],
        });
        Self::flush2_file(p, &u);
        u
    }

    // Save user's settings to file
    fn flush2_file(path: &str, u: &UserToml) {
        match toml::to_string_pretty(&u) {
            Ok(s) => match File::create(path) {
                Ok(mut fi) => {
                    if let Err(err) = fi.write(s.as_bytes()) {
                        panic!("Save users config file except: {}", err);
                    }
                }
                Err(err) => panic!("Convert users to string :{}", err),
            },
            Err(err) => panic!("Convert users to string :{}", err),
        }
    }

    fn flush_users() {
        let clone = USERS.clone();
        thread::spawn(move || {
            let lock = clone.lock().unwrap();
            let users: Vec<User> = lock.iter().map(|(_, v)| v.clone()).collect();
            Self::flush2_file(&Self::path(), &UserToml { user: users });
        });
    }

    fn path() -> String {
        let mut file_path = super::CONF_PATH.lock().unwrap().clone();
        file_path.push_str("/users.toml");
        file_path
    }

    pub fn get_users() -> Vec<User> {
        let lock = USERS.lock().unwrap();
        let s: Vec<User> = lock.iter().map(|(_, v)| v.clone()).collect();
        s
    }

    pub fn take_users(begin: usize, over: usize) -> (usize, Vec<User>) {
        let mut rows = Self::get_users();
        rows.sort_by(|a, b| a.name.cmp(&b.name));
        let len = rows.len();
        (len, rows[begin..over.min(len)].to_vec())
    }

    pub fn get_user(user: &str) -> Option<User> {
        let lock = USERS.lock().unwrap();
        if !lock.contains_key(user) {
            return None;
        }
        Some(lock.get(user).unwrap().clone())
    }

    pub fn save_user(user: &User) -> Result<usize, &str> {
        let clone = USERS.clone();
        let mut lock = clone.lock().unwrap();
        if lock.contains_key(&user.name) {
            lock.insert(user.name.clone(), user.clone());
            Self::flush_users();
            return Ok(1);
        }
        Err("no such user")
    }
}

#[test]
fn test_load_users() {
    let v = User::get_users();
    println!("{:#?}", v);
    if let Some(mut user) = User::get_user("admin") {
        //assert_eq!(1,3);
        user.email = "jarrysix@gmail.com".to_string();
        User::save_user(&user);
    }
    thread::sleep(std::time::Duration::from_secs(3));
}

#[test]
fn test_get_users() {
    let clone = (USERS.clone(), USERS.clone());
    {
        let mut map0 = clone.0.lock().unwrap();
        map0.insert(
            "tom".to_string(),
            User {
                name: "tom".to_string(),
                pwd: "".to_string(),
                flag: 0,
                email: "".to_string(),
                api_tokens: vec![],
            },
        );
    }
    //assert_eq!(get_users()[0].name, "tom".to_owned());
    {
        let mut map1 = clone.1.lock().unwrap();
        map1.insert(
            "william".to_string(),
            User {
                name: "tom".to_string(),
                pwd: "".to_string(),
                flag: 0,
                email: "".to_string(),
                api_tokens: vec![],
            },
        );
        map1.insert(
            "jack".to_string(),
            User {
                name: "tom".to_string(),
                pwd: "".to_string(),
                flag: 0,
                email: "".to_string(),
                api_tokens: vec![],
            },
        );
    }
    assert_eq!(User::get_users().len(), 3);
}
