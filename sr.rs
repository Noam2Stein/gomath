#![feature(prelude_import)]
#![warn(missing_docs)]
//! graphics-math with generics and internal SIMD.
//!
//! - Fully generic (...Vector<Len, Scalar, Storage>)
//! - Optimized with SIMD internally
//! - Simple API (...FVec2)
//! - Optimal for GPU structs
//! - Optional additional types (...Rect, Ray)
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod construct {
    //! base traits for mathamatical types.
    use std::fmt::{Debug, Display};
    /// The base trait for mathamatical types.
    ///
    /// Includes core Rust traits like [Copy] and [Display].
    /// Is automatically implemented for types that implement its supertraits.
    pub trait Construct: InnerConstruct + PartialEq + Display {}
    /// The base trait for inner mathamatical types.
    ///
    /// Includes core Rust traits needed for inner data like [Copy] and [Debug], but not outer traits like [Display].
    /// - Anything that implements [Construct] also implements [InnerConstruct].
    pub trait InnerConstruct: Sized + Send + Sync + Copy + Debug {}
    impl<T: InnerConstruct + PartialEq + Display> Construct for T {}
    impl<T: Sized + Send + Sync + Copy + Debug> InnerConstruct for T {}
}
pub mod scalar {
    //! Scalars are mathamatical types that have magnitude but no direction.
    //! - [f32] and [bool] are scalars.
    //! - [Vec3](crate::vec::Vec3) is not a scalar.
    pub mod aliases {
        //! Type aliases for primitives.
        //! For example: ```FVec3```, ```IVec2```, ```BVecN<N>```
        use crate as gomath;
        use gomath_proc_macros::scalar_aliases;
        /// mathamatical type-aliases for [```f32```]
        pub mod f32 {
            use super::*;
            /// type-aliase for an [```f32```] [```Vector```](gomath::vec::Vector)
            pub type FVector<const N: usize, S> = gomath::vec::Vector<N, f32, S>;
            /// type-aliase for an [```f32```] [```Vector2```](gomath::vec::Vector2)
            pub type FVector2<S> = gomath::vec::Vector2<f32, S>;
            /// type-aliase for an [```f32```] [```Vector3```](gomath::vec::Vector3)
            pub type FVector3<S> = gomath::vec::Vector3<f32, S>;
            /// type-aliase for an [```f32```] [```Vector4```](gomath::vec::Vector4)
            pub type FVector4<S> = gomath::vec::Vector4<f32, S>;
            /// type-aliase for an [```f32```] [```VecN```](gomath::vec::VecN)
            pub type FVecN<const N: usize> = gomath::vec::VecN<N, f32>;
            /// type-aliase for an [```f32```] [```Vec2```](gomath::vec::Vec2)
            pub type FVec2 = gomath::vec::Vec2<f32>;
            /// type-aliase for an [```f32```] [```Vec3```](gomath::vec::Vec3)
            pub type FVec3 = gomath::vec::Vec3<f32>;
            /// type-aliase for an [```f32```] [```Vec4```](gomath::vec::Vec4)
            pub type FVec4 = gomath::vec::Vec4<f32>;
            /// type-aliase for an [```f32```] [```VecNP```](gomath::vec::VecNP)
            pub type FVecNP<const N: usize> = gomath::vec::VecNP<N, f32>;
            /// type-aliase for an [```f32```] [```Vec2P```](gomath::vec::Vec2P)
            pub type FVec2P = gomath::vec::Vec2P<f32>;
            /// type-aliase for an [```f32```] [```Vec3P```](gomath::vec::Vec3P)
            pub type FVec3P = gomath::vec::Vec3P<f32>;
            /// type-aliase for an [```f32```] [```Vec4P```](gomath::vec::Vec4P)
            pub type FVec4P = gomath::vec::Vec4P<f32>;
        }
        /// mathamatical type-aliases for [```f64```]
        pub mod f64 {
            use super::*;
            /// type-aliase for an [```f64```] [```Vector```](gomath::vec::Vector)
            pub type DVector<const N: usize, S> = gomath::vec::Vector<N, f64, S>;
            /// type-aliase for an [```f64```] [```Vector2```](gomath::vec::Vector2)
            pub type DVector2<S> = gomath::vec::Vector2<f64, S>;
            /// type-aliase for an [```f64```] [```Vector3```](gomath::vec::Vector3)
            pub type DVector3<S> = gomath::vec::Vector3<f64, S>;
            /// type-aliase for an [```f64```] [```Vector4```](gomath::vec::Vector4)
            pub type DVector4<S> = gomath::vec::Vector4<f64, S>;
            /// type-aliase for an [```f64```] [```VecN```](gomath::vec::VecN)
            pub type DVecN<const N: usize> = gomath::vec::VecN<N, f64>;
            /// type-aliase for an [```f64```] [```Vec2```](gomath::vec::Vec2)
            pub type DVec2 = gomath::vec::Vec2<f64>;
            /// type-aliase for an [```f64```] [```Vec3```](gomath::vec::Vec3)
            pub type DVec3 = gomath::vec::Vec3<f64>;
            /// type-aliase for an [```f64```] [```Vec4```](gomath::vec::Vec4)
            pub type DVec4 = gomath::vec::Vec4<f64>;
            /// type-aliase for an [```f64```] [```VecNP```](gomath::vec::VecNP)
            pub type DVecNP<const N: usize> = gomath::vec::VecNP<N, f64>;
            /// type-aliase for an [```f64```] [```Vec2P```](gomath::vec::Vec2P)
            pub type DVec2P = gomath::vec::Vec2P<f64>;
            /// type-aliase for an [```f64```] [```Vec3P```](gomath::vec::Vec3P)
            pub type DVec3P = gomath::vec::Vec3P<f64>;
            /// type-aliase for an [```f64```] [```Vec4P```](gomath::vec::Vec4P)
            pub type DVec4P = gomath::vec::Vec4P<f64>;
        }
        /// mathamatical type-aliases for [```u8```]
        pub mod u8 {
            use super::*;
            /// type-aliase for an [```u8```] [```Vector```](gomath::vec::Vector)
            pub type U8Vector<const N: usize, S> = gomath::vec::Vector<N, u8, S>;
            /// type-aliase for an [```u8```] [```Vector2```](gomath::vec::Vector2)
            pub type U8Vector2<S> = gomath::vec::Vector2<u8, S>;
            /// type-aliase for an [```u8```] [```Vector3```](gomath::vec::Vector3)
            pub type U8Vector3<S> = gomath::vec::Vector3<u8, S>;
            /// type-aliase for an [```u8```] [```Vector4```](gomath::vec::Vector4)
            pub type U8Vector4<S> = gomath::vec::Vector4<u8, S>;
            /// type-aliase for an [```u8```] [```VecN```](gomath::vec::VecN)
            pub type U8VecN<const N: usize> = gomath::vec::VecN<N, u8>;
            /// type-aliase for an [```u8```] [```Vec2```](gomath::vec::Vec2)
            pub type U8Vec2 = gomath::vec::Vec2<u8>;
            /// type-aliase for an [```u8```] [```Vec3```](gomath::vec::Vec3)
            pub type U8Vec3 = gomath::vec::Vec3<u8>;
            /// type-aliase for an [```u8```] [```Vec4```](gomath::vec::Vec4)
            pub type U8Vec4 = gomath::vec::Vec4<u8>;
            /// type-aliase for an [```u8```] [```VecNP```](gomath::vec::VecNP)
            pub type U8VecNP<const N: usize> = gomath::vec::VecNP<N, u8>;
            /// type-aliase for an [```u8```] [```Vec2P```](gomath::vec::Vec2P)
            pub type U8Vec2P = gomath::vec::Vec2P<u8>;
            /// type-aliase for an [```u8```] [```Vec3P```](gomath::vec::Vec3P)
            pub type U8Vec3P = gomath::vec::Vec3P<u8>;
            /// type-aliase for an [```u8```] [```Vec4P```](gomath::vec::Vec4P)
            pub type U8Vec4P = gomath::vec::Vec4P<u8>;
        }
        /// mathamatical type-aliases for [```u16```]
        pub mod u16 {
            use super::*;
            /// type-aliase for an [```u16```] [```Vector```](gomath::vec::Vector)
            pub type U16Vector<const N: usize, S> = gomath::vec::Vector<N, u16, S>;
            /// type-aliase for an [```u16```] [```Vector2```](gomath::vec::Vector2)
            pub type U16Vector2<S> = gomath::vec::Vector2<u16, S>;
            /// type-aliase for an [```u16```] [```Vector3```](gomath::vec::Vector3)
            pub type U16Vector3<S> = gomath::vec::Vector3<u16, S>;
            /// type-aliase for an [```u16```] [```Vector4```](gomath::vec::Vector4)
            pub type U16Vector4<S> = gomath::vec::Vector4<u16, S>;
            /// type-aliase for an [```u16```] [```VecN```](gomath::vec::VecN)
            pub type U16VecN<const N: usize> = gomath::vec::VecN<N, u16>;
            /// type-aliase for an [```u16```] [```Vec2```](gomath::vec::Vec2)
            pub type U16Vec2 = gomath::vec::Vec2<u16>;
            /// type-aliase for an [```u16```] [```Vec3```](gomath::vec::Vec3)
            pub type U16Vec3 = gomath::vec::Vec3<u16>;
            /// type-aliase for an [```u16```] [```Vec4```](gomath::vec::Vec4)
            pub type U16Vec4 = gomath::vec::Vec4<u16>;
            /// type-aliase for an [```u16```] [```VecNP```](gomath::vec::VecNP)
            pub type U16VecNP<const N: usize> = gomath::vec::VecNP<N, u16>;
            /// type-aliase for an [```u16```] [```Vec2P```](gomath::vec::Vec2P)
            pub type U16Vec2P = gomath::vec::Vec2P<u16>;
            /// type-aliase for an [```u16```] [```Vec3P```](gomath::vec::Vec3P)
            pub type U16Vec3P = gomath::vec::Vec3P<u16>;
            /// type-aliase for an [```u16```] [```Vec4P```](gomath::vec::Vec4P)
            pub type U16Vec4P = gomath::vec::Vec4P<u16>;
        }
        /// mathamatical type-aliases for [```u32```]
        pub mod u32 {
            use super::*;
            /// type-aliase for an [```u32```] [```Vector```](gomath::vec::Vector)
            pub type UVector<const N: usize, S> = gomath::vec::Vector<N, u32, S>;
            /// type-aliase for an [```u32```] [```Vector2```](gomath::vec::Vector2)
            pub type UVector2<S> = gomath::vec::Vector2<u32, S>;
            /// type-aliase for an [```u32```] [```Vector3```](gomath::vec::Vector3)
            pub type UVector3<S> = gomath::vec::Vector3<u32, S>;
            /// type-aliase for an [```u32```] [```Vector4```](gomath::vec::Vector4)
            pub type UVector4<S> = gomath::vec::Vector4<u32, S>;
            /// type-aliase for an [```u32```] [```VecN```](gomath::vec::VecN)
            pub type UVecN<const N: usize> = gomath::vec::VecN<N, u32>;
            /// type-aliase for an [```u32```] [```Vec2```](gomath::vec::Vec2)
            pub type UVec2 = gomath::vec::Vec2<u32>;
            /// type-aliase for an [```u32```] [```Vec3```](gomath::vec::Vec3)
            pub type UVec3 = gomath::vec::Vec3<u32>;
            /// type-aliase for an [```u32```] [```Vec4```](gomath::vec::Vec4)
            pub type UVec4 = gomath::vec::Vec4<u32>;
            /// type-aliase for an [```u32```] [```VecNP```](gomath::vec::VecNP)
            pub type UVecNP<const N: usize> = gomath::vec::VecNP<N, u32>;
            /// type-aliase for an [```u32```] [```Vec2P```](gomath::vec::Vec2P)
            pub type UVec2P = gomath::vec::Vec2P<u32>;
            /// type-aliase for an [```u32```] [```Vec3P```](gomath::vec::Vec3P)
            pub type UVec3P = gomath::vec::Vec3P<u32>;
            /// type-aliase for an [```u32```] [```Vec4P```](gomath::vec::Vec4P)
            pub type UVec4P = gomath::vec::Vec4P<u32>;
        }
        /// mathamatical type-aliases for [```u64```]
        pub mod u64 {
            use super::*;
            /// type-aliase for an [```u64```] [```Vector```](gomath::vec::Vector)
            pub type U64Vector<const N: usize, S> = gomath::vec::Vector<N, u64, S>;
            /// type-aliase for an [```u64```] [```Vector2```](gomath::vec::Vector2)
            pub type U64Vector2<S> = gomath::vec::Vector2<u64, S>;
            /// type-aliase for an [```u64```] [```Vector3```](gomath::vec::Vector3)
            pub type U64Vector3<S> = gomath::vec::Vector3<u64, S>;
            /// type-aliase for an [```u64```] [```Vector4```](gomath::vec::Vector4)
            pub type U64Vector4<S> = gomath::vec::Vector4<u64, S>;
            /// type-aliase for an [```u64```] [```VecN```](gomath::vec::VecN)
            pub type U64VecN<const N: usize> = gomath::vec::VecN<N, u64>;
            /// type-aliase for an [```u64```] [```Vec2```](gomath::vec::Vec2)
            pub type U64Vec2 = gomath::vec::Vec2<u64>;
            /// type-aliase for an [```u64```] [```Vec3```](gomath::vec::Vec3)
            pub type U64Vec3 = gomath::vec::Vec3<u64>;
            /// type-aliase for an [```u64```] [```Vec4```](gomath::vec::Vec4)
            pub type U64Vec4 = gomath::vec::Vec4<u64>;
            /// type-aliase for an [```u64```] [```VecNP```](gomath::vec::VecNP)
            pub type U64VecNP<const N: usize> = gomath::vec::VecNP<N, u64>;
            /// type-aliase for an [```u64```] [```Vec2P```](gomath::vec::Vec2P)
            pub type U64Vec2P = gomath::vec::Vec2P<u64>;
            /// type-aliase for an [```u64```] [```Vec3P```](gomath::vec::Vec3P)
            pub type U64Vec3P = gomath::vec::Vec3P<u64>;
            /// type-aliase for an [```u64```] [```Vec4P```](gomath::vec::Vec4P)
            pub type U64Vec4P = gomath::vec::Vec4P<u64>;
        }
        /// mathamatical type-aliases for [```u128```]
        pub mod u128 {
            use super::*;
            /// type-aliase for an [```u128```] [```Vector```](gomath::vec::Vector)
            pub type U128Vector<const N: usize, S> = gomath::vec::Vector<N, u128, S>;
            /// type-aliase for an [```u128```] [```Vector2```](gomath::vec::Vector2)
            pub type U128Vector2<S> = gomath::vec::Vector2<u128, S>;
            /// type-aliase for an [```u128```] [```Vector3```](gomath::vec::Vector3)
            pub type U128Vector3<S> = gomath::vec::Vector3<u128, S>;
            /// type-aliase for an [```u128```] [```Vector4```](gomath::vec::Vector4)
            pub type U128Vector4<S> = gomath::vec::Vector4<u128, S>;
            /// type-aliase for an [```u128```] [```VecN```](gomath::vec::VecN)
            pub type U128VecN<const N: usize> = gomath::vec::VecN<N, u128>;
            /// type-aliase for an [```u128```] [```Vec2```](gomath::vec::Vec2)
            pub type U128Vec2 = gomath::vec::Vec2<u128>;
            /// type-aliase for an [```u128```] [```Vec3```](gomath::vec::Vec3)
            pub type U128Vec3 = gomath::vec::Vec3<u128>;
            /// type-aliase for an [```u128```] [```Vec4```](gomath::vec::Vec4)
            pub type U128Vec4 = gomath::vec::Vec4<u128>;
            /// type-aliase for an [```u128```] [```VecNP```](gomath::vec::VecNP)
            pub type U128VecNP<const N: usize> = gomath::vec::VecNP<N, u128>;
            /// type-aliase for an [```u128```] [```Vec2P```](gomath::vec::Vec2P)
            pub type U128Vec2P = gomath::vec::Vec2P<u128>;
            /// type-aliase for an [```u128```] [```Vec3P```](gomath::vec::Vec3P)
            pub type U128Vec3P = gomath::vec::Vec3P<u128>;
            /// type-aliase for an [```u128```] [```Vec4P```](gomath::vec::Vec4P)
            pub type U128Vec4P = gomath::vec::Vec4P<u128>;
        }
        /// mathamatical type-aliases for [```usize```]
        pub mod usize {
            use super::*;
            /// type-aliase for an [```usize```] [```Vector```](gomath::vec::Vector)
            pub type USizeVector<const N: usize, S> = gomath::vec::Vector<N, usize, S>;
            /// type-aliase for an [```usize```] [```Vector2```](gomath::vec::Vector2)
            pub type USizeVector2<S> = gomath::vec::Vector2<usize, S>;
            /// type-aliase for an [```usize```] [```Vector3```](gomath::vec::Vector3)
            pub type USizeVector3<S> = gomath::vec::Vector3<usize, S>;
            /// type-aliase for an [```usize```] [```Vector4```](gomath::vec::Vector4)
            pub type USizeVector4<S> = gomath::vec::Vector4<usize, S>;
            /// type-aliase for an [```usize```] [```VecN```](gomath::vec::VecN)
            pub type USizeVecN<const N: usize> = gomath::vec::VecN<N, usize>;
            /// type-aliase for an [```usize```] [```Vec2```](gomath::vec::Vec2)
            pub type USizeVec2 = gomath::vec::Vec2<usize>;
            /// type-aliase for an [```usize```] [```Vec3```](gomath::vec::Vec3)
            pub type USizeVec3 = gomath::vec::Vec3<usize>;
            /// type-aliase for an [```usize```] [```Vec4```](gomath::vec::Vec4)
            pub type USizeVec4 = gomath::vec::Vec4<usize>;
            /// type-aliase for an [```usize```] [```VecNP```](gomath::vec::VecNP)
            pub type USizeVecNP<const N: usize> = gomath::vec::VecNP<N, usize>;
            /// type-aliase for an [```usize```] [```Vec2P```](gomath::vec::Vec2P)
            pub type USizeVec2P = gomath::vec::Vec2P<usize>;
            /// type-aliase for an [```usize```] [```Vec3P```](gomath::vec::Vec3P)
            pub type USizeVec3P = gomath::vec::Vec3P<usize>;
            /// type-aliase for an [```usize```] [```Vec4P```](gomath::vec::Vec4P)
            pub type USizeVec4P = gomath::vec::Vec4P<usize>;
        }
        /// mathamatical type-aliases for [```i8```]
        pub mod i8 {
            use super::*;
            /// type-aliase for an [```i8```] [```Vector```](gomath::vec::Vector)
            pub type I8Vector<const N: usize, S> = gomath::vec::Vector<N, i8, S>;
            /// type-aliase for an [```i8```] [```Vector2```](gomath::vec::Vector2)
            pub type I8Vector2<S> = gomath::vec::Vector2<i8, S>;
            /// type-aliase for an [```i8```] [```Vector3```](gomath::vec::Vector3)
            pub type I8Vector3<S> = gomath::vec::Vector3<i8, S>;
            /// type-aliase for an [```i8```] [```Vector4```](gomath::vec::Vector4)
            pub type I8Vector4<S> = gomath::vec::Vector4<i8, S>;
            /// type-aliase for an [```i8```] [```VecN```](gomath::vec::VecN)
            pub type I8VecN<const N: usize> = gomath::vec::VecN<N, i8>;
            /// type-aliase for an [```i8```] [```Vec2```](gomath::vec::Vec2)
            pub type I8Vec2 = gomath::vec::Vec2<i8>;
            /// type-aliase for an [```i8```] [```Vec3```](gomath::vec::Vec3)
            pub type I8Vec3 = gomath::vec::Vec3<i8>;
            /// type-aliase for an [```i8```] [```Vec4```](gomath::vec::Vec4)
            pub type I8Vec4 = gomath::vec::Vec4<i8>;
            /// type-aliase for an [```i8```] [```VecNP```](gomath::vec::VecNP)
            pub type I8VecNP<const N: usize> = gomath::vec::VecNP<N, i8>;
            /// type-aliase for an [```i8```] [```Vec2P```](gomath::vec::Vec2P)
            pub type I8Vec2P = gomath::vec::Vec2P<i8>;
            /// type-aliase for an [```i8```] [```Vec3P```](gomath::vec::Vec3P)
            pub type I8Vec3P = gomath::vec::Vec3P<i8>;
            /// type-aliase for an [```i8```] [```Vec4P```](gomath::vec::Vec4P)
            pub type I8Vec4P = gomath::vec::Vec4P<i8>;
        }
        /// mathamatical type-aliases for [```i16```]
        pub mod i16 {
            use super::*;
            /// type-aliase for an [```i16```] [```Vector```](gomath::vec::Vector)
            pub type I16Vector<const N: usize, S> = gomath::vec::Vector<N, i16, S>;
            /// type-aliase for an [```i16```] [```Vector2```](gomath::vec::Vector2)
            pub type I16Vector2<S> = gomath::vec::Vector2<i16, S>;
            /// type-aliase for an [```i16```] [```Vector3```](gomath::vec::Vector3)
            pub type I16Vector3<S> = gomath::vec::Vector3<i16, S>;
            /// type-aliase for an [```i16```] [```Vector4```](gomath::vec::Vector4)
            pub type I16Vector4<S> = gomath::vec::Vector4<i16, S>;
            /// type-aliase for an [```i16```] [```VecN```](gomath::vec::VecN)
            pub type I16VecN<const N: usize> = gomath::vec::VecN<N, i16>;
            /// type-aliase for an [```i16```] [```Vec2```](gomath::vec::Vec2)
            pub type I16Vec2 = gomath::vec::Vec2<i16>;
            /// type-aliase for an [```i16```] [```Vec3```](gomath::vec::Vec3)
            pub type I16Vec3 = gomath::vec::Vec3<i16>;
            /// type-aliase for an [```i16```] [```Vec4```](gomath::vec::Vec4)
            pub type I16Vec4 = gomath::vec::Vec4<i16>;
            /// type-aliase for an [```i16```] [```VecNP```](gomath::vec::VecNP)
            pub type I16VecNP<const N: usize> = gomath::vec::VecNP<N, i16>;
            /// type-aliase for an [```i16```] [```Vec2P```](gomath::vec::Vec2P)
            pub type I16Vec2P = gomath::vec::Vec2P<i16>;
            /// type-aliase for an [```i16```] [```Vec3P```](gomath::vec::Vec3P)
            pub type I16Vec3P = gomath::vec::Vec3P<i16>;
            /// type-aliase for an [```i16```] [```Vec4P```](gomath::vec::Vec4P)
            pub type I16Vec4P = gomath::vec::Vec4P<i16>;
        }
        /// mathamatical type-aliases for [```i32```]
        pub mod i32 {
            use super::*;
            /// type-aliase for an [```i32```] [```Vector```](gomath::vec::Vector)
            pub type IVector<const N: usize, S> = gomath::vec::Vector<N, i32, S>;
            /// type-aliase for an [```i32```] [```Vector2```](gomath::vec::Vector2)
            pub type IVector2<S> = gomath::vec::Vector2<i32, S>;
            /// type-aliase for an [```i32```] [```Vector3```](gomath::vec::Vector3)
            pub type IVector3<S> = gomath::vec::Vector3<i32, S>;
            /// type-aliase for an [```i32```] [```Vector4```](gomath::vec::Vector4)
            pub type IVector4<S> = gomath::vec::Vector4<i32, S>;
            /// type-aliase for an [```i32```] [```VecN```](gomath::vec::VecN)
            pub type IVecN<const N: usize> = gomath::vec::VecN<N, i32>;
            /// type-aliase for an [```i32```] [```Vec2```](gomath::vec::Vec2)
            pub type IVec2 = gomath::vec::Vec2<i32>;
            /// type-aliase for an [```i32```] [```Vec3```](gomath::vec::Vec3)
            pub type IVec3 = gomath::vec::Vec3<i32>;
            /// type-aliase for an [```i32```] [```Vec4```](gomath::vec::Vec4)
            pub type IVec4 = gomath::vec::Vec4<i32>;
            /// type-aliase for an [```i32```] [```VecNP```](gomath::vec::VecNP)
            pub type IVecNP<const N: usize> = gomath::vec::VecNP<N, i32>;
            /// type-aliase for an [```i32```] [```Vec2P```](gomath::vec::Vec2P)
            pub type IVec2P = gomath::vec::Vec2P<i32>;
            /// type-aliase for an [```i32```] [```Vec3P```](gomath::vec::Vec3P)
            pub type IVec3P = gomath::vec::Vec3P<i32>;
            /// type-aliase for an [```i32```] [```Vec4P```](gomath::vec::Vec4P)
            pub type IVec4P = gomath::vec::Vec4P<i32>;
        }
        /// mathamatical type-aliases for [```i64```]
        pub mod i64 {
            use super::*;
            /// type-aliase for an [```i64```] [```Vector```](gomath::vec::Vector)
            pub type I64Vector<const N: usize, S> = gomath::vec::Vector<N, i64, S>;
            /// type-aliase for an [```i64```] [```Vector2```](gomath::vec::Vector2)
            pub type I64Vector2<S> = gomath::vec::Vector2<i64, S>;
            /// type-aliase for an [```i64```] [```Vector3```](gomath::vec::Vector3)
            pub type I64Vector3<S> = gomath::vec::Vector3<i64, S>;
            /// type-aliase for an [```i64```] [```Vector4```](gomath::vec::Vector4)
            pub type I64Vector4<S> = gomath::vec::Vector4<i64, S>;
            /// type-aliase for an [```i64```] [```VecN```](gomath::vec::VecN)
            pub type I64VecN<const N: usize> = gomath::vec::VecN<N, i64>;
            /// type-aliase for an [```i64```] [```Vec2```](gomath::vec::Vec2)
            pub type I64Vec2 = gomath::vec::Vec2<i64>;
            /// type-aliase for an [```i64```] [```Vec3```](gomath::vec::Vec3)
            pub type I64Vec3 = gomath::vec::Vec3<i64>;
            /// type-aliase for an [```i64```] [```Vec4```](gomath::vec::Vec4)
            pub type I64Vec4 = gomath::vec::Vec4<i64>;
            /// type-aliase for an [```i64```] [```VecNP```](gomath::vec::VecNP)
            pub type I64VecNP<const N: usize> = gomath::vec::VecNP<N, i64>;
            /// type-aliase for an [```i64```] [```Vec2P```](gomath::vec::Vec2P)
            pub type I64Vec2P = gomath::vec::Vec2P<i64>;
            /// type-aliase for an [```i64```] [```Vec3P```](gomath::vec::Vec3P)
            pub type I64Vec3P = gomath::vec::Vec3P<i64>;
            /// type-aliase for an [```i64```] [```Vec4P```](gomath::vec::Vec4P)
            pub type I64Vec4P = gomath::vec::Vec4P<i64>;
        }
        /// mathamatical type-aliases for [```i128```]
        pub mod i128 {
            use super::*;
            /// type-aliase for an [```i128```] [```Vector```](gomath::vec::Vector)
            pub type I128Vector<const N: usize, S> = gomath::vec::Vector<N, i128, S>;
            /// type-aliase for an [```i128```] [```Vector2```](gomath::vec::Vector2)
            pub type I128Vector2<S> = gomath::vec::Vector2<i128, S>;
            /// type-aliase for an [```i128```] [```Vector3```](gomath::vec::Vector3)
            pub type I128Vector3<S> = gomath::vec::Vector3<i128, S>;
            /// type-aliase for an [```i128```] [```Vector4```](gomath::vec::Vector4)
            pub type I128Vector4<S> = gomath::vec::Vector4<i128, S>;
            /// type-aliase for an [```i128```] [```VecN```](gomath::vec::VecN)
            pub type I128VecN<const N: usize> = gomath::vec::VecN<N, i128>;
            /// type-aliase for an [```i128```] [```Vec2```](gomath::vec::Vec2)
            pub type I128Vec2 = gomath::vec::Vec2<i128>;
            /// type-aliase for an [```i128```] [```Vec3```](gomath::vec::Vec3)
            pub type I128Vec3 = gomath::vec::Vec3<i128>;
            /// type-aliase for an [```i128```] [```Vec4```](gomath::vec::Vec4)
            pub type I128Vec4 = gomath::vec::Vec4<i128>;
            /// type-aliase for an [```i128```] [```VecNP```](gomath::vec::VecNP)
            pub type I128VecNP<const N: usize> = gomath::vec::VecNP<N, i128>;
            /// type-aliase for an [```i128```] [```Vec2P```](gomath::vec::Vec2P)
            pub type I128Vec2P = gomath::vec::Vec2P<i128>;
            /// type-aliase for an [```i128```] [```Vec3P```](gomath::vec::Vec3P)
            pub type I128Vec3P = gomath::vec::Vec3P<i128>;
            /// type-aliase for an [```i128```] [```Vec4P```](gomath::vec::Vec4P)
            pub type I128Vec4P = gomath::vec::Vec4P<i128>;
        }
        /// mathamatical type-aliases for [```isize```]
        pub mod isize {
            use super::*;
            /// type-aliase for an [```isize```] [```Vector```](gomath::vec::Vector)
            pub type ISizeVector<const N: usize, S> = gomath::vec::Vector<N, isize, S>;
            /// type-aliase for an [```isize```] [```Vector2```](gomath::vec::Vector2)
            pub type ISizeVector2<S> = gomath::vec::Vector2<isize, S>;
            /// type-aliase for an [```isize```] [```Vector3```](gomath::vec::Vector3)
            pub type ISizeVector3<S> = gomath::vec::Vector3<isize, S>;
            /// type-aliase for an [```isize```] [```Vector4```](gomath::vec::Vector4)
            pub type ISizeVector4<S> = gomath::vec::Vector4<isize, S>;
            /// type-aliase for an [```isize```] [```VecN```](gomath::vec::VecN)
            pub type ISizeVecN<const N: usize> = gomath::vec::VecN<N, isize>;
            /// type-aliase for an [```isize```] [```Vec2```](gomath::vec::Vec2)
            pub type ISizeVec2 = gomath::vec::Vec2<isize>;
            /// type-aliase for an [```isize```] [```Vec3```](gomath::vec::Vec3)
            pub type ISizeVec3 = gomath::vec::Vec3<isize>;
            /// type-aliase for an [```isize```] [```Vec4```](gomath::vec::Vec4)
            pub type ISizeVec4 = gomath::vec::Vec4<isize>;
            /// type-aliase for an [```isize```] [```VecNP```](gomath::vec::VecNP)
            pub type ISizeVecNP<const N: usize> = gomath::vec::VecNP<N, isize>;
            /// type-aliase for an [```isize```] [```Vec2P```](gomath::vec::Vec2P)
            pub type ISizeVec2P = gomath::vec::Vec2P<isize>;
            /// type-aliase for an [```isize```] [```Vec3P```](gomath::vec::Vec3P)
            pub type ISizeVec3P = gomath::vec::Vec3P<isize>;
            /// type-aliase for an [```isize```] [```Vec4P```](gomath::vec::Vec4P)
            pub type ISizeVec4P = gomath::vec::Vec4P<isize>;
        }
        /// mathamatical type-aliases for [```bool```]
        pub mod bool {
            use super::*;
            /// type-aliase for an [```bool```] [```Vector```](gomath::vec::Vector)
            pub type BVector<const N: usize, S> = gomath::vec::Vector<N, bool, S>;
            /// type-aliase for an [```bool```] [```Vector2```](gomath::vec::Vector2)
            pub type BVector2<S> = gomath::vec::Vector2<bool, S>;
            /// type-aliase for an [```bool```] [```Vector3```](gomath::vec::Vector3)
            pub type BVector3<S> = gomath::vec::Vector3<bool, S>;
            /// type-aliase for an [```bool```] [```Vector4```](gomath::vec::Vector4)
            pub type BVector4<S> = gomath::vec::Vector4<bool, S>;
            /// type-aliase for an [```bool```] [```VecN```](gomath::vec::VecN)
            pub type BVecN<const N: usize> = gomath::vec::VecN<N, bool>;
            /// type-aliase for an [```bool```] [```Vec2```](gomath::vec::Vec2)
            pub type BVec2 = gomath::vec::Vec2<bool>;
            /// type-aliase for an [```bool```] [```Vec3```](gomath::vec::Vec3)
            pub type BVec3 = gomath::vec::Vec3<bool>;
            /// type-aliase for an [```bool```] [```Vec4```](gomath::vec::Vec4)
            pub type BVec4 = gomath::vec::Vec4<bool>;
            /// type-aliase for an [```bool```] [```VecNP```](gomath::vec::VecNP)
            pub type BVecNP<const N: usize> = gomath::vec::VecNP<N, bool>;
            /// type-aliase for an [```bool```] [```Vec2P```](gomath::vec::Vec2P)
            pub type BVec2P = gomath::vec::Vec2P<bool>;
            /// type-aliase for an [```bool```] [```Vec3P```](gomath::vec::Vec3P)
            pub type BVec3P = gomath::vec::Vec3P<bool>;
            /// type-aliase for an [```bool```] [```Vec4P```](gomath::vec::Vec4P)
            pub type BVec4P = gomath::vec::Vec4P<bool>;
        }
    }
    pub mod default_impl {
        //! A default implementation for Scalar that doesn't use SIMD.
        //!
        //! Use the [scalar_default_impl] macro.
        use crate::vec::inner::*;
        use super::*;
        mod vec {
            use crate::vec::{api::*, *};
            use super::*;
            mod array {
                use std::mem::transmute_copy;
                use super::*;
                impl<T: ScalarDefaultImpl> ScalarVecArrayApi<2, VecAligned> for T {
                    fn from_array(array: [Self; 2]) -> InnerVector<2, Self, VecAligned> {
                        unsafe { transmute_copy(&array) }
                    }
                }
                impl<T: ScalarDefaultImpl> ScalarVecArrayApi<3, VecAligned> for T {
                    fn from_array(array: [Self; 3]) -> InnerVector<3, Self, VecAligned> {
                        unsafe {
                            transmute_copy(&[array[0], array[1], array[2], array[2]])
                        }
                    }
                }
                impl<T: ScalarDefaultImpl> ScalarVecArrayApi<4, VecAligned> for T {
                    fn from_array(array: [Self; 4]) -> InnerVector<4, Self, VecAligned> {
                        unsafe { transmute_copy(&array) }
                    }
                }
            }
            impl<T: ScalarDefaultImpl> ScalarVecApi<2, VecPacked> for T {}
            impl<T: ScalarDefaultImpl> ScalarVecApi<3, VecPacked> for T {}
            impl<T: ScalarDefaultImpl> ScalarVecApi<4, VecPacked> for T {}
            impl<T: ScalarDefaultImpl> ScalarVecApi<2, VecAligned> for T {}
            impl<T: ScalarDefaultImpl> ScalarVecApi<3, VecAligned> for T {}
            impl<T: ScalarDefaultImpl> ScalarVecApi<4, VecAligned> for T {}
        }
        pub use gomath_proc_macros::scalar_default_impl;
        /// Automatically implements Scalar using a default implementation.
        /// Has scalar supertraits that can't be implemented automatically by generics.
        ///
        /// Use [scalar_default_impl] instead.
        pub trait ScalarDefaultImpl: Construct + PartialOrd + ScalarInnerVecs {}
        impl<T: ScalarDefaultImpl> Scalar for T {}
    }
    mod primitive_impls {
        mod bool {
            use crate as gomath;
            use crate::scalar::default_impl::scalar_default_impl;
            impl gomath::scalar::default_impl::ScalarDefaultImpl for bool {}
            mod inner_vecs_6861707285520076819 {
                use super::*;
                unsafe impl gomath::vec::inner::ScalarInnerVecs for bool {
                    type InnerAlignedVec2 = InnerAlignedVec2;
                    type InnerAlignedVec4 = InnerAlignedVec4;
                }
                #[repr(align(2))]
                pub struct InnerAlignedVec2([bool; 2]);
                #[automatically_derived]
                impl ::core::fmt::Debug for InnerAlignedVec2 {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "InnerAlignedVec2",
                            &&self.0,
                        )
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for InnerAlignedVec2 {
                    #[inline]
                    fn clone(&self) -> InnerAlignedVec2 {
                        let _: ::core::clone::AssertParamIsClone<[bool; 2]>;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for InnerAlignedVec2 {}
                #[repr(align(4))]
                pub struct InnerAlignedVec4([bool; 4]);
                #[automatically_derived]
                impl ::core::fmt::Debug for InnerAlignedVec4 {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "InnerAlignedVec4",
                            &&self.0,
                        )
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for InnerAlignedVec4 {
                    #[inline]
                    fn clone(&self) -> InnerAlignedVec4 {
                        let _: ::core::clone::AssertParamIsClone<[bool; 4]>;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for InnerAlignedVec4 {}
                const _: () = if !(size_of::<bool>() == 1) {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "the provided size for bool: 1 bytes, is not its size",
                            ),
                        );
                    }
                };
                const _: () = if !(size_of::<InnerAlignedVec2>() == 1 * 2) {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "requirement not met: size_of<InnerAlignedVec2> == size_of<T> * 2",
                            ),
                        );
                    }
                };
                const _: () = if !(align_of::<InnerAlignedVec2>() == 1 * 2) {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "requirement not met: align_of<InnerAlignedVec2> == size_of<T> * 2",
                            ),
                        );
                    }
                };
                const _: () = if !(size_of::<InnerAlignedVec4>() == 1 * 4) {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "requirement not met: size_of<InnerAlignedVec4> == size_of<T> * 4",
                            ),
                        );
                    }
                };
                const _: () = if !(align_of::<InnerAlignedVec4>() == 1 * 4) {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "requirement not met: align_of<InnerAlignedVec4> == size_of<T> * 4",
                            ),
                        );
                    }
                };
            }
        }
        mod floats {
            mod f32 {
                use crate as gomath;
                use crate::scalar::default_impl::scalar_default_impl;
                impl gomath::scalar::default_impl::ScalarDefaultImpl for f32 {}
                mod inner_vecs_14215670250043581766 {
                    use super::*;
                    unsafe impl gomath::vec::inner::ScalarInnerVecs for f32 {
                        type InnerAlignedVec2 = InnerAlignedVec2;
                        type InnerAlignedVec4 = InnerAlignedVec4;
                    }
                    #[repr(align(8))]
                    pub struct InnerAlignedVec2([f32; 2]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec2 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec2",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec2 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec2 {
                            let _: ::core::clone::AssertParamIsClone<[f32; 2]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec2 {}
                    #[repr(align(16))]
                    pub struct InnerAlignedVec4([f32; 4]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec4 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec4",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec4 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec4 {
                            let _: ::core::clone::AssertParamIsClone<[f32; 4]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec4 {}
                    const _: () = if !(size_of::<f32>() == 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "the provided size for f32: 4 bytes, is not its size",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec2>() == 4 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec2>() == 4 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec4>() == 4 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec4>() == 4 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                }
            }
            mod f64 {
                use crate as gomath;
                use crate::scalar::default_impl::scalar_default_impl;
                impl gomath::scalar::default_impl::ScalarDefaultImpl for f64 {}
                mod inner_vecs_11668025185570632548 {
                    use super::*;
                    unsafe impl gomath::vec::inner::ScalarInnerVecs for f64 {
                        type InnerAlignedVec2 = InnerAlignedVec2;
                        type InnerAlignedVec4 = InnerAlignedVec4;
                    }
                    #[repr(align(16))]
                    pub struct InnerAlignedVec2([f64; 2]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec2 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec2",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec2 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec2 {
                            let _: ::core::clone::AssertParamIsClone<[f64; 2]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec2 {}
                    #[repr(align(32))]
                    pub struct InnerAlignedVec4([f64; 4]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec4 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec4",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec4 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec4 {
                            let _: ::core::clone::AssertParamIsClone<[f64; 4]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec4 {}
                    const _: () = if !(size_of::<f64>() == 8) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "the provided size for f64: 8 bytes, is not its size",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec2>() == 8 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec2>() == 8 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec4>() == 8 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec4>() == 8 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                }
            }
        }
        mod signed_ints {
            mod i128 {
                use crate as gomath;
                use crate::scalar::default_impl::scalar_default_impl;
                impl gomath::scalar::default_impl::ScalarDefaultImpl for i128 {}
                mod inner_vecs_3887899319199191083 {
                    use super::*;
                    unsafe impl gomath::vec::inner::ScalarInnerVecs for i128 {
                        type InnerAlignedVec2 = InnerAlignedVec2;
                        type InnerAlignedVec4 = InnerAlignedVec4;
                    }
                    #[repr(align(32))]
                    pub struct InnerAlignedVec2([i128; 2]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec2 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec2",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec2 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec2 {
                            let _: ::core::clone::AssertParamIsClone<[i128; 2]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec2 {}
                    #[repr(align(64))]
                    pub struct InnerAlignedVec4([i128; 4]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec4 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec4",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec4 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec4 {
                            let _: ::core::clone::AssertParamIsClone<[i128; 4]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec4 {}
                    const _: () = if !(size_of::<i128>() == 16) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "the provided size for i128: 16 bytes, is not its size",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec2>() == 16 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec2>() == 16 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec4>() == 16 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec4>() == 16 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                }
            }
            mod i16 {
                use crate as gomath;
                use crate::scalar::default_impl::scalar_default_impl;
                impl gomath::scalar::default_impl::ScalarDefaultImpl for i16 {}
                mod inner_vecs_10900643894459623307 {
                    use super::*;
                    unsafe impl gomath::vec::inner::ScalarInnerVecs for i16 {
                        type InnerAlignedVec2 = InnerAlignedVec2;
                        type InnerAlignedVec4 = InnerAlignedVec4;
                    }
                    #[repr(align(4))]
                    pub struct InnerAlignedVec2([i16; 2]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec2 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec2",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec2 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec2 {
                            let _: ::core::clone::AssertParamIsClone<[i16; 2]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec2 {}
                    #[repr(align(8))]
                    pub struct InnerAlignedVec4([i16; 4]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec4 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec4",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec4 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec4 {
                            let _: ::core::clone::AssertParamIsClone<[i16; 4]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec4 {}
                    const _: () = if !(size_of::<i16>() == 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "the provided size for i16: 2 bytes, is not its size",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec2>() == 2 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec2>() == 2 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec4>() == 2 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec4>() == 2 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                }
            }
            mod i32 {
                use crate as gomath;
                use crate::scalar::default_impl::scalar_default_impl;
                impl gomath::scalar::default_impl::ScalarDefaultImpl for i32 {}
                mod inner_vecs_9840377960296536376 {
                    use super::*;
                    unsafe impl gomath::vec::inner::ScalarInnerVecs for i32 {
                        type InnerAlignedVec2 = InnerAlignedVec2;
                        type InnerAlignedVec4 = InnerAlignedVec4;
                    }
                    #[repr(align(8))]
                    pub struct InnerAlignedVec2([i32; 2]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec2 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec2",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec2 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec2 {
                            let _: ::core::clone::AssertParamIsClone<[i32; 2]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec2 {}
                    #[repr(align(16))]
                    pub struct InnerAlignedVec4([i32; 4]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec4 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec4",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec4 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec4 {
                            let _: ::core::clone::AssertParamIsClone<[i32; 4]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec4 {}
                    const _: () = if !(size_of::<i32>() == 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "the provided size for i32: 4 bytes, is not its size",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec2>() == 4 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec2>() == 4 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec4>() == 4 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec4>() == 4 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                }
            }
            mod i64 {
                use crate as gomath;
                use crate::scalar::default_impl::scalar_default_impl;
                impl gomath::scalar::default_impl::ScalarDefaultImpl for i64 {}
                mod inner_vecs_17059750471201740779 {
                    use super::*;
                    unsafe impl gomath::vec::inner::ScalarInnerVecs for i64 {
                        type InnerAlignedVec2 = InnerAlignedVec2;
                        type InnerAlignedVec4 = InnerAlignedVec4;
                    }
                    #[repr(align(16))]
                    pub struct InnerAlignedVec2([i64; 2]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec2 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec2",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec2 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec2 {
                            let _: ::core::clone::AssertParamIsClone<[i64; 2]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec2 {}
                    #[repr(align(32))]
                    pub struct InnerAlignedVec4([i64; 4]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec4 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec4",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec4 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec4 {
                            let _: ::core::clone::AssertParamIsClone<[i64; 4]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec4 {}
                    const _: () = if !(size_of::<i64>() == 8) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "the provided size for i64: 8 bytes, is not its size",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec2>() == 8 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec2>() == 8 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec4>() == 8 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec4>() == 8 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                }
            }
            mod i8 {
                use crate as gomath;
                use crate::scalar::default_impl::scalar_default_impl;
                impl gomath::scalar::default_impl::ScalarDefaultImpl for i8 {}
                mod inner_vecs_15064204760916713565 {
                    use super::*;
                    unsafe impl gomath::vec::inner::ScalarInnerVecs for i8 {
                        type InnerAlignedVec2 = InnerAlignedVec2;
                        type InnerAlignedVec4 = InnerAlignedVec4;
                    }
                    #[repr(align(2))]
                    pub struct InnerAlignedVec2([i8; 2]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec2 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec2",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec2 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec2 {
                            let _: ::core::clone::AssertParamIsClone<[i8; 2]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec2 {}
                    #[repr(align(4))]
                    pub struct InnerAlignedVec4([i8; 4]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec4 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec4",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec4 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec4 {
                            let _: ::core::clone::AssertParamIsClone<[i8; 4]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec4 {}
                    const _: () = if !(size_of::<i8>() == 1) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "the provided size for i8: 1 bytes, is not its size",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec2>() == 1 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec2>() == 1 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec4>() == 1 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec4>() == 1 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                }
            }
            mod isize {
                use crate as gomath;
                use crate::scalar::default_impl::scalar_default_impl;
                impl gomath::scalar::default_impl::ScalarDefaultImpl for isize {}
                mod inner_vecs_1706441205137726090 {
                    use super::*;
                    unsafe impl gomath::vec::inner::ScalarInnerVecs for isize {
                        type InnerAlignedVec2 = InnerAlignedVec2;
                        type InnerAlignedVec4 = InnerAlignedVec4;
                    }
                    #[repr(align(16))]
                    pub struct InnerAlignedVec2([isize; 2]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec2 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec2",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec2 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec2 {
                            let _: ::core::clone::AssertParamIsClone<[isize; 2]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec2 {}
                    #[repr(align(32))]
                    pub struct InnerAlignedVec4([isize; 4]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec4 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec4",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec4 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec4 {
                            let _: ::core::clone::AssertParamIsClone<[isize; 4]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec4 {}
                    const _: () = if !(size_of::<isize>() == 8) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "the provided size for isize: 8 bytes, is not its size",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec2>() == 8 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec2>() == 8 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec4>() == 8 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec4>() == 8 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                }
            }
        }
        mod unsigned_ints {
            mod u128 {
                use crate as gomath;
                use crate::scalar::default_impl::scalar_default_impl;
                impl gomath::scalar::default_impl::ScalarDefaultImpl for u128 {}
                mod inner_vecs_7026937442784286838 {
                    use super::*;
                    unsafe impl gomath::vec::inner::ScalarInnerVecs for u128 {
                        type InnerAlignedVec2 = InnerAlignedVec2;
                        type InnerAlignedVec4 = InnerAlignedVec4;
                    }
                    #[repr(align(32))]
                    pub struct InnerAlignedVec2([u128; 2]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec2 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec2",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec2 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec2 {
                            let _: ::core::clone::AssertParamIsClone<[u128; 2]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec2 {}
                    #[repr(align(64))]
                    pub struct InnerAlignedVec4([u128; 4]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec4 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec4",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec4 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec4 {
                            let _: ::core::clone::AssertParamIsClone<[u128; 4]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec4 {}
                    const _: () = if !(size_of::<u128>() == 16) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "the provided size for u128: 16 bytes, is not its size",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec2>() == 16 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec2>() == 16 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec4>() == 16 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec4>() == 16 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                }
            }
            mod u16 {
                use crate as gomath;
                use crate::scalar::default_impl::scalar_default_impl;
                impl gomath::scalar::default_impl::ScalarDefaultImpl for u16 {}
                mod inner_vecs_14807517481747284577 {
                    use super::*;
                    unsafe impl gomath::vec::inner::ScalarInnerVecs for u16 {
                        type InnerAlignedVec2 = InnerAlignedVec2;
                        type InnerAlignedVec4 = InnerAlignedVec4;
                    }
                    #[repr(align(4))]
                    pub struct InnerAlignedVec2([u16; 2]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec2 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec2",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec2 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec2 {
                            let _: ::core::clone::AssertParamIsClone<[u16; 2]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec2 {}
                    #[repr(align(8))]
                    pub struct InnerAlignedVec4([u16; 4]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec4 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec4",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec4 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec4 {
                            let _: ::core::clone::AssertParamIsClone<[u16; 4]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec4 {}
                    const _: () = if !(size_of::<u16>() == 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "the provided size for u16: 2 bytes, is not its size",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec2>() == 2 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec2>() == 2 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec4>() == 2 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec4>() == 2 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                }
            }
            mod u32 {
                use crate as gomath;
                use crate::scalar::default_impl::scalar_default_impl;
                impl gomath::scalar::default_impl::ScalarDefaultImpl for u32 {}
                mod inner_vecs_17805753030591285685 {
                    use super::*;
                    unsafe impl gomath::vec::inner::ScalarInnerVecs for u32 {
                        type InnerAlignedVec2 = InnerAlignedVec2;
                        type InnerAlignedVec4 = InnerAlignedVec4;
                    }
                    #[repr(align(8))]
                    pub struct InnerAlignedVec2([u32; 2]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec2 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec2",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec2 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec2 {
                            let _: ::core::clone::AssertParamIsClone<[u32; 2]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec2 {}
                    #[repr(align(16))]
                    pub struct InnerAlignedVec4([u32; 4]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec4 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec4",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec4 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec4 {
                            let _: ::core::clone::AssertParamIsClone<[u32; 4]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec4 {}
                    const _: () = if !(size_of::<u32>() == 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "the provided size for u32: 4 bytes, is not its size",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec2>() == 4 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec2>() == 4 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec4>() == 4 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec4>() == 4 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                }
            }
            mod u64 {
                use crate as gomath;
                use crate::scalar::default_impl::scalar_default_impl;
                impl gomath::scalar::default_impl::ScalarDefaultImpl for u64 {}
                mod inner_vecs_2583076820244694339 {
                    use super::*;
                    unsafe impl gomath::vec::inner::ScalarInnerVecs for u64 {
                        type InnerAlignedVec2 = InnerAlignedVec2;
                        type InnerAlignedVec4 = InnerAlignedVec4;
                    }
                    #[repr(align(16))]
                    pub struct InnerAlignedVec2([u64; 2]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec2 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec2",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec2 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec2 {
                            let _: ::core::clone::AssertParamIsClone<[u64; 2]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec2 {}
                    #[repr(align(32))]
                    pub struct InnerAlignedVec4([u64; 4]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec4 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec4",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec4 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec4 {
                            let _: ::core::clone::AssertParamIsClone<[u64; 4]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec4 {}
                    const _: () = if !(size_of::<u64>() == 8) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "the provided size for u64: 8 bytes, is not its size",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec2>() == 8 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec2>() == 8 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec4>() == 8 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec4>() == 8 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                }
            }
            mod u8 {
                use crate as gomath;
                use crate::scalar::default_impl::scalar_default_impl;
                impl gomath::scalar::default_impl::ScalarDefaultImpl for u8 {}
                mod inner_vecs_7395965558579549491 {
                    use super::*;
                    unsafe impl gomath::vec::inner::ScalarInnerVecs for u8 {
                        type InnerAlignedVec2 = InnerAlignedVec2;
                        type InnerAlignedVec4 = InnerAlignedVec4;
                    }
                    #[repr(align(2))]
                    pub struct InnerAlignedVec2([u8; 2]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec2 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec2",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec2 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec2 {
                            let _: ::core::clone::AssertParamIsClone<[u8; 2]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec2 {}
                    #[repr(align(4))]
                    pub struct InnerAlignedVec4([u8; 4]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec4 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec4",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec4 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec4 {
                            let _: ::core::clone::AssertParamIsClone<[u8; 4]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec4 {}
                    const _: () = if !(size_of::<u8>() == 1) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "the provided size for u8: 1 bytes, is not its size",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec2>() == 1 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec2>() == 1 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec4>() == 1 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec4>() == 1 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                }
            }
            mod usize {
                use crate as gomath;
                use crate::scalar::default_impl::scalar_default_impl;
                impl gomath::scalar::default_impl::ScalarDefaultImpl for usize {}
                mod inner_vecs_3088197000634950847 {
                    use super::*;
                    unsafe impl gomath::vec::inner::ScalarInnerVecs for usize {
                        type InnerAlignedVec2 = InnerAlignedVec2;
                        type InnerAlignedVec4 = InnerAlignedVec4;
                    }
                    #[repr(align(16))]
                    pub struct InnerAlignedVec2([usize; 2]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec2 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec2",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec2 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec2 {
                            let _: ::core::clone::AssertParamIsClone<[usize; 2]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec2 {}
                    #[repr(align(32))]
                    pub struct InnerAlignedVec4([usize; 4]);
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InnerAlignedVec4 {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "InnerAlignedVec4",
                                &&self.0,
                            )
                        }
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for InnerAlignedVec4 {
                        #[inline]
                        fn clone(&self) -> InnerAlignedVec4 {
                            let _: ::core::clone::AssertParamIsClone<[usize; 4]>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for InnerAlignedVec4 {}
                    const _: () = if !(size_of::<usize>() == 8) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "the provided size for usize: 8 bytes, is not its size",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec2>() == 8 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec2>() == 8 * 2) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec2> == size_of<T> * 2",
                                ),
                            );
                        }
                    };
                    const _: () = if !(size_of::<InnerAlignedVec4>() == 8 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: size_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                    const _: () = if !(align_of::<InnerAlignedVec4>() == 8 * 4) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "requirement not met: align_of<InnerAlignedVec4> == size_of<T> * 4",
                                ),
                            );
                        }
                    };
                }
            }
        }
    }
    use crate::{construct::Construct, vec::*};
    /// trait for types that can be put inside mathamatical types like [vectors](crate::vec::Vector) and [matricies](crate::mat::Matrix).
    ///
    /// useful when using mathamatical types while being generic over the scalar type.
    /// # Examples
    /// ```
    /// fn print_x<T: Scalar>(vec: Vec2<T>) {
    ///     println!("x is equal to {}", vec.x())
    /// }
    /// ```
    ///
    /// # Implementing [Scalar]
    /// To implement [Scalar] you need to implement all [Vector](crate::vec::Vector) fns for the scalar type.
    /// This is so that each vector fn can be optimized differently for each scalar.
    /// for example, [f32] uses SIMD to implement fns on most targets.
    ///
    /// To make an unoptimized scalar type use [scalar_default_impl](default_impl::scalar_default_impl).
    ///
    /// To make a wrapper scaler type for an existing scalar (for example Meters(f32)) use ```todo!()```
    pub trait Scalar: Construct + PartialOrd + inner::ScalarInnerVecs + api::ScalarVecApi<
            2,
            VecPacked,
        > + api::ScalarVecApi<
            3,
            VecPacked,
        > + api::ScalarVecApi<
            4,
            VecPacked,
        > + api::ScalarVecApi<
            2,
            VecAligned,
        > + api::ScalarVecApi<3, VecAligned> + api::ScalarVecApi<4, VecAligned> {}
    pub use gomath_proc_macros::scalar_aliases;
}
pub mod vec {
    //! Staticly-lengthed vectors of [scalars](scalar) with lengths between 2 and 4.
    use crate::{construct::*, scalar::*};
    mod alignment {
        use super::*;
        /// Sealed trait for the alignment rules of a vector.
        /// - Doesn't affect the outer vector API, just the inner implementation.
        /// - Use the [```VecN```]```<N, T>``` type alias to use the default storage.
        ///
        /// Implemented by ```VecAligned``` and ```VecPacked```, each have different uses and advantages.
        /// To understand them first understand [Rust type-layout](<https://doc.rust-lang.org/reference/type-layout.html>).
        ///
        /// ### [VecPacked]
        ///
        /// ensures that the vector has the same type-layout as ```[T; N]```.
        /// ```
        /// // VecNP<N, T> = Vector<N, T, VecPacked>
        /// assert_eq!(
        ///     size_of::<VecNP<N, T>>(),
        ///     size_of::<T>() * N
        /// );
        /// assert_eq!(
        ///     align_of::<VecNP<N, T>>(),
        ///     align_of::<T>()
        /// );
        /// ```
        ///
        /// - use [```VecNP```]<N, T>
        ///
        /// ### [VecAligned]
        ///
        /// ensures that the vector has the next alignment from ```[T; N]```'s size, and a size equal to the alignment.
        /// ```
        /// // VecN<N, T> = Vector<N, T, VecAligned>
        /// assert_eq!(
        ///     size_of::<VecN<N, T>>(),
        ///     (size_of::<T>() * N).next_power_of_two()
        /// );
        /// assert_eq!(
        ///     align_of::<VecN<N, T>>(),
        ///     (size_of::<T>() * N).next_power_of_two()
        /// );
        /// ```
        ///
        /// - This means that the size and alignment of ```Vec3<T>``` is the same as ```Vec4<T>```'s.
        /// - This means that ```size/align_of<Vec2> = size_of<T> * 2```, and ```size/align_of<Vec3> = size/align_of<Vec4> = size_of<T> * 4```.
        ///
        /// - use [```VecN```]<N, T>
        ///
        /// ## How to pick
        ///
        /// Sometimes the ```VecAligned``` type-layout is required.
        /// For example in GPU uniform-structs that have strict type-layout requirements, which include vectors following the ```VecAligned``` type-layout.
        /// When neither storage is required, choose based on their performance advantages/disadvantages:
        ///
        /// - ```VecAligned``` results in faster computations because of SIMD registers which require the extra alignment.
        /// - ```VecAligned``` may take more space because of the larger alignment, and that a ```Vec3``` always takes the space of a ```Vec4```.
        /// - ```VecPacked``` takes less space because of the minimal alignment and the lack of padding.
        /// - ```VecPacked``` may result in slower computations because of the SIMD register's requirements.
        ///
        /// Basically only use ```VecPacked``` (```VecNP```) when storing large arrays of vectors that you don't perform much computation on.
        /// On any other case use ```VecAligned``` (```VecN```, The default).
        #[allow(private_bounds)]
        pub trait VecAlignment: Seal + inner::VecAlignmentInnerVecs + api::VecAlignmentApi {}
        /// Vector inner storage that ensures that the vector has the next alignment from ```[T; N]```'s size, and a size equal to the alignment.
        /// ```
        /// // VecN<N, T> = Vector<N, T, VecAligned>
        /// assert_eq!(
        ///     size_of::<VecN<N, T>>(),
        ///     (size_of::<T>() * N).next_power_of_two()
        /// );
        /// assert_eq!(
        ///     align_of::<VecN<N, T>>(),
        ///     (size_of::<T>() * N).next_power_of_two()
        /// );
        /// ```
        ///
        /// - This means that the size and alignment of ```Vec3<T>``` is the same as ```Vec4<T>```'s.
        /// - This means that ```size/align_of<Vec2> = size_of<T> * 2```, and ```size/align_of<Vec3> = size/align_of<Vec4> = size_of<T> * 4```.
        ///
        /// ## When to use
        ///
        /// Sometimes the ```VecAligned``` type-layout is required.
        /// For example in GPU uniform-structs that have strict type-layout requirements, which include vectors following the ```VecAligned``` type-layout.
        /// When not required, choose based on performance advantages/disadvantages:
        ///
        /// - results in faster computations than ```VecPacked``` because of SIMD registers which require the extra alignment.
        /// - may take more space than ```VecPacked``` because of the larger alignment, and that a ```Vec3``` always takes the space of a ```Vec4```.
        ///
        /// Always recommended except for when storing large arrays of vectors that you don't perform much computation on.
        pub struct VecAligned;
        impl Seal for VecAligned {}
        impl VecAlignment for VecAligned {}
        /// Vector inner storage that ensures that the vector has the same type-layout as ```[T; N]```.
        /// ```
        /// // VecNP<N, T> = Vector<N, T, VecPacked>
        /// assert_eq!(
        ///     size_of::<VecNP<N, T>>(),
        ///     size_of::<T>() * N
        /// );
        /// assert_eq!(
        ///     align_of::<VecNP<N, T>>(),
        ///     align_of::<T>()
        /// );
        /// ```
        ///
        /// ## When to use
        ///
        /// Sometimes the ```VecAligned``` type-layout is required.
        /// For example in GPU uniform-structs that have strict type-layout requirements, which include vectors following the ```VecAligned``` type-layout.
        /// When ```VecAligned``` isn't required, choose based on performance advantages/disadvantages:
        ///
        /// - takes less space than ```VecAligned``` because of the minimal alignment and the lack of padding.
        /// - may result in slower computations than ```VecAligned``` because of the SIMD register's requirements.
        ///
        /// Only recommended when storing large arrays of vectors that you don't perform much computation on.
        pub struct VecPacked;
        impl Seal for VecPacked {}
        impl VecAlignment for VecPacked {}
        trait Seal {}
    }
    mod len {
        use super::*;
        /// Sealed trait for ```ScalarCount```s that are valid as vector lengths.
        ///
        /// Vectors can only have lengths 2, 3, or 4 because internally vector fns have differently optimized implementations for each length.
        ///
        /// This trait is implemented by ```ScalarCount<2/3/4>``` and is used to validate that a generic vector length is either 2, 3, or 4 with ```where ScalarCount<N>: VecLen<N>```.
        ///
        /// # Examples
        /// ```
        /// // Line is generic over dimension count.
        /// struct Line<const N: usize>
        /// where
        ///     ScalarCount<N>: VecLen<N>,
        /// {
        ///     start: FVecN<N>,
        ///     end: FVecN<N>,
        /// }
        ///
        /// type Line2D = Line<2>;
        /// type Line3D = Line<3>;
        /// type Line4D = Line<4>;
        ///
        /// // Using an N that isn't 2, 3, or 4. Wont work.
        /// struct InvalidStruct {
        ///     line: Line<6>, // ERROR: the trait bound `ScalarCount<6>: VecLen<6>` is not satisfied
        /// }
        /// ```
        #[allow(private_bounds)]
        pub trait VecLen<
            const N: usize,
        >: Seal + inner::VecLenInnerVec + api::VecLenApi<N>
        where
            ScalarCount<N>: VecLen<N>,
        {}
        /// Count of scalars that may or may not be a [```VecLen```].
        ///
        /// Vectors can only have lengths 2, 3, or 4 because internally vector fns have differently optimized implementations for each length.
        ///
        /// Only ```ScalarCount<2/3/4>``` implements ```VecLen```.
        /// this is used to validate that a generic vector length is either 2, 3, or 4 with ```where ScalarCount<N>: VecLen<N>```.
        ///
        /// # Examples
        /// ```
        /// // Line is generic over dimension count.
        /// struct Line<const N: usize>
        /// where
        ///     ScalarCount<N>: VecLen<N>,
        /// {
        ///     start: FVecN<N>,
        ///     end: FVecN<N>,
        /// }
        ///
        /// type Line2D = Line<2>;
        /// type Line3D = Line<3>;
        /// type Line4D = Line<4>;
        ///
        /// // Using an N that isn't 2, 3, or 4. Wont work.
        /// struct InvalidStruct {
        ///     line: Line<6>, // ERROR: the trait bound `ScalarCount<6>: VecLen<6>` is not satisfied
        /// }
        /// ```
        pub struct ScalarCount<const VALUE: usize>;
        impl Seal for ScalarCount<2> {}
        impl Seal for ScalarCount<4> {}
        impl Seal for ScalarCount<3> {}
        impl VecLen<2> for ScalarCount<2> {}
        impl VecLen<3> for ScalarCount<3> {}
        impl VecLen<4> for ScalarCount<4> {}
        trait Seal {}
    }
    pub use alignment::*;
    pub use len::*;
    pub mod api {
        //! Scalar supertraits for vector API
        use super::*;
        mod array {
            use std::mem::{transmute, transmute_copy};
            use gomath_proc_macros::vec_api;
            use super::{
                inner::{InnerVector, ScalarInnerVecs},
                *,
            };
            impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
            where
                ScalarCount<N>: VecLen<N>,
            {
                #[inline(always)]
                pub fn from_array(array: [T; N]) -> Self {
                    Vector {
                        inner: <ScalarCount<
                            N,
                        > as VecLenArrayApi<
                            N,
                        >>::from_array::<
                            T: Scalar,
                            A: VecAlignment,
                        >(array.map(|item| item)),
                    }
                }
                #[inline(always)]
                pub fn into_array(self) -> [T; N] {
                    unsafe { transmute_copy(&self) }
                }
                #[inline(always)]
                pub fn as_array(&self) -> &[T; N] {
                    unsafe { transmute(self) }
                }
                #[inline(always)]
                pub fn as_array_mut(&mut self) -> &mut [T; N] {
                    unsafe { transmute(self) }
                }
            }
            pub trait ScalarVecArrayApi<
                const N: usize,
                A: VecAlignment,
            >: inner::ScalarInnerVecs
            where
                ScalarCount<N>: VecLen<N>,
            {
                fn from_array(array: [Self; N]) -> inner::InnerVector<N, Self, A>;
            }
            pub(super) trait VecLenArrayApi<const N: usize>: inner::VecLenInnerVec
            where
                ScalarCount<N>: VecLen<N>,
            {
                fn from_array<T: Scalar, A: VecAlignment>(
                    array: [T; N],
                ) -> inner::InnerVector<N, T, A>;
            }
            impl VecLenArrayApi<2> for ScalarCount<2> {
                #[inline(always)]
                fn from_array<T: Scalar, A: VecAlignment>(
                    array: [T; 2],
                ) -> inner::InnerVector<2, T, A> {
                    <A as VecAlignmentArrayApi<2>>::from_array::<T: Scalar>(array)
                }
            }
            impl VecLenArrayApi<3> for ScalarCount<3> {
                #[inline(always)]
                fn from_array<T: Scalar, A: VecAlignment>(
                    array: [T; 3],
                ) -> inner::InnerVector<3, T, A> {
                    <A as VecAlignmentArrayApi<3>>::from_array::<T: Scalar>(array)
                }
            }
            impl VecLenArrayApi<4> for ScalarCount<4> {
                #[inline(always)]
                fn from_array<T: Scalar, A: VecAlignment>(
                    array: [T; 4],
                ) -> inner::InnerVector<4, T, A> {
                    <A as VecAlignmentArrayApi<4>>::from_array::<T: Scalar>(array)
                }
            }
            pub(super) trait VecAlignmentArrayApi<
                const N: usize,
            >: inner::VecAlignmentInnerVecs
            where
                ScalarCount<N>: VecLen<N>,
            {
                fn from_array<T: Scalar>(
                    array: [T; N],
                ) -> inner::InnerVector<N, T, Self>;
            }
            impl VecAlignmentArrayApi<2> for VecPacked {
                #[inline(always)]
                fn from_array<T: Scalar>(
                    array: [T; 2],
                ) -> inner::InnerVector<2, T, Self> {
                    <T as ScalarVecArrayApi<2, Self>>::from_array(array)
                }
            }
            impl VecAlignmentArrayApi<2> for VecAligned {
                #[inline(always)]
                fn from_array<T: Scalar>(
                    array: [T; 2],
                ) -> inner::InnerVector<2, T, Self> {
                    <T as ScalarVecArrayApi<2, Self>>::from_array(array)
                }
            }
            impl VecAlignmentArrayApi<3> for VecPacked {
                #[inline(always)]
                fn from_array<T: Scalar>(
                    array: [T; 3],
                ) -> inner::InnerVector<3, T, Self> {
                    <T as ScalarVecArrayApi<3, Self>>::from_array(array)
                }
            }
            impl VecAlignmentArrayApi<3> for VecAligned {
                #[inline(always)]
                fn from_array<T: Scalar>(
                    array: [T; 3],
                ) -> inner::InnerVector<3, T, Self> {
                    <T as ScalarVecArrayApi<3, Self>>::from_array(array)
                }
            }
            impl VecAlignmentArrayApi<4> for VecPacked {
                #[inline(always)]
                fn from_array<T: Scalar>(
                    array: [T; 4],
                ) -> inner::InnerVector<4, T, Self> {
                    <T as ScalarVecArrayApi<4, Self>>::from_array(array)
                }
            }
            impl VecAlignmentArrayApi<4> for VecAligned {
                #[inline(always)]
                fn from_array<T: Scalar>(
                    array: [T; 4],
                ) -> inner::InnerVector<4, T, Self> {
                    <T as ScalarVecArrayApi<4, Self>>::from_array(array)
                }
            }
            impl<const N: usize, T: ScalarInnerVecs> ScalarVecArrayApi<N, VecPacked>
            for T
            where
                ScalarCount<N>: VecLen<N>,
            {
                fn from_array(array: [Self; N]) -> InnerVector<N, Self, VecPacked> {
                    array
                }
            }
        }
        mod or_scalar {
            use super::*;
            pub type VectorOrScalar<const N: usize, T, S> = <ScalarCount<
                N,
            > as VecLenOrOne>::VectorOrScalar<T, S>;
            pub type InnerVectorOrScalar<const N: usize, T, S> = <ScalarCount<
                N,
            > as VecLenOrOne>::InnerVectorOrScalar<T, S>;
            #[allow(private_bounds)]
            pub trait VecLenOrOne: Seal {
                type VectorOrScalar<T: Scalar, S: VecAlignment>: Construct;
                type InnerVectorOrScalar<T: Scalar, S: VecAlignment>: InnerConstruct;
            }
            impl Seal for ScalarCount<1> {}
            impl Seal for ScalarCount<2> {}
            impl Seal for ScalarCount<3> {}
            impl Seal for ScalarCount<4> {}
            impl VecLenOrOne for ScalarCount<1> {
                type InnerVectorOrScalar<T: Scalar, S: VecAlignment> = T;
                type VectorOrScalar<T: Scalar, S: VecAlignment> = T;
            }
            impl VecLenOrOne for ScalarCount<2> {
                type VectorOrScalar<T: Scalar, S: VecAlignment> = Vector2<T, S>;
                type InnerVectorOrScalar<T: Scalar, S: VecAlignment> = inner::InnerVector<
                    2,
                    T,
                    S,
                >;
            }
            impl VecLenOrOne for ScalarCount<3> {
                type VectorOrScalar<T: Scalar, S: VecAlignment> = Vector3<T, S>;
                type InnerVectorOrScalar<T: Scalar, S: VecAlignment> = inner::InnerVector<
                    3,
                    T,
                    S,
                >;
            }
            impl VecLenOrOne for ScalarCount<4> {
                type VectorOrScalar<T: Scalar, S: VecAlignment> = Vector4<T, S>;
                type InnerVectorOrScalar<T: Scalar, S: VecAlignment> = inner::InnerVector<
                    4,
                    T,
                    S,
                >;
            }
            trait Seal {}
        }
        pub use array::*;
        pub use or_scalar::*;
        mod construct {
            use super::*;
            mod copy {
                use super::*;
                impl<const N: usize, T: Scalar, S: VecAlignment> Clone
                for Vector<N, T, S>
                where
                    ScalarCount<N>: VecLen<N>,
                {
                    #[inline(always)]
                    fn clone(&self) -> Self {
                        Self { inner: self.inner }
                    }
                }
                impl<const N: usize, T: Scalar, S: VecAlignment> Copy for Vector<N, T, S>
                where
                    ScalarCount<N>: VecLen<N>,
                {}
            }
            mod debug {
                use std::fmt::{Debug, Formatter, Result};
                use super::*;
                impl<const N: usize, T: Scalar, S: VecAlignment> Debug
                for Vector<N, T, S>
                where
                    ScalarCount<N>: VecLen<N>,
                {
                    #[inline(always)]
                    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                        self.inner.fmt(f)
                    }
                }
            }
            mod display {
                use std::fmt::{self, Display, Formatter};
                use super::*;
                impl<const N: usize, T: Scalar, S: VecAlignment> Display
                for Vector<N, T, S>
                where
                    ScalarCount<N>: VecLen<N>,
                {
                    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                        f.write_fmt(
                            format_args!(
                                "({0})",
                                self.into_array().map(|c| c.to_string()).join(", "),
                            ),
                        )
                    }
                }
            }
            mod partial_eq {
                use super::*;
                impl<const N: usize, T: Scalar, S: VecAlignment> PartialEq
                for Vector<N, T, S>
                where
                    ScalarCount<N>: VecLen<N>,
                {
                    #[inline(always)]
                    fn eq(&self, other: &Self) -> bool {
                        self.into_array().eq(&other.into_array())
                    }
                }
            }
        }
        /// Scalar supertrait for implementing the vector API for a specific <N, S>.
        /// This is an empty trait with more supertraits for every related fn set.
        pub trait ScalarVecApi<
            const N: usize,
            S: VecAlignment,
        >: inner::ScalarInnerVecs + array::ScalarVecArrayApi<N, S>
        where
            ScalarCount<N>: VecLen<N>,
        {}
        #[allow(private_bounds)]
        pub(super) trait VecLenApi<const N: usize>: array::VecLenArrayApi<N>
        where
            ScalarCount<N>: VecLen<N>,
        {}
        impl VecLenApi<2> for ScalarCount<2> {}
        impl VecLenApi<3> for ScalarCount<3> {}
        impl VecLenApi<4> for ScalarCount<4> {}
        #[allow(private_bounds)]
        pub(super) trait VecAlignmentApi: array::VecAlignmentArrayApi<
                2,
            > + array::VecAlignmentArrayApi<3> + array::VecAlignmentArrayApi<4> {}
        impl VecAlignmentApi for VecPacked {}
        impl VecAlignmentApi for VecAligned {}
    }
    pub mod inner {
        //! Behaviour for selecting an inner-vector type based on a vector's length, scalar, and storage.
        //!
        //!
        use super::*;
        /// The type of the inner-value inside a vector
        pub type InnerVector<const N: usize, T, S> = <S as VecAlignmentInnerVecs>::InnerVec<
            N,
            T,
        >;
        /// Scalar supertrait that specifies inner-types for vectors that can't be declared generically.
        ///
        /// - Unsafe to implement manually because the implementation is expected to comply with type-layout guarentees.
        /// Instead, implement using [```aligned_vecs```].
        pub unsafe trait ScalarInnerVecs: Construct {
            /// Inner-type for ```VecAligned``` Vec2s.
            /// - Guarenteed: ```size = align = size_of::<T>().next_power_of_two() * 2```
            type InnerAlignedVec2: InnerConstruct;
            /// Inner-type for ```VecAligned``` Vec4s and Vec3s.
            /// - Guarenteed: ```size = align = size_of::<T>().next_power_of_two() * 4```
            type InnerAlignedVec4: InnerConstruct;
        }
        pub use gomath_proc_macros::inner_vecs;
        #[doc(hidden)]
        #[allow(private_bounds)]
        pub trait VecLenInnerVec: Seal {
            type InnerAlignedVec<T: ScalarInnerVecs>: InnerConstruct;
        }
        impl Seal for ScalarCount<2> {}
        impl Seal for ScalarCount<4> {}
        impl Seal for ScalarCount<3> {}
        impl VecLenInnerVec for ScalarCount<2> {
            type InnerAlignedVec<T: ScalarInnerVecs> = T::InnerAlignedVec2;
        }
        impl VecLenInnerVec for ScalarCount<3> {
            type InnerAlignedVec<T: ScalarInnerVecs> = T::InnerAlignedVec4;
        }
        impl VecLenInnerVec for ScalarCount<4> {
            type InnerAlignedVec<T: ScalarInnerVecs> = T::InnerAlignedVec4;
        }
        #[doc(hidden)]
        #[allow(private_bounds)]
        pub trait VecAlignmentInnerVecs: Seal {
            type InnerVec<const N: usize, T: ScalarInnerVecs>: InnerConstruct
            where
                ScalarCount<N>: VecLenInnerVec;
        }
        impl Seal for VecPacked {}
        impl VecAlignmentInnerVecs for VecPacked {
            type InnerVec<const N: usize, T: ScalarInnerVecs> = [T; N]
            where
                ScalarCount<N>: VecLenInnerVec;
        }
        impl Seal for VecAligned {}
        impl VecAlignmentInnerVecs for VecAligned {
            type InnerVec<const N: usize, T: ScalarInnerVecs> = <ScalarCount<
                N,
            > as VecLenInnerVec>::InnerAlignedVec<T>
            where
                ScalarCount<N>: VecLenInnerVec;
        }
        trait Seal: Sized {}
    }
    /// Statically-lengthed vector generic over N - length, T - Scalar, and A - Alignment.
    ///
    /// Storage affects the inner implementation of vector fns.
    ///
    /// Only use this type when being generic over N, T, and S.
    /// there are simpler type aliases to this type for when being not generic.
    ///
    /// - ```Vector2<T, S>```, ```Vector3<T, S>```, and ```Vector4<T, S>``` fill N.
    /// - ```VecN<N, T>```, ```Vec2<T>```, ```Vec3<T>```, and ```Vec4<T>``` use the default storage [```VecAligned```].
    /// - ```VecNP<N, T>```, ```Vec2P<T>```, ```Vec3P<T>```, and ```Vec4P<T>``` use the non-default storage [```VecPacked```].
    /// - [```scalar::aliases```](crate::scalar::aliases) contains aliases for each primitive.
    ///
    /// # Examples
    /// ```
    /// fn print_vec<const N: usize, T: Scalar, S: VecStorage>(vec: Vector<N, T, S>)
    /// where
    ///     ScalarCount<N>: VecLen<N>, // Required by Vector to ensure that N is either 2, 3, or 4.
    /// {
    ///     println!("{vec}")
    /// }
    /// ```
    #[repr(transparent)]
    pub struct Vector<const N: usize, T: Scalar, A: VecAlignment>
    where
        ScalarCount<N>: VecLen<N>,
    {
        inner: inner::InnerVector<N, T, A>,
    }
    /// type alias to [```Vector```]```<2, T, S>```
    pub type Vector2<T, A> = Vector<2, T, A>;
    /// type alias to [```Vector```]```<3, T, S>```
    pub type Vector3<T, A> = Vector<3, T, A>;
    /// type alias to [```Vector```]```<4, T, S>```
    pub type Vector4<T, A> = Vector<4, T, A>;
    /// Statically-lengthed vector generic over N - length, and T - Scalar.
    /// - type alias to [```Vector```]```<N, T, VecAligned>```
    pub type VecN<const N: usize, T> = Vector<N, T, VecAligned>;
    /// type alias to [```VecN```]```<2, T>```
    pub type Vec2<T> = VecN<2, T>;
    /// type alias to [```VecN```]```<3, T>```
    pub type Vec3<T> = VecN<3, T>;
    /// type alias to [```VecN```]```<4, T>```
    pub type Vec4<T> = VecN<4, T>;
    /// Statically-lengthed vector generic over N - length, and T - Scalar.
    /// - type alias to [```Vector```]```<N, T, VecPacked>```
    /// - If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```VecN```].
    pub type VecNP<const N: usize, T> = Vector<N, T, VecPacked>;
    /// type alias to [```VecNP```]```<2, T>```
    /// - If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Vec2```].
    pub type Vec2P<T> = VecNP<2, T>;
    /// type alias to [```VecNP```]```<3, T>```
    /// - If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Vec3```].
    pub type Vec3P<T> = VecNP<3, T>;
    /// type alias to [```VecNP```]```<4, T>```
    /// - If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Vec4```].
    pub type Vec4P<T> = VecNP<4, T>;
}
