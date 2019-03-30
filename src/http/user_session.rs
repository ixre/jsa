use crate::{User, conn};
use crate::http::get_session;
use crate::repo::UserRepo;
use rocket::http::Cookies;

pub fn session_id(cookies: &Cookies) -> String {
    if let Some(ck) = cookies.get("SessionID") {
        return ck.value().to_string();
    }
    String::from("")
}

pub fn session_user(cookie:&Cookies)->Option<User>{
    let sid = session_id(cookie);
    match get_session(&sid) {
        Some(d) => {
            let user_id = d.get("UserId")
                .unwrap_or(&String::from("0"))
                .parse().unwrap();
            UserRepo::get(&conn(), user_id)
        }
        _ => None
    }
}