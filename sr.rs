#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod construct {
    pub trait Construct: InnerConstruct + std::fmt::Display {}
    impl<T: InnerConstruct + std::fmt::Display> Construct for T {}
    pub trait InnerConstruct: Sized + Send + Sync + Copy + PartialEq + std::fmt::Debug {}
    impl<T: Sized + Send + Sync + Copy + PartialEq + std::fmt::Debug> InnerConstruct
    for T {}
}
pub mod scalar {
    pub mod default_impl {
        use crate::vec::ScalarAlignedVecs;
        use super::*;
        mod vec {
            use crate::vec::*;
            use super::*;
            mod array {
                use std::mem::transmute_copy;
                use crate::vec::array::*;
                use super::*;
                impl<
                    const N: usize,
                    T: ScalarDefaultImpl,
                > ScalarVecArrayApi<N, VecPacked> for T
                where
                    ScalarCount<N>: VecLen<N>,
                {
                    fn from_array(array: [Self; N]) -> InnerVector<N, VecPacked, Self> {
                        array
                    }
                }
                impl<T: ScalarDefaultImpl> ScalarVecArrayApi<2, VecAligned> for T {
                    fn from_array(array: [Self; 2]) -> InnerVector<2, VecAligned, Self> {
                        unsafe { transmute_copy(&array) }
                    }
                }
                impl<T: ScalarDefaultImpl> ScalarVecArrayApi<3, VecAligned> for T {
                    fn from_array(array: [Self; 3]) -> InnerVector<3, VecAligned, Self> {
                        unsafe {
                            transmute_copy(&[array[0], array[1], array[2], array[2]])
                        }
                    }
                }
                impl<T: ScalarDefaultImpl> ScalarVecArrayApi<4, VecAligned> for T {
                    fn from_array(array: [Self; 4]) -> InnerVector<4, VecAligned, Self> {
                        unsafe { transmute_copy(&array) }
                    }
                }
            }
            impl<T: ScalarDefaultImpl> ScalarVec for T {}
            impl<T: ScalarDefaultImpl> ScalarVecApiImpl<2, VecPacked> for T {}
            impl<T: ScalarDefaultImpl> ScalarVecApiImpl<3, VecPacked> for T {}
            impl<T: ScalarDefaultImpl> ScalarVecApiImpl<4, VecPacked> for T {}
            impl<T: ScalarDefaultImpl> ScalarVecApiImpl<2, VecAligned> for T {}
            impl<T: ScalarDefaultImpl> ScalarVecApiImpl<3, VecAligned> for T {}
            impl<T: ScalarDefaultImpl> ScalarVecApiImpl<4, VecAligned> for T {}
        }
        pub trait ScalarDefaultImpl: Construct + ScalarAlignedVecs {}
        impl<T: ScalarDefaultImpl> Scalar for T {}
    }
    mod primitive_impls {
        mod floats {
            mod f32 {
                use gomath_proc_macros::aligned_vecs;
                use crate::{
                    scalar::default_impl::ScalarDefaultImpl, vec::ScalarAlignedVecs,
                };
                impl ScalarDefaultImpl for f32 {}
                unsafe impl ScalarAlignedVecs for f32 {
                    type InnerAlignedVec2 = AlignedFVec2;
                    type InnerAlignedVec4 = AlignedFVec4;
                }
                #[repr(align(8))]
                pub struct AlignedFVec2([f32; 2]);
                #[automatically_derived]
                impl ::core::fmt::Debug for AlignedFVec2 {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "AlignedFVec2",
                            &&self.0,
                        )
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for AlignedFVec2 {
                    #[inline]
                    fn clone(&self) -> AlignedFVec2 {
                        let _: ::core::clone::AssertParamIsClone<[f32; 2]>;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for AlignedFVec2 {}
                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for AlignedFVec2 {}
                #[automatically_derived]
                impl ::core::cmp::PartialEq for AlignedFVec2 {
                    #[inline]
                    fn eq(&self, other: &AlignedFVec2) -> bool {
                        self.0 == other.0
                    }
                }
                #[automatically_derived]
                impl ::core::default::Default for AlignedFVec2 {
                    #[inline]
                    fn default() -> AlignedFVec2 {
                        AlignedFVec2(::core::default::Default::default())
                    }
                }
                #[repr(align(16))]
                pub struct AlignedFVec4([f32; 4]);
                #[automatically_derived]
                impl ::core::fmt::Debug for AlignedFVec4 {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "AlignedFVec4",
                            &&self.0,
                        )
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for AlignedFVec4 {
                    #[inline]
                    fn clone(&self) -> AlignedFVec4 {
                        let _: ::core::clone::AssertParamIsClone<[f32; 4]>;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for AlignedFVec4 {}
                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for AlignedFVec4 {}
                #[automatically_derived]
                impl ::core::cmp::PartialEq for AlignedFVec4 {
                    #[inline]
                    fn eq(&self, other: &AlignedFVec4) -> bool {
                        self.0 == other.0
                    }
                }
                #[automatically_derived]
                impl ::core::default::Default for AlignedFVec4 {
                    #[inline]
                    fn default() -> AlignedFVec4 {
                        AlignedFVec4(::core::default::Default::default())
                    }
                }
                const _: () = if !(size_of::<f32>() == 4) {
                    {
                        ::core::panicking::panic_fmt((/*ERROR*/));
                    }
                };
                const _: () = if !(size_of::<AlignedFVec2>() == 4 * 2) {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "requirement not met: size_of<AlignedVec2> == size_of<T> * 2",
                            ),
                        );
                    }
                };
                const _: () = if !(align_of::<AlignedFVec2>() == 4 * 2) {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "requirement not met: align_of<AlignedVec2> == size_of<T> * 2",
                            ),
                        );
                    }
                };
                const _: () = if !(size_of::<AlignedFVec4>() == 4 * 4) {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "requirement not met: size_of<AlignedVec4> == size_of<T> * 4",
                            ),
                        );
                    }
                };
                const _: () = if !(align_of::<AlignedFVec4>() == 4 * 4) {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "requirement not met: align_of<AlignedVec4> == size_of<T> * 4",
                            ),
                        );
                    }
                };
            }
        }
        mod signed_ints {}
        mod unsigned_ints {}
    }
    use crate::{construct::Construct, vec::ScalarVec};
    pub trait Scalar: Construct + ScalarVec {}
}
pub mod vec {
    use crate::{construct::*, scalar::*};
    mod len {
        use super::*;
        #[allow(private_bounds)]
        pub trait VecLen<const N: usize>: Seal + VecLenInnerVec + VecLenApi<N>
        where
            ScalarCount<N>: VecLen<N>,
        {}
        pub trait VecLenInnerVec {
            type InnerAlignedVec<T: ScalarAlignedVecs>: InnerConstruct;
        }
        pub struct ScalarCount<const VALUE: usize>;
        impl Seal for ScalarCount<2> {}
        impl Seal for ScalarCount<4> {}
        impl Seal for ScalarCount<3> {}
        impl VecLen<2> for ScalarCount<2> {}
        impl VecLen<3> for ScalarCount<3> {}
        impl VecLen<4> for ScalarCount<4> {}
        impl VecLenInnerVec for ScalarCount<2> {
            type InnerAlignedVec<T: ScalarAlignedVecs> = T::InnerAlignedVec2;
        }
        impl VecLenInnerVec for ScalarCount<3> {
            type InnerAlignedVec<T: ScalarAlignedVecs> = T::InnerAlignedVec4;
        }
        impl VecLenInnerVec for ScalarCount<4> {
            type InnerAlignedVec<T: ScalarAlignedVecs> = T::InnerAlignedVec4;
        }
        trait Seal {}
    }
    mod scalar {
        use super::*;
        pub trait ScalarVec: ScalarAlignedVecs + ScalarVecApiImpl<
                2,
                VecPacked,
            > + ScalarVecApiImpl<
                3,
                VecPacked,
            > + ScalarVecApiImpl<
                4,
                VecPacked,
            > + ScalarVecApiImpl<
                2,
                VecAligned,
            > + ScalarVecApiImpl<3, VecAligned> + ScalarVecApiImpl<4, VecAligned> {}
    }
    mod storage {
        use super::*;
        #[allow(private_bounds)]
        pub trait VecStorage: Seal + VecStorageInnerVecs + VecStorageApi {}
        pub trait VecStorageInnerVecs: Sized {
            type InnerVec<const N: usize, T: ScalarAlignedVecs>: InnerConstruct
            where
                ScalarCount<N>: VecLenInnerVec;
        }
        pub struct VecPacked;
        impl Seal for VecPacked {}
        impl VecStorage for VecPacked {}
        impl VecStorageInnerVecs for VecPacked {
            type InnerVec<const N: usize, T: ScalarAlignedVecs> = [T; N]
            where
                ScalarCount<N>: VecLenInnerVec;
        }
        pub struct VecAligned;
        impl Seal for VecAligned {}
        impl VecStorage for VecAligned {}
        impl VecStorageInnerVecs for VecAligned {
            type InnerVec<const N: usize, T: ScalarAlignedVecs> = <ScalarCount<
                N,
            > as VecLenInnerVec>::InnerAlignedVec<T>
            where
                ScalarCount<N>: VecLenInnerVec;
        }
        pub unsafe trait ScalarAlignedVecs: Construct {
            type InnerAlignedVec2: InnerConstruct;
            type InnerAlignedVec4: InnerConstruct;
        }
        pub use gomath_proc_macros::aligned_vecs;
        trait Seal {}
    }
    pub use len::*;
    pub use scalar::*;
    pub use storage::*;
    mod api {
        use super::*;
        pub mod array {
            use std::mem::{transmute, transmute_copy};
            use gomath_proc_macros::vec_api;
            use super::*;
            impl<const N: usize, S: VecStorage, T: Scalar> Vector<N, S, T>
            where
                ScalarCount<N>: VecLen<N>,
            {
                #[inline(always)]
                pub fn from_array(array: [T; N]) -> Self {
                    Self {
                        inner: <ScalarCount<
                            N,
                        > as VecLenArrayApi<
                            N,
                        >>::from_array::<S, T>(array.map(|item| item)),
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
            pub trait ScalarVecArrayApi<const N: usize, S: VecStorage>: ScalarAlignedVecs
            where
                ScalarCount<N>: VecLen<N>,
            {
                fn from_array(array: [Self; N]) -> InnerVector<N, S, Self>;
            }
            pub(super) trait VecLenArrayApi<const N: usize>: VecLenInnerVec
            where
                ScalarCount<N>: VecLen<N>,
            {
                fn from_array<S: VecStorage, T: Scalar>(
                    array: [T; N],
                ) -> InnerVector<N, S, T>;
            }
            impl VecLenArrayApi<2> for ScalarCount<2> {
                #[inline(always)]
                fn from_array<S: VecStorage, T: Scalar>(
                    array: [T; 2],
                ) -> InnerVector<2, S, T> {
                    <S as VecStorageArrayApi<2>>::from_array::<T>(array)
                }
            }
            impl VecLenArrayApi<3> for ScalarCount<3> {
                #[inline(always)]
                fn from_array<S: VecStorage, T: Scalar>(
                    array: [T; 3],
                ) -> InnerVector<3, S, T> {
                    <S as VecStorageArrayApi<3>>::from_array::<T>(array)
                }
            }
            impl VecLenArrayApi<4> for ScalarCount<4> {
                #[inline(always)]
                fn from_array<S: VecStorage, T: Scalar>(
                    array: [T; 4],
                ) -> InnerVector<4, S, T> {
                    <S as VecStorageArrayApi<4>>::from_array::<T>(array)
                }
            }
            pub(super) trait VecStorageArrayApi<const N: usize>: VecStorageInnerVecs
            where
                ScalarCount<N>: VecLen<N>,
            {
                fn from_array<T: Scalar>(array: [T; N]) -> InnerVector<N, Self, T>;
            }
            impl VecStorageArrayApi<2> for VecPacked {
                #[inline(always)]
                fn from_array<T: Scalar>(array: [T; 2]) -> InnerVector<2, Self, T> {
                    <T as ScalarVecArrayApi<2, Self>>::from_array(array)
                }
            }
            impl VecStorageArrayApi<2> for VecAligned {
                #[inline(always)]
                fn from_array<T: Scalar>(array: [T; 2]) -> InnerVector<2, Self, T> {
                    <T as ScalarVecArrayApi<2, Self>>::from_array(array)
                }
            }
            impl VecStorageArrayApi<3> for VecPacked {
                #[inline(always)]
                fn from_array<T: Scalar>(array: [T; 3]) -> InnerVector<3, Self, T> {
                    <T as ScalarVecArrayApi<3, Self>>::from_array(array)
                }
            }
            impl VecStorageArrayApi<3> for VecAligned {
                #[inline(always)]
                fn from_array<T: Scalar>(array: [T; 3]) -> InnerVector<3, Self, T> {
                    <T as ScalarVecArrayApi<3, Self>>::from_array(array)
                }
            }
            impl VecStorageArrayApi<4> for VecPacked {
                #[inline(always)]
                fn from_array<T: Scalar>(array: [T; 4]) -> InnerVector<4, Self, T> {
                    <T as ScalarVecArrayApi<4, Self>>::from_array(array)
                }
            }
            impl VecStorageArrayApi<4> for VecAligned {
                #[inline(always)]
                fn from_array<T: Scalar>(array: [T; 4]) -> InnerVector<4, Self, T> {
                    <T as ScalarVecArrayApi<4, Self>>::from_array(array)
                }
            }
        }
        pub mod construct {
            use super::*;
            mod copy {
                use super::*;
                impl<const N: usize, S: VecStorage, T: Scalar> Clone for Vector<N, S, T>
                where
                    ScalarCount<N>: VecLen<N>,
                {
                    #[inline(always)]
                    fn clone(&self) -> Self {
                        Self { inner: self.inner }
                    }
                }
                impl<const N: usize, S: VecStorage, T: Scalar> Copy for Vector<N, S, T>
                where
                    ScalarCount<N>: VecLen<N>,
                {}
            }
            mod debug {
                use std::fmt::{Debug, Formatter, Result};
                use super::*;
                impl<const N: usize, S: VecStorage, T: Scalar> Debug for Vector<N, S, T>
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
                impl<const N: usize, S: VecStorage, T: Scalar> Display
                for Vector<N, S, T>
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
                impl<const N: usize, S: VecStorage, T: Scalar> PartialEq
                for Vector<N, S, T>
                where
                    ScalarCount<N>: VecLen<N>,
                {
                    #[inline(always)]
                    fn eq(&self, other: &Self) -> bool {
                        self.inner.eq(&other.inner)
                    }
                }
            }
        }
        pub mod or_scalar {
            use super::*;
            pub type VectorOrScalar<const N: usize, S, T> = <ScalarCount<
                N,
            > as VecLenOrOne>::VectorOrScalar<S, T>;
            pub type InnerVectorOrScalar<const N: usize, S, T> = <ScalarCount<
                N,
            > as VecLenOrOne>::InnerVectorOrScalar<S, T>;
            #[allow(private_bounds)]
            pub trait VecLenOrOne: Seal {
                type VectorOrScalar<S: VecStorage, T: Scalar>: Construct;
                type InnerVectorOrScalar<S: VecStorage, T: Scalar>: InnerConstruct;
            }
            impl Seal for ScalarCount<1> {}
            impl Seal for ScalarCount<2> {}
            impl Seal for ScalarCount<3> {}
            impl Seal for ScalarCount<4> {}
            impl VecLenOrOne for ScalarCount<1> {
                type VectorOrScalar<S: VecStorage, T: Scalar> = T;
                type InnerVectorOrScalar<S: VecStorage, T: Scalar> = T;
            }
            impl VecLenOrOne for ScalarCount<2> {
                type VectorOrScalar<S: VecStorage, T: Scalar> = Vector<2, S, T>;
                type InnerVectorOrScalar<S: VecStorage, T: Scalar> = InnerVector<
                    2,
                    S,
                    T,
                >;
            }
            impl VecLenOrOne for ScalarCount<3> {
                type VectorOrScalar<S: VecStorage, T: Scalar> = Vector<3, S, T>;
                type InnerVectorOrScalar<S: VecStorage, T: Scalar> = InnerVector<
                    3,
                    S,
                    T,
                >;
            }
            impl VecLenOrOne for ScalarCount<4> {
                type VectorOrScalar<S: VecStorage, T: Scalar> = Vector<4, S, T>;
                type InnerVectorOrScalar<S: VecStorage, T: Scalar> = InnerVector<
                    4,
                    S,
                    T,
                >;
            }
            trait Seal {}
        }
        pub trait ScalarVecApiImpl<
            const N: usize,
            S: VecStorage,
        >: ScalarAlignedVecs + array::ScalarVecArrayApi<N, S>
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
        pub(super) trait VecStorageApi: array::VecStorageArrayApi<
                2,
            > + array::VecStorageArrayApi<3> + array::VecStorageArrayApi<4> {}
        impl VecStorageApi for VecPacked {}
        impl VecStorageApi for VecAligned {}
    }
    pub use api::*;
    #[repr(transparent)]
    pub struct Vector<const N: usize, S: VecStorage, T: Scalar>
    where
        ScalarCount<N>: VecLen<N>,
    {
        inner: InnerVector<N, S, T>,
    }
    pub type Vec2S<S, T> = Vector<2, S, T>;
    pub type Vec3S<S, T> = Vector<3, S, T>;
    pub type Vec4S<S, T> = Vector<4, S, T>;
    pub type VecN<const N: usize, T> = Vector<N, VecAligned, T>;
    pub type Vec2<T> = Vector<2, VecAligned, T>;
    pub type Vec3<T> = Vector<3, VecAligned, T>;
    pub type Vec4<T> = Vector<4, VecAligned, T>;
    pub type VecNP<const N: usize, T> = Vector<N, VecPacked, T>;
    pub type Vec2P<T> = Vector<2, VecPacked, T>;
    pub type Vec3P<T> = Vector<3, VecPacked, T>;
    pub type Vec4P<T> = Vector<4, VecPacked, T>;
    pub type InnerVector<const N: usize, S, T> = <S as VecStorageInnerVecs>::InnerVec<
        N,
        T,
    >;
}
