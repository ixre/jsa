use sha1::Sha1;
use md5;
use std::time;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

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