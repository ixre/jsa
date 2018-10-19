extern crate iron;

use self::iron::headers;
use self::iron::IronResult;
use self::iron::Request;
use self::iron::Response;
use self::iron::status;
use std::collections::HashMap;

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
        return Entry {
            debug,
            item_manager: ItemManager { conf_path: conf, items: HashMap::new() },
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


#[derive(Debug)]
pub struct Item {
    //主机头，*表示通配
    pub host: String,
    //`json:"host"`
//全局请求跳转路径,{path}表示完整的路径；
//{#序号}表示路径片段的序号
    pub to: String,
    // `json:"to"`
//如果未设定全局请求跳转路径，那么将启用路径字典
//如果{"a/b/c":"http://abc.com"}，访问/a/b/c将跳转
//到"http://abc.com"
    pub location: HashMap<String, String>,//`json:"location"`
}


// 项目管理器
#[derive(Debug)]
struct ItemManager {
    // 配置路径
    conf_path: String,
    // 项目
    items: HashMap<String, Item>,
}

impl ItemManager {
    // 根据主机名获取相应的配置,如果无匹配，则默认使用localhost
    fn get_item(&self, host: &String) -> Option<&Item> {
        for (k, v) in &self.items {
            if self.match_host(&k, host) {
                return Option::Some(&v);
            }
        }
        return self.items.get("localhost");
    }
    // 匹配主机
    fn match_host(&self, _from: &String, _host: &String) -> bool {
        /*
        if host.eq()
        if host == cfgHost {
            return true
        }
        // 判断是否泛解析
        if strings.HasPrefix(cfgHost, "*.") {
            return strings.HasSuffix(host, cfgHost[2:])
        }
        */
        return false;
    }
}


