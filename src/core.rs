extern crate iron;

use item::{Item, ItemManager};
use self::iron::headers;
use self::iron::IronResult;
use self::iron::Request;
use self::iron::Response;
use self::iron::status;
use self::iron::Url;
use std::collections::HashMap;
use std::time;

const GLOB_DEBUG: bool = false;

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
    //查找匹配
    fn get_target<'a>(&self, path: &str, location_maps: &'a HashMap<String, String>,
                      default: &'a str) -> (&'a str,usize) {
        let mut target = default;
        let mut any_match_pos: usize = 0;
        for (k, v) in location_maps {
            debug_log(&["[ Compare]:对比相同，path:".to_owned(), path.to_owned(),
                "; key:".to_owned(), k.clone()]);
            if path == k {  // 判断路径是否相同
                target = &v;
                break;
            }
            //匹配如：/d/* 含通配符的路径
            if k.ends_with("*") {
                any_match_pos = k.len() - 1; //通配符所在的索引位置
                let anyMatch = path.starts_with(&k[0..any_match_pos]);
                //debugLog(&["[ Compare]:判断通配:".to_owned(), &k[0..any_match_pos]]);
                if anyMatch {
                    target = v;
                    break;
                }
            }
        }
        return (target,any_match_pos);
    }
    fn get_location<'a>(&self, r: &Request, path: &str, segments: Vec<&str>, query: &str, it: &'a Item) -> &'a str {
        let tuple = self.get_target(&path, &it.location, &it.to);
        let mut target = tuple.0.to_string();
        let pos = tuple.1;
        if target == "" {
            return "";
        }
        //路径通配
        if target.contains("{*}") || pos >=0 {
            target = target.replace("{*}", &path[pos..]);
        }
        // 全局请求跳转路径,{path}表示完整的路径；
        if target.contains("{path}") {
            target = target.replace("{path}", &path[1..]);
        }
        // 处理查询条件，{query}表示查询条件
        let concat = if query == "" { "" } else { "?" };
        let qt = target.contains("{query}");
        if qt {
            target = target.replace("{query}", &(concat.to_owned() + query));
        }
        // 加上时间戳请求 {timestamp}会返回时间戳
        if target.contains("{timestamp}") {
            let unix = time::SystemTime::now().duration_since(time::UNIX_EPOCH)
                .unwrap().as_secs();
            let mut unix = unix.to_string();
            if !qt || concat == "" {
                unix = "?_stamp=".to_owned() + &unix
            } else {
                unix = "&_stamp=".to_owned() + &unix
            }
            target = target.replace("{timestamp}", unix.as_str())
        }


        /*
             // {query}表示查询条件

             // {timestamp}会返回时间戳

             //路径通配
             if strings.Contains(target, "{*}") && anyMatchPos != -1 {
                 target = strings.Replace(target, "{*}", path[anyMatchPos:], -1)
             }
             //匹配含有路径片段的URL,{#序号}表示指定的路径片段
             if strings.Contains(target, "{#") {
                 segments := strings.Split(path[1:], "/")
                 for i, l := 0, len(segments); i < l; i++ {
                     target = strings.Replace(target, "{#"+strconv.Itoa(i)+"}",
                                              segments[i], -1)
                 }
             }
             debugLog("--- origin:", path, "; target:", target)
             return target, true
     */
        //let path = r.url.as_ref().query().unwrap();
        println!("{:#?}", target);
        return "";
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
        let item = self.item_manager.get_item(&host);
        debug_log(&[String::from("source host"), host]);
        if !item.is_none() {
            let qn = r.url.query();
            let mut query = "";
            if qn.is_some() {
                query = qn.unwrap();
            }
            let location = self.get_location(r, path, segments, query, item.unwrap());
            if location.len() > 0 {
                let mut rsp = Response::with(status::Found);
                rsp.headers.set(headers::Location(location.to_string()));
                return Ok(rsp);
            }
        }
        return Ok(Response::with((status::Ok, "Not match any host")));
    }
}



