use std::ops::*;

use gomath_proc_macros::rhs_ops;

use crate::element::ops::*;

use super::*;

macro_rules! rhs_op {
    ($element_trait:ident($vec2_fn:ident, $vec3_fn:ident, $vec4_fn:ident): $std_trait:ident($std_fn:ident)) => {
        impl<Rhs: Element, T: ElementDefaultImpl + $std_trait<Rhs, Output: Element>>
            $element_trait<Rhs> for T
        {
            #[inline(always)]
            fn $vec2_fn(
                vec: Vec2<Self>,
                rhs: Vec2<Rhs>,
            ) -> Vec2<<Self as $std_trait<Rhs>>::Output> {
                Vec2::from_array([vec.x().$std_fn(rhs.x()), vec.y().$std_fn(rhs.y())])
            }
            #[inline(always)]
            fn $vec3_fn(
                vec: Vec3<Self>,
                rhs: Vec3<Rhs>,
            ) -> Vec3<<Self as $std_trait<Rhs>>::Output> {
                Vec3::from_array([
                    vec.x().$std_fn(rhs.x()),
                    vec.y().$std_fn(rhs.y()),
                    vec.z().$std_fn(rhs.z()),
                ])
            }
            #[inline(always)]
            fn $vec4_fn(
                vec: Vec4<Self>,
                rhs: Vec4<Rhs>,
            ) -> Vec4<<Self as $std_trait<Rhs>>::Output> {
                Vec4::from_array([
                    vec.x().$std_fn(rhs.x()),
                    vec.y().$std_fn(rhs.y()),
                    vec.z().$std_fn(rhs.z()),
                    vec.w().$std_fn(rhs.w()),
                ])
            }
        }
    };
}
rhs_ops!(rhs_op);
