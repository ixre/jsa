use std::collections::HashMap;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathItem {
    pub id: i32,
    pub path: String,
    pub pattern: String,
    pub note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainPaths {
    pub host: String,
    pub global_pattern: String,
    pub paths: Vec<PathItem>,
}

/*
host = "to2.net"
global_pattern = ""
[[paths]]
id = 1
path = "/"
pattern = "//baidu.com"
note = ""
*/