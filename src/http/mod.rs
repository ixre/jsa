use std::io::Cursor;

use iron::status::Status;
use rocket::outcome::Outcome;
use rocket::outcome::Outcome::Success;
use rocket::request;
use rocket::request::FromRequest;
use rocket::response;
use rocket::response::Responder;
use rocket::response::ResponseBuilder;
use rocket::Request;
use rocket::Response;

use crate::jsa::ItemManager;

pub mod admin;
pub mod entry;

/*
#[derive(Debug)]
struct HTML<'a> {
    rsp: ResponseBuilder<'a>
}

impl<'a> HTML {
    pub fn new() -> HTML<'a> {
        HTML {
            rsp: Response::build()
        }
    }
}


impl<'a> Responder<'a> for HTML {
    fn respond_to(self, request: &Request) -> response::Result<'a> {
        //self.response.respond_to(request)

    }
}
*/

pub struct Context<'a, 'r> {
    req: &'a Request<'r>,
}

impl<'a, 'r> Context<'a, 'r> {
    pub fn header<T: Into<String>>(&self, s: T) -> String {
        self.req.headers().get(s.into().as_str()).collect()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Context<'a, 'r> {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, !> {
        let h: String = request.headers().get("Host").collect();
        Success(Context { req: request })
    }
}
