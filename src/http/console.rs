use rocket::response::Redirect;
use rocket_contrib::json::JsonValue;
use crate::http::WrappedResult;
use serde_json::Map;

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

#[post("/user/logi2n")]
pub fn login2() -> JsonValue {
    json!({"hello":"123"})
}
#[post("/user/login")]
pub fn login() ->WrappedResult {
    let mut map = Map::new();
    map.insert("CookieName".to_string(), "user".into());
    map.insert("CookieValue".to_string(), "123".into());
    WrappedResult::new(1, "", map)
}