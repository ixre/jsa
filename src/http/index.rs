use std::io::Cursor;
use std::path::PathBuf;

use rocket::response;
use rocket::Response;
use rocket::response::NamedFile;

use super::all_request;
use super::Context;

#[get("/")]
pub fn index<'a>(ctx: Context) -> response::Result<'a> {
    all_request(ctx)
}

#[get("/favicon.ico")]
pub fn favicon() -> Option<NamedFile> {
    NamedFile::open("./static/favicon.ico").ok()
}

#[get("/<_all..>", rank = 11)]
pub fn all<'a>(_all: PathBuf, ctx: Context) -> response::Result<'a> {
    all_request(ctx)
}

#[get("/board")]
pub fn board<'a>() -> response::Result<'a> {
    Response::build()
        .raw_header("Content-Type", "text/html")
        .raw_status(404, "Not Found")
        .sized_body(Cursor::new(
            "<script>location.assign(\
             '/console/#/home')</script>",
        ))
        .ok()
}
