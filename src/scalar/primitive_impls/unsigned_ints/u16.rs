use crate as gomath;

use crate::scalar::default_impl::scalar_default_impl;

scalar_default_impl!(
    <u16>(2):
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
    AlignedU16Vec2, AlignedU16Vec4
);
