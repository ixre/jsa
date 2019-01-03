



#[get("/login")]
pub fn login() -> &'static str {
    "admin page"
}

#[get("/")]
pub fn index2() -> &'static str {
    //let mut s = "Hello, world!".to_string();
    //s.push_str(req.headers().get_one("Host").unwrap());
    //s.as_str();
    "hello world"
}

