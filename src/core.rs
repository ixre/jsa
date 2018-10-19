extern crate iron;

use item::{Item, ItemManager};
use self::iron::headers;
use self::iron::IronResult;
use self::iron::Request;
use self::iron::Response;
use self::iron::status;

const GLOB_DEBUG: bool = !false;

fn debug_log(s: &[String]) {
    if GLOB_DEBUG {
        println!("[ Jrd][ Log]: {}", s.join(" "))
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

impl Entry {
    fn get_location(&self, _r: &Request, _it: &Item) -> &str {
        return "";
    }
}

// 实现handler
impl iron::Handler for Entry {
    // 处理请求
    fn handle(&self, r: &mut Request) -> IronResult<Response> {
        let host = r.url.host().to_string();
        let item = self.item_manager.get_item(&host);
        debug_log(&[String::from("source host"), host]);
        if !item.is_none() {
            let location = self.get_location(r, item.unwrap());
            if location.len() > 0 {
                let mut rsp = Response::with(status::Found);
                rsp.headers.set(headers::Location(location.to_string()));
                return Ok(rsp);
            }
        }
        return Ok(Response::with((status::Ok, "Not match any host")));
    }
}



