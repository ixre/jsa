use crate::http::Context;
use md5;
use sha1::Sha1;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time;

/// Generate user's pwd. It's use SHA1 algorithm
pub fn pwd<P: Into<String>>(p: P) -> String {
    let mut s = Sha1::new();
    s.update(p.into().as_bytes());
    s.digest().to_string()
}

/// Gets unixstamp of system time
pub fn unix_sec() -> u64 {
    time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn hash<T: Hash>(t: T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

/// Return short hash letters
pub fn short_hash<T: Hash>(t: T) -> String {
    let digest = md5::compute(hash(t).to_string());
    (&format!("{:x}", digest)[16..24]).to_owned()
}

#[test]
fn test_hash() {
    let h1 = short_hash("to2.net");
    let h2 = short_hash("s.to2.net");
    assert_eq!(h1, h2);
}

/// Get prefix link of current request url.
pub fn self_pre_link(ctx: &Context) -> String {
    let host = ctx.header("Host");
    let origin = ctx.header("Origin");
    if origin.ends_with(&host) {
        return origin;
    }
    let mut s = origin[0..origin.find("//").unwrap_or(0)].to_owned();
    s.push_str("//");
    s.push_str(&host);
    s
}
