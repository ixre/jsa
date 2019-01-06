use std::path::PathBuf;

use crate::http::all_request;
use crate::http::Context;
use rocket::response;
use rocket::response::NamedFile;
use rocket::response::Redirect;

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

#[get("/login")]
pub fn login() -> Redirect {
    Redirect::temporary("/console/")
}
