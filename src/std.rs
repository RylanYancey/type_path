use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use crate::{TypePath, cell::GenericTypeCell};

// Array impl
impl<T: TypePath, const N: usize> TypePath for [T; N] {
    fn type_path() -> &'static str {
        static CELL: GenericTypeCell = GenericTypeCell::new();
        CELL.get_or_insert::<Self, _, _>(|| format!("[{}; {N}]", T::type_path()))
    }
}

macro_rules! impl_type_path_tuple {
    ($(($P:ident, $p:ident)),*) => {
        impl<$($P: TypePath),*> TypePath for ($($P,)*) {
            fn type_path() -> &'static str {
                static CELL: GenericTypeCell = GenericTypeCell::new();
                CELL.get_or_insert::<Self, _, _>(|| {
                    let mut result = String::new();
                    result.push('(');
                    $(result.push_str(<$P as TypePath>::type_path());)*
                    result.push(')');
                    result
                })
            }
        }
    }
}

variadics_please::all_tuples!(impl_type_path_tuple, 1, 16, T, t);

macro_rules! impl_type_path_basic {
    ($ty:ty, $val:literal) => {
        impl TypePath for $ty {
            fn type_path() -> &'static str {
                $val
            }
        }
    };
}

impl_type_path_basic!(String, "std::string::String");
impl_type_path_basic!(u8, "std::u8");
impl_type_path_basic!(u16, "std::u16");
impl_type_path_basic!(u32, "std::u32");
impl_type_path_basic!(u64, "std::u64");
impl_type_path_basic!(u128, "std::u128");
impl_type_path_basic!(usize, "std::usize");
impl_type_path_basic!(i8, "std::i8");
impl_type_path_basic!(i16, "std::i16");
impl_type_path_basic!(i32, "std::i32");
impl_type_path_basic!(i64, "std::i64");
impl_type_path_basic!(i128, "std::i128");
impl_type_path_basic!(isize, "std::isize");
impl_type_path_basic!(bool, "std::bool");
impl_type_path_basic!(char, "std::char");
impl_type_path_basic!(f32, "std::f32");
impl_type_path_basic!(f64, "std::f64");

macro_rules! impl_type_path {
    ($path:literal, $ty:ident<$($gen:ident),*>) => {
        impl<$($gen: TypePath),*> TypePath for $ty<$($gen),*> {
            fn type_path() -> &'static str {
                static CELL: GenericTypeCell = GenericTypeCell::new();
                CELL.get_or_insert::<Self, _, _>(|| {
                    let mut result = String::from($path);
                    result.push('<');
                    $(result.push_str(<$gen as TypePath>::type_path());)*
                    result.push('>');
                    result
                })
            }
        }
    };
}

impl_type_path!("std::vec::Vec", Vec<T>);
impl_type_path!("std::option::Option", Option<T>);
impl_type_path!("std::result::Result", Result<T, E>);
impl_type_path!("std::boxed::Box", Box<T>);
impl_type_path!("std::sync::Arc", Arc<T>);
impl_type_path!("std::collections::HashMap", HashMap<K, V, H>);
impl_type_path!("std::collections::HashSet", HashSet<K, H>);
