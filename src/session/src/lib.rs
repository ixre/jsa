extern crate typemap;
pub use typemap::Key;
use std::sync::Arc;

mod hash_session;

/// This `Trait` defines a session storage struct. It must be implemented on any store passed to `Sessions`.
/// The `K` should be session key.
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



/// A session which provides basic CRUD operations.
pub struct Session<K: Key> {
    key: K,
    store: Arc<Box<SessionStore<K> + 'static + Send + Sync>>
}

impl<K: Key> Session<K> {
    /// Create a new session
    pub fn new(key: K, store: Box<SessionStore<K> + 'static + Send + Sync>) -> Session<K> {
        Session {
            key: key,
            store: Arc::new(store)
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