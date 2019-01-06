use rocket::request;
use rocket::request::FromRequest;
use rocket::Outcome;
use rocket::Request;

pub use self::jsa_request::all_request;

//pub mod console;
pub mod index;
mod jsa_request;

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
