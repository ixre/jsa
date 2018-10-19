use std::collections::HashMap;
use std::fs;
use std::fs::DirEntry;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::path::Path;
use std::process::exit;

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
    pub fn new(path: String) -> Result<ItemManager, String> {
        let it = ItemManager {
            conf_path: path,
            items: HashMap::new(),
        };
        &it.load();
        return Ok(it);
    }
    fn load(&self) {
        self.check();
    }
    // 检查目录，并初始化
    fn check(&self) -> io::Result<()> {
        for entry in fs::read_dir(&self.conf_path)? {
            let entry = entry?;
            if entry.path().to_str().unwrap().ends_with(".conf") {
                return Ok(());
            }
        }
        // 未找到配置文件初始化一个示例
        let mut map = HashMap::new();
        map.insert("/a".to_owned(), "http://a.com/{path}{query}{timestamp}".to_owned());
        map.insert("/a/*".to_owned(), "http://a.com/t-{*}".to_owned());
        map.insert("/1/2/3/".to_owned(), "http://a.com/{#0}-{#1}-{#2}".to_owned());
        let it = &Item {
            host: "localhost localhost:8302 *.to2.net".to_owned(),
            to: "http://www.to2.net/{path}{query}".to_owned(),
            location: map,
        };
        let arr = [it];
        return Ok(());
    }

    fn visit_dirs(&self, dir: &Path, pattern: &String, cb: &Fn(&DirEntry, &String)) -> io::Result<()> {
        if fs::metadata(dir)?.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                if fs::metadata(entry.path())?.is_dir() {
                    self.visit_dirs(&entry.path(), pattern, cb)?;
                } else {
                    cb(&entry, pattern);
                }
            }
        } else {
            let entry = fs::read_dir(dir)?.next().unwrap()?;
            cb(&entry, pattern);
        }
        Ok(())
    }
    /*
    // 检查目录，并初始化
func (i *ItemManager) checkDir(path string) {
	_, err := os.Stat(path)
	//创建目录
	if os.IsNotExist(err) {
		os.MkdirAll(path, os.ModePerm)
		i.initExample(path)
	} else {
		//是否存在.conf文件,不存在，则初始化
		fi, _ := os.Open(path)
		exits := false
		list, _ := fi.Readdirnames(-1)
		for _, v := range list {
			if strings.HasSuffix(v, ".conf") {
				exits = true
			}
		}
		if !exits {
			i.initExample(path)
		}
	}
}

*/

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
