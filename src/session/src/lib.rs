extern crate typemap;

mod hash_session;

/// This `Trait` defines a session storage struct. It must be implemented on any store passed to `Sessions`.
pub trait SessionStore<K: typemap::Key>: Sync {
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