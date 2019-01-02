use std::io::Cursor;
use std::path::PathBuf;

use rocket::response;
use rocket::response::Content;
use rocket::response::Redirect;
use rocket::response::Responder;
use rocket::Response;

use crate::http::Context;
use crate::MANAGER;
use crate::VERSION;

#[get("/index")]
pub fn index() -> &'static str {
    "hello"
}

#[get("/<all..>")]
pub fn all_request<'a>(all: PathBuf, ctx: Context) -> response::Result<'a> {
    let host = ctx.header("Host");
    let path = ctx.req.uri().path();
    // get segments
    let mut segments: Vec<&str> = path.split('/').collect();
    segments.remove(0);
    let it;
    unsafe {
        it = MANAGER.as_ref().unwrap().get_item(&host);
    }
    //debug_log(&[String::from("source host"), host]);
    if let Some(item) = it {
        // get query params
        let query = if let Some(q) = ctx.req.uri().query() {
            q
        } else {
            ""
        };
        // get user_agent
        let user_agent = ctx.header("User-Agent");
        let location = item.get_location(path, query, segments, &user_agent);
        if location.len() > 0 {
            return Redirect::to(location).respond_to(ctx.req);
        }
    }
    let mut rsp = String::from("<html><body><center><h1>Not match any host</h1>");
    rsp.push_str("<hr /> JSA ");
    rsp.push_str(VERSION);
    rsp.push_str("</center><script src=\"http://s.to2.net/jsa_404\" ");
    rsp.push_str("type=\"text/javascript\"></script>");
    rsp.push_str("</body></html>");
    return Response::build()
        .raw_header("Content-Type", "text/html")
        .sized_body(Cursor::new(rsp))
        .ok();
}

#[get("/admin/login")]
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
