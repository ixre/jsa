extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use std::fs;
use std::fs::DirEntry;
use std::fs::File;
use std::io;
use std::process::exit;
use std::time;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    //主机头，*表示通配
    pub host: String,
    //全局请求跳转路径,{path}表示完整的路径；
    //{#序号}表示路径片段的序号
    pub to: String,
    //如果未设定全局请求跳转路径，那么将启用路径字典
    //如果{"a/b/c":"http://abc.com"}，访问/a/b/c将跳转
    //到"http://abc.com"
    pub location: HashMap<String, String>,
}

impl Item {
    //查找匹配
    fn get_target(&self, path: &str) -> (&str, usize) {
        let location_maps = &self.location;
        let default = &self.to;
        let mut target = default;
        let mut any_match_pos: usize = 0;
        if location_maps.contains_key(path) {
            return (location_maps.get(path).unwrap(), any_match_pos);
        }
        for (k, v) in location_maps {
            //匹配如：/d/* 含通配符的路径
            if k.ends_with("*") {
                let pos = k.len() - 1; //通配符所在的索引位置
                let anyMatch = path.starts_with(&k[0..pos]);
                //debugLog(&["[ Compare]:判断通配:".to_owned(), &k[0..any_match_pos]]);
                if anyMatch {
                    target = v;
                    any_match_pos = pos;
                    break;
                }
            }
        }
        return (target, any_match_pos);
    }
    // 获取目标地址
    pub fn get_location<'a>(&self, path: &'a str, query: &'a str, segments: Vec<&str>) -> String {
        let tuple = self.get_target(&path);
        let mut target = tuple.0.to_string();
        let pos = tuple.1;
        if target == "" {
            return String::from("");
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
        //路径通配
        if target.contains("{*}") || pos >= 0 {
            target = target.replace("{*}", &path[pos..]);
        }
        //匹配含有路径片段的URL,{#序号}表示指定的路径片段
        if target.contains("{#") {
            let mut i = 0;
            for seg in &segments {
                let mut dst = String::from("{#");
                dst.push_str(&i.to_string());
                dst.push_str(&"}");
                let v = segments.get(i);
                if v.is_none() {
                    break;
                }
                target = target.replace(dst.as_str(), v.unwrap());
                i += 1;
            }
        }
        //println!("{:#?}", target);
        return target;
    }
}


// 项目管理器
#[derive(Debug)]
pub struct ItemManager {
    // 配置路径
    pub conf_path: String,
    // 项目
    pub items: HashMap<String, Item>,
}

impl ItemManager {
    pub fn new(path: String) -> Result<ItemManager, String> {
        let mut path = path;
        if !path.ends_with("/") {
            path = path + "/";
        }
        let mut it = ItemManager {
            conf_path: path,
            items: HashMap::new(),
        };
        it.load();
        return Ok(it);
    }
    fn load(&mut self) {
        self.check();
    }
    // 检查目录，并初始化
    fn check(&mut self) -> io::Result<()> {
        let mut exists = false;
        let r = fs::read_dir(&self.conf_path);
        if r.is_err() {
            if r.unwrap_err().kind() == io::ErrorKind::NotFound {
                fs::create_dir(&self.conf_path);
            }
        }else {
            for entry in r? {
                let path = entry?.path();
                let file_path = path.to_str().unwrap();
                if file_path.ends_with(".conf") {
                    self.load_from(file_path);
                    exists = true;
                }
            }
        }
        // 未找到配置文件初始化一个示例
        if !exists {
            let mut map = HashMap::new();
            map.insert("/a".to_owned(), "http://a.com/a{timestamp}".to_owned());
            map.insert("/a/*".to_owned(), "http://a.com/t-{*}".to_owned());
            map.insert("/a/b".to_owned(), "http://a.com/{path}{query}{timestamp}".to_owned());
            map.insert("/a/b/c".to_owned(), "http://a.com/{#0}-{#1}-{#2}".to_owned());
            let it = Item {
                host: "localhost *.a.com".to_owned(),
                to: "http://www.google.com/{path}{query}".to_owned(),
                location: map,
            };
            let r = File::create(self.conf_path.to_owned() + "default.conf");
            if r.is_err() {
                return Err((r.unwrap_err()));
            }
            println!("{}", self.conf_path.to_owned());
            let vec = vec!(it);
            self.append(&vec);
            serde_json::to_writer_pretty(r.unwrap(), &vec);
        }
        return Ok(());
    }

    // load items from a file
    fn load_from(&mut self, path: &str) -> Result<bool, &str> {
        let fi = File::open(path).unwrap();
        let items: Vec<Item> = serde_json::from_reader(fi)
            .expect(&("can't read config from file :".to_owned() + path));
        self.append(&items);
        if 1 > 0 {
            return Err("error");
        }
        return Ok(true);
    }

    fn append(&mut self, items: &Vec<Item>) {
        for it in items {
            let host_arr: Vec<&str> = it.host.split(" ").collect();
            for host in host_arr {
                if self.items.contains_key(host) {
                    println!("[ Jrd][ Panic]: host {} already exists", host);
                    exit(1);
                }
                self.items.insert(host.to_owned(), it.clone());
            }
        }
    }

    // 根据主机名获取相应的配置,如果无匹配，则默认使用localhost
    pub fn get_item(&self, host: &String) -> Option<&Item> {
        for (k, v) in &self.items {
            if self.match_host(&k, host) {
                return Option::Some(&v);
            }
        }
        return self.items.get("localhost");
    }
    // 匹配主机
    fn match_host(&self, from: &String, host: &String) -> bool {
        // 判断是否泛解析
        if host.starts_with("*.") {
            return from.ends_with(&host[2..]);
        }
        return *from == *host;
    }
}
