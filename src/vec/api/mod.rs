use super::*;

pub mod array;
pub mod construct;
pub mod or_scalar;

pub trait ScalarVecApiImpl<const N: usize, S: VecStorage>:
    ScalarInnerVecs + array::ScalarVecArrayImpl<N, S>
where
    ScalarCount<N>: VecLen<N>,
{
}

pub(super) trait VecLenApi<const N: usize>: array::VecLenArrayApi<N>
where
    ScalarCount<N>: VecLen<N>,
{
}
impl VecLenApi<2> for ScalarCount<2> {}
impl VecLenApi<3> for ScalarCount<3> {}
impl VecLenApi<4> for ScalarCount<4> {}

pub(super) trait VecStorageApi:
    array::VecStorageArray<2> + array::VecStorageArray<3> + array::VecStorageArray<4>
{
}
impl VecStorageApi for VecPacked {}
impl VecStorageApi for VecAligned {}
