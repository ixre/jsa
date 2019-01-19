use std::io::Cursor;

use rocket::response;
use rocket::response::Redirect;
use rocket::response::Responder;
use rocket::Response;

use crate::MANAGER;
use crate::VERSION;

use super::Context;

///! Handle redirect request.
pub fn all_request<'a>(ctx: Context) -> response::Result<'a> {
    let mut host = ctx.header("Host");
    if host != "" {
        let vec: Vec<&str> = host.split(":").collect();
        host = String::from(*vec.get(0).unwrap());
    }
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
            return Redirect::temporary(location).respond_to(ctx.req);
        }
    }
    let mut rsp = String::from("<html><body><center><h1>Not match any host</h1>");
    rsp.push_str("<hr /> JSA ");
    rsp.push_str(VERSION);
    rsp.push_str("</center><script src=\"http://s.to2.net/jsa_404?host=");
    rsp.push_str(&host);
    rsp.push_str("&amp;path=");
    rsp.push_str(&path);
    rsp.push_str("\" type=\"text/javascript\"></script>");
    rsp.push_str("</body></html>");
    return Response::build()
        .raw_header("Content-Type", "text/html")
        .raw_status(404, "Not Found")
        .sized_body(Cursor::new(rsp))
        .ok();
}
