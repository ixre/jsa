use rocket::request;
use rocket::request::FromRequest;
use rocket::response;
use rocket::response::content;
use rocket::response::Responder;
use rocket::Outcome;
use rocket::Request;

pub use self::jsa_request::all_request;
pub use self::route::mount_routes;
pub mod console;
pub mod index;
mod jsa_request;
mod route;

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

#[derive(Debug, Clone, PartialEq, Default)]
pub struct WrappedResult<T = serde_json::Value> {
    code: i8,
    err_msg: String,
    data: T,
}

impl<'a> WrappedResult {
    pub fn new<M: Into<String>, D: Into<serde_json::Value>>(
        err_code: i8,
        err_msg: M,
        data: D,
    ) -> Self {
        WrappedResult {
            code: err_code,
            err_msg: err_msg.into(),
            data: data.into(),
        }
    }
}

impl<'r> Responder<'r> for WrappedResult {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        let r = json!({
            "code":self.code,
            "err_msg":self.err_msg,
            "data":self.data
        });
        content::Json(r).respond_to(req)
    }
}
