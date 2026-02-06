use glam::*;

use crate::TypePath;

macro_rules! impl_type_path_basic {
    ($ty:ty, $val:literal) => {
        impl TypePath for $ty {
            fn type_path() -> &'static str {
                $val
            }
        }
    };
}

impl_type_path_basic!(Vec2, "glam::Vec2");
impl_type_path_basic!(Vec3, "glam::Vec3");
impl_type_path_basic!(Vec4, "glam::Vec4");
impl_type_path_basic!(Vec3A, "glam::Vec3A");

impl_type_path_basic!(DVec2, "glam::DVec2");
impl_type_path_basic!(DVec3, "glam::DVec3");
impl_type_path_basic!(DVec4, "glam::DVec4");

impl_type_path_basic!(I8Vec2, "glam::I8Vec2");
impl_type_path_basic!(I8Vec3, "glam::I8Vec3");
impl_type_path_basic!(I8Vec4, "glam::I8Vec4");

impl_type_path_basic!(I16Vec2, "glam::I16Vec2");
impl_type_path_basic!(I16Vec3, "glam::I16Vec3");
impl_type_path_basic!(I16Vec4, "glam::I16Vec4");

impl_type_path_basic!(IVec2, "glam::IVec2");
impl_type_path_basic!(IVec3, "glam::IVec3");
impl_type_path_basic!(IVec4, "glam::IVec4");

impl_type_path_basic!(I64Vec2, "glam::I64Vec2");
impl_type_path_basic!(I64Vec3, "glam::I64Vec3");
impl_type_path_basic!(I64Vec4, "glam::I64Vec4");

impl_type_path_basic!(U8Vec2, "glam::U8Vec2");
impl_type_path_basic!(U8Vec3, "glam::U8Vec3");
impl_type_path_basic!(U8Vec4, "glam::U8Vec4");

impl_type_path_basic!(U16Vec2, "glam::U16Vec2");
impl_type_path_basic!(U16Vec3, "glam::U16Vec3");
impl_type_path_basic!(U16Vec4, "glam::U16Vec4");

impl_type_path_basic!(UVec2, "glam::UVec2");
impl_type_path_basic!(UVec3, "glam::UVec3");
impl_type_path_basic!(UVec4, "glam::UVec4");

impl_type_path_basic!(U64Vec2, "glam::U64Vec2");
impl_type_path_basic!(U64Vec3, "glam::U64Vec3");
impl_type_path_basic!(U64Vec4, "glam::U64Vec4");

impl_type_path_basic!(USizeVec2, "glam::USizeVec2");
impl_type_path_basic!(USizeVec3, "glam::USizeVec3");
impl_type_path_basic!(USizeVec4, "glam::USizeVec4");

impl_type_path_basic!(BVec2, "glam::BVec2");
impl_type_path_basic!(BVec3, "glam::BVec3");
impl_type_path_basic!(BVec4, "glam::BVec4");

impl_type_path_basic!(Mat2, "glam::Mat2");
impl_type_path_basic!(Mat3, "glam::Mat3");
impl_type_path_basic!(Mat3A, "glam::Mat3A");
impl_type_path_basic!(Mat4, "glam::Mat4");

impl_type_path_basic!(Quat, "glam::Quat");
impl_type_path_basic!(Affine2, "glam::Affine2");
impl_type_path_basic!(Affine3, "glam::Affine3");
impl_type_path_basic!(Affine3A, "glam::Affine3A");
