use crate as gomath;

use crate::scalar::default_impl::scalar_default_impl;

#[cfg(target_pointer_width = "32")]
scalar_default_impl!(
    <isize>(4):
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
    AlignedISizeVec2, AlignedISizeVec4
);

#[cfg(target_pointer_width = "64")]
scalar_default_impl!(
    <isize>(8):
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
    AlignedISizeVec2, AlignedISizeVec4
);
