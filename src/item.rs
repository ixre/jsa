extern crate serde;
extern crate serde_json;
use std::collections::HashMap;
use std::fs;
use std::fs::DirEntry;
use std::fs::File;
use std::io;
use std::process::exit;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct ItemManager {
    // 配置路径
    pub conf_path: String,
    // 项目
    pub items: HashMap<String, Item>,
}

impl ItemManager {
    pub fn new(path: String) -> Result<ItemManager, String> {
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
        for entry in fs::read_dir(&self.conf_path)? {
            let path = entry?.path();
            let file_path = path.to_str().unwrap();
            if file_path.ends_with(".conf") {
                self.load_from(file_path);
                exists = true;
            }
        }
        // 未找到配置文件初始化一个示例
        if !exists {
            let mut map = HashMap::new();
            map.insert("/a".to_owned(), "http://a.com/{path}{query}{timestamp}".to_owned());
            map.insert("/a/*".to_owned(), "http://a.com/t-{*}".to_owned());
            map.insert("/1/2/3/".to_owned(), "http://a.com/{#0}-{#1}-{#2}".to_owned());
            let it = Item {
                host: "localhost localhost:8302 *.to2.net".to_owned(),
                to: "http://www.to2.net/{path}{query}".to_owned(),
                location: map,
            };
            let r = File::create(self.conf_path.to_owned() + "default.conf");
            if r.is_err() {
                return Err((r.unwrap_err()));
            }
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
