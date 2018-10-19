use std::collections::HashMap;

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
pub struct ItemManager {
    // 配置路径
    pub conf_path: String,
    // 项目
    pub items: HashMap<String, Item>,
}

impl ItemManager {
    pub fn new(path: String) -> Result<ItemManager,String>{
        let it= ItemManager {
            conf_path: path,
            items: HashMap::new(),
        };
        &it.load();
        return Ok(it);
    }
    fn load(&self){
        println!("{}",self.conf_path);
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
