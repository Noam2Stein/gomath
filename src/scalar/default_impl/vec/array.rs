use std::mem::transmute_copy;

use super::*;

impl<T: ScalarDefaultImpl> ScalarVecArrayApi<2, VecAligned> for T {
    fn from_array(array: [Self; 2]) -> InnerVector<2, Self, VecAligned> {
        unsafe { transmute_copy(&array) }
    }
}
impl<T: ScalarDefaultImpl> ScalarVecArrayApi<3, VecAligned> for T {
    fn from_array(array: [Self; 3]) -> InnerVector<3, Self, VecAligned> {
        unsafe { transmute_copy(&[array[0], array[1], array[2], array[2]]) }
    }
}
impl<T: ScalarDefaultImpl> ScalarVecArrayApi<4, VecAligned> for T {
    fn from_array(array: [Self; 4]) -> InnerVector<4, Self, VecAligned> {
        unsafe { transmute_copy(&array) }
    }
}
