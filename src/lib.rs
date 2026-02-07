use crate::cell::GenericTypeCell;

pub mod cell;

#[cfg(feature = "uuid")]
pub mod uuid;

#[cfg(feature = "std")]
pub mod std;

#[cfg(feature = "glam")]
pub mod glam;

#[cfg(feature = "bytes")]
pub mod bytes;

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

    /// Returns the type name, without any paths on the outer or inner types.
    fn short_type_path() -> &'static str {
        static CELL: GenericTypeCell = GenericTypeCell::new();
        CELL.get_or_insert::<Self, _, _>(|| into_short_type_path(Self::type_path()))
    }
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

pub struct TypePathTable {
    pub type_path: &'static str,
    pub short_type_path: fn() -> &'static str,
}

impl TypePathTable {
    pub fn of<T: TypePath>() -> Self {
        Self {
            type_path: T::type_path(),
            short_type_path: T::short_type_path,
        }
    }

    pub fn type_path(&self) -> &'static str {
        self.type_path
    }

    pub fn short_type_path(&self) -> &'static str {
        (self.short_type_path)()
    }
}

/// Removes type paths from a string.
/// For example, `std::option::Option<std::vec::Vec<std::u32>>` becomes `Option<Vec<u32>>`.
fn into_short_type_path(s: &str) -> String {
    let mut out = String::with_capacity(s.len() / 2);
    let mut iter = s.split("::").peekable();
    while let Some(next) = iter.next() {
        if let Some(_) = iter.peek() {
            // The two characters following `next` are ::.
            if let Some(i) = next.rfind(|c: char| !c.is_alphanumeric()) {
                out.push_str(&next[..=i]);
            }
        } else {
            out.push_str(next);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::into_short_type_path;

    #[test]
    fn test_into_short_type_path() {
        assert_eq!(
            &into_short_type_path("std::vec::Vec<std::option::Option<std::u32>>"),
            "Vec<Option<u32>>"
        );

        assert_eq!(
            &into_short_type_path(
                "std::vec::Vec<std::option::Option<(std::u32, std::string::String)>>"
            ),
            "Vec<Option<(u32, String)>>"
        );

        assert_eq!(
            &into_short_type_path(
                "std::vec::Vec<std::collections::HashMap<std::string::String, std::result::Result<std::usize, std::u32>>>"
            ),
            "Vec<HashMap<String, Result<usize, u32>>>"
        );
    }
}
