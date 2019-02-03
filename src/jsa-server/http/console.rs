extern crate time;

use std::collections::HashMap;

use rocket::http::{Cookie, Cookies};
use rocket::response::Redirect;
use rocket_contrib::json::JsonValue;
use serde_json::Map;

use crate::http::Context;
use crate::http::WrappedResult;

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

#[post("/login")]
pub fn login(mut cookies: Cookies) -> WrappedResult {
    let key = session::generate_id();
    // Save to session storage
    let mut session_map = HashMap::new();
    session_map.insert("is_admin".to_string(), "1".to_string());
    session_map.insert("nick_name".to_string(), "admin".to_string());
    super::flush_session(&key, session_map);
    // flush to client
    let mut ck_id = Cookie::new("SessionID", key);
    let mut expires = time::now_utc();
    expires.tm_min += 30;
    ck_id.set_expires(expires);
    ck_id.set_path("/console/api");
    let mut cli_map = Map::new();
    cli_map.insert("SessionID".into(), ck_id.value().into());
    cookies.add(ck_id);
    // return to client
    WrappedResult::new(0, "", cli_map)
}

fn session_id(cookies: &Cookies) -> String {
    if let Some(ck) = cookies.get("SessionID") {
        return ck.value().to_string();
    }
    String::from("")
}

#[post("/initial")]
pub fn initial(ctx: Context) -> JsonValue {
    let mut nick_name = String::from("");
    let sid = session_id(&ctx.req.cookies());
    if let Some(d) = super::get_session(&sid) {
        nick_name = d.get("nick_name").unwrap().to_string();
    }
    json!({"nick_name":nick_name})
}