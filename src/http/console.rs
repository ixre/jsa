use rocket::response::Redirect;

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
