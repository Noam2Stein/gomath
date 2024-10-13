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
            type InnerVector<S: VecStorageInnerVecs, T: ScalarInnerVecs>: InnerConstruct;
        }
        pub struct ScalarCount<const VALUE: usize>;
        impl Seal for ScalarCount<2> {}
        impl Seal for ScalarCount<4> {}
        impl Seal for ScalarCount<3> {}
        impl VecLen<2> for ScalarCount<2> {}
        impl VecLen<3> for ScalarCount<3> {}
        impl VecLen<4> for ScalarCount<4> {}
        impl VecLenInnerVec for ScalarCount<2> {
            type InnerVector<S: VecStorageInnerVecs, T: ScalarInnerVecs> = S::InnerVec2<
                T,
            >;
        }
        impl VecLenInnerVec for ScalarCount<3> {
            type InnerVector<S: VecStorageInnerVecs, T: ScalarInnerVecs> = S::InnerVec3<
                T,
            >;
        }
        impl VecLenInnerVec for ScalarCount<4> {
            type InnerVector<S: VecStorageInnerVecs, T: ScalarInnerVecs> = S::InnerVec4<
                T,
            >;
        }
        trait Seal {}
    }
    mod scalar {
        use super::*;
        pub trait ScalarVec: ScalarInnerVecs + ScalarVecApiImpl<
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
        pub(super) trait VecStorageInnerVecs: Sized + Seal {
            type InnerVec2<T: ScalarInnerVecs>: InnerConstruct;
            type InnerVec3<T: ScalarInnerVecs>: InnerConstruct;
            type InnerVec4<T: ScalarInnerVecs>: InnerConstruct;
        }
        pub struct VecPacked;
        impl Seal for VecPacked {}
        impl VecStorage for VecPacked {}
        impl VecStorageInnerVecs for VecPacked {
            type InnerVec2<T: ScalarInnerVecs> = [T; 2];
            type InnerVec3<T: ScalarInnerVecs> = [T; 3];
            type InnerVec4<T: ScalarInnerVecs> = [T; 4];
        }
        pub struct VecAligned;
        impl Seal for VecAligned {}
        impl VecStorage for VecAligned {}
        impl VecStorageInnerVecs for VecAligned {
            type InnerVec2<T: ScalarInnerVecs> = T::InnerAlignedVec2;
            type InnerVec3<T: ScalarInnerVecs> = T::InnerAlignedVec3;
            type InnerVec4<T: ScalarInnerVecs> = T::InnerAlignedVec4;
        }
        pub unsafe trait ScalarInnerVecs: Construct {
            type InnerAlignedVec2: InnerConstruct;
            type InnerAlignedVec3: InnerConstruct;
            type InnerAlignedVec4: InnerConstruct;
        }
        trait Seal {}
    }
    pub use len::*;
    pub use scalar::*;
    pub use storage::*;
    mod api {
        use super::*;
        pub mod array {
            use gomath_proc_macros::vec_api;
            use super::*;
            impl<const N: usize, S: VecStorage, T: Scalar> Vector<N, S, T>
            where
                ScalarCount<N>: VecLen<N>,
            {
                #[inline(always)]
                pub fn from_array(array: [T; N]) -> Self {
                    Self {
                        inner: <ScalarCount<N> as TheNTrait>::from_array::<S, T>(array),
                    }
                }
                #[inline(always)]
                pub fn into_array(self) -> [T; N] {
                    <ScalarCount<N> as TheNTrait>::into_array::<S, T>(self.inner)
                }
                #[inline(always)]
                pub fn as_array(&self) -> &[T; N] {
                    unsafe {
                        std::mem::transmute(
                            <ScalarCount<N> as TheNTrait>::as_array::<S, T>(&self.inner),
                        )
                    }
                }
                #[inline(always)]
                pub fn as_array_mut(&mut self) -> &mut [T; N] {
                    unsafe {
                        std::mem::transmute(
                            <ScalarCount<
                                N,
                            > as TheNTrait>::as_array_mut::<S, T>(&mut self.inner),
                        )
                    }
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
                        ::core::panicking::panic("not yet implemented")
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
            pub type VectorOrScalar<const N: usize, S: VecStorage, T> = <ScalarCount<
                N,
            > as VecLenOrOne>::Construct<S, T>;
            pub type VecNInnerOrScalar<const N: usize, S: VecStorage, T> = <ScalarCount<
                N,
            > as VecLenOrOne>::Construct<S, T>;
            #[allow(private_bounds)]
            pub trait VecLenOrOne: Seal {
                type Construct<S: VecStorage, T: Scalar>: Construct;
                type InnerConstruct<S: VecStorage, T: Scalar>: InnerConstruct;
            }
            impl Seal for ScalarCount<1> {}
            impl Seal for ScalarCount<2> {}
            impl Seal for ScalarCount<3> {}
            impl Seal for ScalarCount<4> {}
            impl VecLenOrOne for ScalarCount<1> {
                type Construct<S: VecStorage, T: Scalar> = T;
                type InnerConstruct<S: VecStorage, T: Scalar> = T;
            }
            impl VecLenOrOne for ScalarCount<2> {
                type Construct<S: VecStorage, T: Scalar> = Vector<2, S, T>;
                type InnerConstruct<S: VecStorage, T: Scalar> = InnerVector<2, S, T>;
            }
            impl VecLenOrOne for ScalarCount<3> {
                type Construct<S: VecStorage, T: Scalar> = Vector<3, S, T>;
                type InnerConstruct<S: VecStorage, T: Scalar> = InnerVector<3, S, T>;
            }
            impl VecLenOrOne for ScalarCount<4> {
                type Construct<S: VecStorage, T: Scalar> = Vector<4, S, T>;
                type InnerConstruct<S: VecStorage, T: Scalar> = InnerVector<4, S, T>;
            }
            trait Seal {}
        }
        pub trait ScalarVecApiImpl<
            const N: usize,
            S: VecStorage,
        >: ScalarInnerVecs + array::ScalarVecArrayImpl<N, S>
        where
            ScalarCount<N>: VecLen<N>,
        {}
        pub(super) trait VecLenApi<const N: usize>: array::VecLenArray<N>
        where
            ScalarCount<N>: VecLen<N>,
        {}
        impl VecLenApi<2> for ScalarCount<2> {}
        impl VecLenApi<3> for ScalarCount<3> {}
        impl VecLenApi<4> for ScalarCount<4> {}
        pub(super) trait VecStorageApi: array::VecStorageArray<
                2,
            > + array::VecStorageArray<3> + array::VecStorageArray<4> {}
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
    pub type InnerVector<const N: usize, S, T> = <ScalarCount<
        N,
    > as VecLenInnerVec>::InnerVector<S, T>;
}
