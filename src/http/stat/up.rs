use core::borrow::Borrow;
use std::collections::HashMap;
use std::io::Cursor;

use base64;
use rocket::{Data, Outcome, Request, response, Response};
use rocket::request;
use rocket::request::FromRequest;

use crate::{conn, util};
use crate::models::stat;
use crate::models::stat::StatFromType;
use crate::repo::DomainRepo;

pub struct Pack{
    hash:String,
    pack:HashMap<String,String>,
    client_ip:String
}
impl Pack{
    fn decode_pack(pack:&String,map:&mut HashMap<String,String>) {
        let u = base64::decode(pack).unwrap_or(vec![]);
        let pack_str = String::from_utf8(u).unwrap();
        let url = url::form_urlencoded::parse(pack_str.as_bytes());
        for (key, value) in url.into_owned() {
            map.insert(key.clone(), value.clone());
        }
    }
    fn get<T:Into<String>>(&self,k:T)->Option<String> {
        match self.pack.get(&k.into()) {
            Some(s) => Some(s.to_owned()),
            _ => None
        }
    }
    fn get_from(&self)->String{
        if let Some(query) = self.get("query"){
            for it in stat::INTERNAL_STAT_FROM_VEC.iter(){
                if it.detect_type == StatFromType::Query as i16{
                    if query.contains(&it.keyword){
                        return it.key.clone();
                    }
                }
            }
        }
        if let Some(referer) = self.get("referer"){
            for it in stat::INTERNAL_STAT_FROM_VEC.iter(){
                if it.detect_type == StatFromType::Referer as i16{
                    if referer.contains(&it.keyword){
                        return it.key.clone();
                    }
                }
            }
        }
        if let Some(user_agent) = self.get("user_agent"){
            for it in stat::INTERNAL_STAT_FROM_VEC.iter(){
                if it.detect_type == StatFromType::UserAgent as i16{
                    if user_agent.contains(&it.keyword){
                        return it.key.clone();
                    }
                }
            }
        }
        String::from("-")
    }
    fn get_os(&self)->String {
        if let Some(user_agent) = self.get("user_agent") {
            if user_agent.contains("Mac OS X") {
                return String::from("Mac");
            }
            if user_agent.contains("Windows") {
                return String::from("Windows");
            }
            if user_agent.contains("Linux") {
                return String::from("Linux");
            }
            if user_agent.contains("Android") {
                return String::from("Android");
            }
            if user_agent.contains("iPhone") {
                return String::from("IPhone");
            }
        }
        String::from("")
    }
}

impl<'f, 'r> FromRequest<'f, 'r> for Pack{
    type Error = !;
    fn from_request(request: &'f Request<'r>) -> request::Outcome<Self, !> {
        let string = request.uri().query().unwrap();
        let url = url::form_urlencoded::parse(string.as_bytes());
        let mut hash:String = String::from("");
        let mut mp = HashMap::new();
        for (key, value) in url.into_owned() {
            match key.as_str() {
                "hash" => hash = value,
                "pack" => Self::decode_pack(&value,&mut mp),
                _=>{}
            }
        }
        let mut client_ip = String::from("");
        if let Some(ip) = request.client_ip(){
            client_ip = ip.to_string();
        }
        Outcome::Success(Pack{ hash, pack: mp,client_ip })
    }
}


 fn response<'a>(s:String)->response::Result<'a>{
     Response::build()
         .raw_header("Content-Type", "text/javascript;charset=utf-8")
         .raw_status(200, "")
         .sized_body(Cursor::new(s))
         .ok()
 }

#[derive(Serialize,Debug)]
pub struct PostResult{
    // 来源
    pub from:String,
    // 设备系统
    pub device_os:String,
    /// 客户端IP
    pub client_ip:String,
    // 用户区域
    pub user_district:String,
    // 用户大致位置
    pub user_place:String
}

#[get("/site_po")]
pub fn site_po<'a>(pack:Pack) ->response::Result<'a> {
    let conn = conn();
    let d = DomainRepo::get_domain(&conn,pack.hash.to_owned());
    dbg!(&pack.pack);
    let callback = pack.get("callback").unwrap_or("_callback".to_string());
    // 获取位置
    let district = util::ip_district(pack.client_ip.clone());
    // 包装结果
    let pr = PostResult{
        from: pack.get_from(),
        device_os: pack.get_os(),
        client_ip: pack.client_ip.clone(),
        user_district:district.0,
        user_place:district.1
    };
    let mut s = String::from(callback);
    s.push_str("(");
    s.push_str(&serde_json::to_string(&json!(pr)).unwrap());
    //s.push_str(&pack_str);
    s.push_str(");");
    response(s)
}
