use sha1::Sha1;
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
