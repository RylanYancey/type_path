use std::any::{Any, TypeId};

use parking_lot::RwLock;
use ryhash::TypeIdMap;

/// Utility struct for constructing and storing &'static strs associated with a type.
///
/// Intended to  be used in the implementation of TypePath for structs with generic parameters.
pub struct GenericTypeCell(RwLock<TypeIdMap<&'static str>>);

impl GenericTypeCell {
    pub const fn new() -> Self {
        Self(RwLock::new(TypeIdMap::with_hasher(
            std::hash::BuildHasherDefault::new(),
        )))
    }

    pub fn get_or_insert<G, F>(&self, f: F) -> &'static str
    where
        G: Any + ?Sized,
        F: FnOnce() -> String,
    {
        let type_id = TypeId::of::<G>();
        match self.0.read().get(&type_id) {
            Some(v) => *v,
            None => self
                .0
                .write()
                .entry(type_id)
                .or_insert(Box::leak(Box::new((f)()))),
        }
    }
}
