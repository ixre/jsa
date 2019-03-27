extern crate time;

use std::collections::HashMap;

use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket_contrib::json::JsonValue;
use serde_json::Map;

use crate::http::Context;
use crate::http::WrappedResult;
use crate::http::{flush_session, get_session, remove_session};
use crate::{User, UserFlag};
use crate::{NAME, VERSION};

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
        let super_str = if is_super { "1" } else { "0" }.to_string();
        // Save to session storage
        let key = session::generate_id();
        let mut map = HashMap::new();
        map.insert("UserID".to_string(), u.user.to_string());
        map.insert("SuperUser".to_string(), super_str.clone());
        map.insert("NickName".to_string(), u.user.to_string());
        flush_session(&key, map);
        // flush to client
        let mut ck_id = Cookie::new("SessionID", key);
        let mut expires = time::now_utc();
        expires.tm_min += 30;
        ck_id.set_expires(expires);
        ck_id.set_path("/console/api");
        let mut map = Map::new();
        map.insert("SessionID".into(), ck_id.value().into());
        map.insert("SuperUser".into(), super_str.into());
        cookies.add(ck_id);
        // return to client
        return WrappedResult::new(0, "", map);
    }
    return WrappedResult::new(1, "用户或密码不正确", "");
}

#[post("/logout")]
pub fn logout(mut cookies: Cookies) -> WrappedResult {
    let sid = session_id(&cookies);
    if sid.len() == 0 {
        return WrappedResult::new(1, "logout success", Map::new());
    }
    // Clean session id
    let mut cookie = cookies.get("SessionID").unwrap().to_owned();
    cookie.set_expires(time::empty_tm());
    cookie.set_value(sid.clone());
    cookies.remove(cookie);
    // Clean session storage
    remove_session(&sid);
    WrappedResult::new(0, "logout success", Map::new())
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
    if let Some(_) = get_session(&sid) {
        return json!({"code":0,"SessionID":sid});
    }
    return json!({"code":2,"err_msg":"会话已过期".to_string()});
}

/// Return initialize data for dashboard
#[post("/initial")]
pub fn initial(ctx: Context) -> JsonValue {
    let sid = session_id(&ctx.req.cookies());
    match get_session(&sid) {
        Some(d) => {
            let user_id = d.get("UserID").unwrap().to_string();
            return match User::get_user(&user_id) {
                Some(u) => json!({"sys_name":NAME,
                "version":VERSION,
                "user":{
                    "user":u.user,
                    "name":u.name,
                    "nick_name":u.name,
                    "flag":u.flag,
                    "email":u.email,
                    "api_enabled":u.api_tokens.len() > 0
                }}),
                None => json!({"code":2,"err_msg":"user not exists"}),
            };
        }
        None => json!({"code":1,"err_msg":"access denied"}),
    }
}
