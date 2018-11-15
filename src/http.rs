extern crate iron;

use crate::jsa:: ItemManager;
use self::iron::headers;
use self::iron::IronResult;
use self::iron::Request;
use self::iron::Response;
use self::iron::status;
use std::str;

const GLOB_DEBUG: bool = false;

fn debug_log(s: &[String]) {
    if GLOB_DEBUG {
        println!("[ Jsa][ Log]: {}", s.join(" "))
    }
}

#[derive(Debug)]
pub struct Entry {
    debug: bool,
    item_manager: ItemManager,
}

impl Entry {
    pub fn new(conf: String, debug: bool) -> Entry {
        let r = ItemManager::new(conf);
        return Entry {
            debug,
            item_manager: r.unwrap(),
        };
    }
}

// 实现handler
impl iron::Handler for Entry {
    // 处理请求
    fn handle(&self, r: &mut Request) -> IronResult<Response> {
        let segments = r.url.path();
        // get path
        let mut path = segments.join("/");
        path.insert(0, '/');
        let path = path.as_str();
        if path == "/favicon.ico" { // 不处理favicon.ico请求
            return Ok(Response::with(status::NotFound));
        }
        let host = r.url.host().to_string();
        let it = self.item_manager.get_item(&host);
        debug_log(&[String::from("source host"), host]);
        if let Some(item) = it {
            // get query params
            let query = if let Some(q) = r.url.query() { q } else { "" };
            // get user_agent
            let opt = r.headers.get_raw("User-Agent");
            let mut user_agent = "";
            if opt.is_some() {
                let v = opt.unwrap().get(0).unwrap();
                user_agent = str::from_utf8(&v).unwrap();
            }
            let location = item.get_location(path, query, segments, user_agent);
            if location.len() > 0 {
                let mut rsp = Response::with(status::Found);
                rsp.headers.set(headers::Location(location.to_string()));
                return Ok(rsp);
            }
        }
        return Ok(Response::with((status::Ok, "Not match any host")));
    }
}



