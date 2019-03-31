use regex::Regex;

pub fn ip_district<T:Into<String>>(ip:T)->Option<String>{
    let mut url = String::from("http://www.ip138.com/ips1388.asp?ip=");
    url.push_str(&ip.into());
    url.push_str("&action=2");
    let req = reqwest::get(&url);
    if req.is_ok() {
        let body = req.unwrap().text().unwrap();
        let re = Regex::new("<li>本站主数据：\\s*(.+?)\\s*</li>").unwrap();
        if let Some(cap) = re.captures(&body) {
           return Some(cap.get(1).unwrap().as_str().to_owned());
        }
    }
    None
}

#[test]
fn test_ip_district(){
    let d1 = ip_district("223.74.70.186");
    assert_eq!(Some(String::from("广东省佛山市 移动")),d1);
}