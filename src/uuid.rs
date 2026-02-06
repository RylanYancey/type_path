use crate::TypePath;

/// A u128 computed by hashing the type path.
///
/// This is not cached on the type, so I recommend you cache it separately to avoid re-computing.
#[diagnostic::on_unimplemented(
    message = "`{Self}` does not implement `TypeUuid`.",
    note = "consider annotating `{Self}` with `#[derive(TypePath)]`, as `TypeUuid` is implemented for implementors of `TypePath.`"
)]
pub trait TypeUuid {
    fn type_uuid() -> u128;
}

impl<T: TypePath> TypeUuid for T {
    fn type_uuid() -> u128 {
        xxhash_rust::xxh3::xxh3_128(T::type_path().as_bytes())
    }
}

/// Dynamic dispatch for `[TypeUuid]`.
pub trait DynamicTypeUuid {
    fn type_uuid(&self) -> u128;
}

impl<T: TypeUuid> DynamicTypeUuid for T {
    fn type_uuid(&self) -> u128 {
        T::type_uuid()
    }
}
