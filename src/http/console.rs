use rocket::response::Redirect;

#[get("/")]
pub fn index()->Redirect{
    println!("hello");
    Redirect::temporary("/console/app/")
}

#[get("/login")]
pub fn login() -> &'static str {
    "admin page"
}

#[get("/index2")]
pub fn index2() -> &'static str {
    //let mut s = "Hello, world!".to_string();
    //s.push_str(req.headers().get_one("Host").unwrap());
    //s.as_str();
    "hello world"
}

