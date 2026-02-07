use std::any::{Any, TypeId};

use parking_lot::RwLock;
use ryhash::TypeIdMap;

/// Utility struct for constructing and storing &'static strs associated with a type.
///
/// Intended to  be used in the implementation of TypePath for structs with generic parameters.
pub struct GenericTypeCell<T: ?Sized + 'static = str>(RwLock<TypeIdMap<&'static T>>);

impl<T: ?Sized + 'static> GenericTypeCell<T> {
    pub const fn new() -> Self {
        Self(RwLock::new(TypeIdMap::with_hasher(
            std::hash::BuildHasherDefault::new(),
        )))
    }

    pub fn get_or_insert<G, F, A>(&self, f: F) -> &'static T
    where
        G: Any + ?Sized,
        F: FnOnce() -> A,
        Box<T>: From<A>,
    {
        let type_id = TypeId::of::<G>();
        if let Some(v) = self.0.read().get(&type_id) {
            return v;
        }

        let v = (f)();
        let v: &'static T = Box::leak(Box::from(v));
        self.0.write().entry(type_id).or_insert(v)
    }
}
