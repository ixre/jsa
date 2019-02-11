extern crate time;

use std::collections::HashMap;

use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::json::JsonValue;
use serde_json::Map;

use crate::{User, UserFlag};
use crate::http::Context;
use crate::http::WrappedResult;

use super::super::{NAME, VERSION};

#[get("/")]
pub fn index() -> Redirect {
    Redirect::temporary("/console/")
}

#[get("/index2")]
pub fn index2() -> &'static str {
    //let mut s = "Hello, world!".to_string();
    //s.push_str(req.headers().get_one("Host").unwrap());
    //s.as_str();
    "hello world"
}

#[get("/login")]
pub fn login2() -> JsonValue {
    json!({"hello":"123"})
}

#[derive(FromForm, Debug)]
pub struct LoginParams {
    user: String,
    pwd: String,
}

#[post("/login", data = "<user>")]
pub fn login(mut cookies: Cookies, user: Form<LoginParams>) -> WrappedResult {
    if let Some(u) = User::get_user(&user.user) {
        if user.pwd != u.pwd {
            return WrappedResult::new(2, "密码不正确", "");
        }
        let flags = (UserFlag::Enabled as i8, UserFlag::SuperUser as i8);
        if u.flag & flags.0 != flags.0 {
            return WrappedResult::new(2, "用户已停用", "");
        }
        let is_super = u.flag & flags.1 == flags.1;
        // Save to session storage
        let key = session::generate_id();
        let mut map = HashMap::new();
        map.insert("UserID".to_string(), u.name.to_string());
        map.insert("SuperUser".to_string(), if is_super { "1" } else { "0" }.to_string());
        map.insert("NickName".to_string(), u.name.to_string());
        super::flush_session(&key, map);
        // flush to client
        let mut ck_id = Cookie::new("SessionID", key);
        let mut expires = time::now_utc();
        expires.tm_min += 30;
        ck_id.set_expires(expires);
        ck_id.set_path("/console/api");
        let mut map = Map::new();
        map.insert("SessionID".into(), ck_id.value().into());
        map.insert("SuperUser".into(), if is_super { "1" } else { "0" }.into());
        cookies.add(ck_id);
        // return to client
        return WrappedResult::new(0, "", map);
    }
    return WrappedResult::new(1, "用户或密码不正确", "");
}

#[post("/user/logout")]
pub fn logout(mut cookies: Cookies) -> WrappedResult {
    let sid = session_id(&cookies);
    if sid.len() == 0 {
        return WrappedResult::new(1, "logout success", Map::new());
    }
    // Clean session id
    let mut cookie = cookies.get("SessionID").unwrap().to_owned();
    cookie.set_expires(time::empty_tm());
    cookies.remove(cookie);
    // Clean session storage
    super::remove_session(&sid);
    WrappedResult::new(0, "", Map::new())
}

fn session_id(cookies: &Cookies) -> String {
    if let Some(ck) = cookies.get("SessionID") {
        return ck.value().to_string();
    }
    String::from("")
}

#[post("/check_session")]
pub fn check_session(cookies: Cookies) -> JsonValue {
    let sid = session_id(&cookies);
    if sid.len() == 0 {
        return json!({"code":1,"err_msg":"用户未登陆".to_string()});
    }
    if let Some(_) = super::get_session(&sid) {
        return json!({"code":0,"SessionID":sid});
    }
    return json!({"code":2,"err_msg":"会话已过期".to_string()});
}

/// Return initialize data for dashboard
#[post("/initial")]
pub fn initial(ctx: Context) -> JsonValue {
    let mut nick_name = String::new();
    let sid = session_id(&ctx.req.cookies());
    if let Some(d) = super::get_session(&sid) {
        nick_name = d.get("NickName").unwrap().to_string();
    }
    json!({"nick_name":nick_name,
    "sys_name":NAME,
    "version":VERSION})
}
