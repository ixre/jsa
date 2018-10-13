extern crate iron;

use self::iron::IronResult;
use self::iron::Request;
use self::iron::Response;
use self::iron::status;
use std::collections::HashMap;

const glob_debug: bool = !false;

fn debug_log(s: &[String]) {
    if glob_debug {
        println!("[ Jrd][ Log]: {}", s.join(" "))
    }
}

#[derive(Debug)]
pub struct Entry {
    debug:bool,
    item_manager:ItemManager,
}
impl Entry{
    pub fn new(conf:String,debug:bool)-> Entry {
        // glob_debug = debug;
        return Entry {
            debug,
            item_manager: ItemManager{ confPath: String::new(), items:HashMap::new() },
        }
    }
}
// 实现handler
impl iron::Handler for Entry{
    fn handle(&self,r:&mut Request) -> IronResult<Response>{
        let host = r.url.host().to_string();
        debug_log(&mut [String::from("source host"), host]);
        /*
        var item * Item = r.itemManager.GetItemByHost(host)
        if item != nil {
            if location, b: = r.getLocation(req, item);
            b {
                rsp.Header().Add("Location", location)
            rsp.WriteHeader(302)
            return;
        }
    }
    rsp.Write([]byte("Not match any host")) */
        Ok(Response::with((status::Ok, "Hello World!")))
    }
}


pub fn hello() {
    println!("hello gor")
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
    confPath: String,
    // 项目
    items: HashMap<String, Item>,
}


