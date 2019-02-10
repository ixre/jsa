extern crate md5;
extern crate typemap;
extern crate uuid;

use core::hash::Hash;
use std::collections::HashMap;
use std::sync::Arc;

pub use self::hash_storage::HashSessionStore;
pub use typemap::Key;
use uuid::Uuid;

mod hash_storage;
mod session;
mod tests;

/// This `Trait` defines a session storage struct. It must be implemented on any store passed to `Sessions`.
/// The `K` should be session key.
///  let session_id = "1".into();
/// ```
///  // Create session storage
///  let store = hash_session();
///  // Insert new session
///  let mut map = HashMap::new();
///  map.insert("user_id".to_string(), "1".to_string());
///  let mut map_mut = map.clone();
///  store.set(&session_id, map);
///  // Update existed session
///  map_mut.insert("user_name".to_string(),"jarrysix".to_string());
///  store.set(&session_id, map_mut);
///  // Get session
///  let map :HashMap<String,String> = store.get(&session_id).unwrap();
///  println!("{:#?}",map);
///```
pub trait SessionStore<K: Key>: Sync {
    /// Set the value of the session belonging to `key`, replacing any previously set value.
    fn insert(&self, key: &K, value: K::Value);
    /// Retrieve the value of this session.
    ///
    /// Returns `None` if the session belonging to `key` has not been set.
    fn get(&self, key: &K) -> Option<K::Value>;
    /// Swap the given value with the current value of the session belonging to `key`.
    ///
    /// Returns the value being replaced, or `None` if this session was not yet set.
    fn set(&self, key: &K, value: K::Value) -> Option<K::Value>;
    /// Remove the session stored at this key.
    fn remove(&self, key: &K) -> bool;
}

pub fn generate_id()->String{
    let md5_str = md5::compute(Uuid::new_v4().to_hyphenated().to_string());
    format!("{:x}",md5_str)
}

/// A session which provides basic CRUD operations.
pub struct Session<K: Key> {
    key: K,
    store: Arc<Box<SessionStore<K> + 'static + Send + Sync>>,
}

impl<K: Key> Session<K> {
    /// Create a new session
    pub fn new(key: K, store: Box<SessionStore<K> + 'static + Send + Sync>) -> Session<K> {
        Session {
            key: key,
            store: Arc::new(store),
        }
    }
    /// Set the value of this session, replacing any previously set value.
    pub fn insert(&self, value: K::Value) {
        self.store.insert(&self.key, value)
    }
    /// Retrieve the value of this session.
    ///
    /// Returns `None` if this session has not been set.
    pub fn find(&self) -> Option<K::Value> {
        self.store.get(&self.key)
    }
    /// Swap the given value with the current value of this session.
    ///
    /// Returns the value being replaced.
    /// Returns `None` if this session was not yet set.
    pub fn set(&self, value: K::Value) -> Option<K::Value> {
        self.store.set(&self.key, value)
    }

    /// Remove the session stored at this key.
    pub fn remove(&self) -> bool {
        self.store.remove(&self.key)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct SessionPair(String);

impl From<String> for SessionPair {
    fn from(s: String) -> Self {
        SessionPair(s)
    }
}

impl From<&str> for SessionPair {
    fn from(s: &str) -> Self {
        SessionPair(s.into())
    }
}

/// Impl Key for some types
impl typemap::Key for SessionPair {
    type Value = HashMap<String, String>;
}

pub fn hash_session() -> HashSessionStore<SessionPair> {
    let s: HashSessionStore<SessionPair> = HashSessionStore::new();
    s
}


pub fn hash_session_type<T: Key + Eq + Hash>() -> HashSessionStore<T> {
    let s: HashSessionStore<T> = HashSessionStore::new();
    s
}
