//use regex::NoExpand;
use regex::Regex;

// 查询IP: www.cip.cc/xxx.xxx
lazy_static! {
    static ref IPIP_REGEX_TRIM: Regex = Regex::new("(<[^>]+?>)|\\r|\\s|\\n").unwrap();
    static ref IPIP_PLACE: Regex =
        Regex::new("地理位置(.+?)(产品详情|\\(可信度)").unwrap();
}
pub fn ip_district<T: Into<String>>(ip: T) -> (String, String) {
    let ip = ip.into();
    if ip.len() == 0
        || ip.eq("127.0.0.1")
        || ip.starts_with("192.168.")
        || ip.starts_with("172.16.")
        || ip.starts_with("172.17.")
        || ip.starts_with("172.31.")
        || ip.starts_with("10.")
    {
        return (String::from("本地网络"), String::from("局域网"));
    }
    let params = [("ip", ip)];
    let client = reqwest::Client::new();
    match client
        .post("https://www.ipip.net/ip.html")
        .header(
            reqwest::header::USER_AGENT,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.14; rv:67.0) Gecko/20100101 Firefox/67.0",
        )
        .header(reqwest::header::REFERER, "https://www.ipip.net")
        .form(&params)
        .send()
    {
        Ok(mut rsp) => {
            let body = rsp.text().unwrap_or("".to_string());
            let body: &str = &IPIP_REGEX_TRIM.replace_all(&body, "");
            let mut place_arr = vec![];
            for caps in IPIP_PLACE.captures_iter(&body) {
                if let Some(c) = caps.get(1) {
                    place_arr.push(c.as_str().to_owned());
                }
            }
            dbg!(&place_arr);
            match place_arr.len() {
                3 => (place_arr[0].clone(), place_arr[2].clone()),
                2 => (place_arr[0].clone(), place_arr[1].clone()),
                1 => (place_arr[0].clone(), String::from("")),
                _ => (String::from(""), String::from("")),
            }
        }
        Err(err) => {
            println!("{}", err);
            (String::from(""), String::from(""))
        }
    }
}

pub fn ip_district2<T: Into<String>>(ip: T) -> Option<String> {
    let mut url = String::from("http://www.ip138.com/ips1388.asp?ip=");
    url.push_str(&ip.into());
    url.push_str("&action=2");
    let req = reqwest::get(&url);
    if req.is_ok() {
        let body = req.unwrap().text().unwrap();
        let re = Regex::new("<li>本站主数据：\\s*(.+?)\\s*</li>").unwrap();
        dbg!(&body);
        if let Some(cap) = re.captures(&body) {
            return Some(cap.get(1).unwrap().as_str().to_owned());
        }
    }
    None
}

#[test]
fn test_ip_district() {
    let d1 = ip_district("223.74.70.186");
    dbg!(d1);
    assert_eq!(0, 1);
}
