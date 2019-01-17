extern crate time;

use rocket::http::{Cookie, Cookies};
use rocket::response::Redirect;
use rocket_contrib::json::JsonValue;
use serde_json::Map;

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
    let mut session_id = Cookie::new("SessionID", "21293123123");
    let mut expires = time::now_utc();
    expires.tm_min += 30;
    session_id.set_expires(expires);
    session_id.set_path("/console/api");
    let mut map = Map::new();
    map.insert("SessionID".into(), session_id.value().into());
    cookies.add(session_id);
    WrappedResult::new(1, "", map)
}
