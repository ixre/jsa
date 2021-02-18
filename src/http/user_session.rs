use rocket::http::Cookies;
use rocket_contrib::json::JsonValue;

use crate::http::get_session;
use crate::repo::UserRepo;
use crate::{conn, User};

pub fn session_id(cookies: &Cookies) -> String {
    if let Some(ck) = cookies.get("SessionID") {
        return ck.value().to_string();
    }
    String::from("")
}

pub fn check_access(cookie: &Cookies, uid: i32) -> bool {
    let sid = session_id(cookie);
    match get_session(&sid) {
        Some(d) => {
            let zero = &String::from("0");
            let user_id = d.get("UserId").unwrap_or(zero).parse().unwrap_or(0);
            user_id == uid || d.get("SuperUser").unwrap_or(zero).eq("1")
        }
        _ => false,
    }
}

pub fn access_denied() -> JsonValue {
    json!({"err_code":-100,"err_msg":"access denied"})
}

pub fn uid(cookie: &Cookies) -> Option<i32> {
    let sid = session_id(cookie);
    match get_session(&sid) {
        Some(d) => Some(
            d.get("UserId")
                .unwrap_or(&String::from("0"))
                .parse()
                .unwrap_or(0),
        ),
        _ => None,
    }
}

pub fn session_user(cookie: &Cookies) -> Option<User> {
    let sid = session_id(cookie);
    match get_session(&sid) {
        Some(d) => {
            let user_id = d
                .get("UserId")
                .unwrap_or(&String::from("0"))
                .parse()
                .unwrap();
            UserRepo::get(&conn(), user_id)
        }
        _ => None,
    }
}
