pub mod cell;

#[cfg(feature = "uuid")]
pub mod uuid;

#[cfg(feature = "std")]
pub mod std;

#[cfg(feature = "glam")]
pub mod glam;

/// A stable version of std::any::type_name, with support for generics and path overrides.
#[diagnostic::on_unimplemented(
    message = "`{Self}` does not implement `TypePath` so cannot provide static type path information",
    note = "consider annotating `{Self}` with `#[derive(TypePath)]`"
)]
pub trait TypePath: 'static {
    /// Returns the fully qualified path of the underlying type.
    ///
    /// For `Option<Vec<usize>>`, this is `"std::option::Option<std::vec::Vec<usize>>"`.
    fn type_path() -> &'static str;
}

/// Dynamic Dispatch for [`TypePath`].
#[diagnostic::on_unimplemented(
    message = "`{Self}` does not implement `TypePath` so cannot provide dynamic type path information",
    note = "consider annotating `{Self}` with `#[derive(TypePath)]`"
)]
pub trait DynamicTypePath {
    fn type_path(&self) -> &'static str;
}

impl<T: TypePath> DynamicTypePath for T {
    fn type_path(&self) -> &'static str {
        T::type_path()
    }
}
