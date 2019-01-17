use super::Key;
use super::SessionStore;
use super::Session;
use std::marker::PhantomData;


/// The sessioning middleware.
///
/// `Sessions` middleware is given a key-generating function and a
/// data store to use for sessioning.
///
/// The key is used to select a session from the store.
/// No session is actually created during selection. It is up to downstream
/// middleware to create/swap/edit sessions stored to a key.
///
/// `Sessions` allows guest sessioning (sessions without explicit authorization).
/// To prevent guest sessioning, the key generator can produce
/// an `Option` value so that all unauthorized users have an empty session.
///
/// Session keys can be stored in the `Request` or `Alloy`.
/// Usually, keys are stored in signed cookies, but anything
/// retrievable from `Request` or `Alloy` will work.
pub struct Sessions<K: Key, S: SessionStore<K>> {
    key_generator: fn(&K) -> K,
    session_store: S
}

impl<K: typemap::Key, S: SessionStore<K> + Clone> Clone for Sessions<K, S> {
    fn clone(&self) -> Sessions<K, S> {
        Sessions {
            key_generator: self.key_generator,
            session_store: self.session_store.clone()
        }
    }
}

/*
impl<K: typemap::Key, S: SessionStore<K>> Sessions<K, S> {
    /// Instantiate new sessioning middleware with the given
    /// key-generating function and session store.
    ///
    /// `key_generator` should generate keys based on the `Request` and `Alloy`.
    /// These keys should be unique, as identical keys will map to the same session.
    ///
    /// The `Alloy` can be used to access
    /// stores such as cookies to allow persistent sessions for users.
    ///
    /// `session_store` must implement the `SessionStore` trait.
    /// A default `Session` is provided to fulfill this.
    pub fn new(key_generator: fn(&Request) -> K,
               store: S) -> Sessions<K, S> {
        Sessions {
            key_generator: key_generator,
            session_store: store
        }
    }
}
*/

/// Key for inserting a Session<K, V> in the request extensions.
pub struct RequestSession<K> {
    phantom: PhantomData<K>
}

//impl<K: 'static, V: 'static> Assoc<session::Session<K, V>> for RequestSession {}
impl<K: Key> Key for RequestSession<K> { type Value = Session<K>; }
