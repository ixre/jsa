
use rocket::request;
use rocket::Request;
use rocket::request::FromRequest;
use rocket::Outcome;


pub mod console;
pub mod entry;

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
        Outcome::Success(Context { req: request })
    }
}
