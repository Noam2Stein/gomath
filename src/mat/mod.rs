use std::fmt::{self, Display, Formatter};

use crate::{element::*, vec::*};

mod major;
pub use major::*;

trait Seal {}

#[allow(private_bounds)]
pub trait MatCxR: Seal + fmt::Debug + Copy + PartialEq + PartialOrd + Default + Display {
    type T: Element;
    const C: usize;
    const R: usize;
    type M: MatMajor;
}

macro_rules! mat {
    ($outer:ident($inner:ident): $c:literal * $r:literal) => {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        pub struct $outer<M: MatMajor, T: Element = f32> {
            inner: M::$inner<T>,
        }
        impl<M: MatMajor, T: Element> Seal for $outer<M, T> {}

        impl<M: MatMajor, T: Element> Display for $outer<M, T> {
            fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
                todo!()
            }
        }
        impl<M: MatMajor, T: Element> MatCxR for $outer<M, T> {
            type T = T;
            const C: usize = $c;
            const R: usize = $r;
            type M = M;
        }
    };
}
mat!(Mat2  (Mat2Inner  ): 2 * 2);
mat!(Mat2x3(Mat2x3Inner): 2 * 3);
mat!(Mat2x4(Mat2x4Inner): 2 * 4);
mat!(Mat3x2(Mat2Inner  ): 3 * 2);
mat!(Mat3  (Mat2x3Inner): 3 * 3);
mat!(Mat3x4(Mat2x4Inner): 3 * 4);
mat!(Mat4x2(Mat2Inner  ): 4 * 2);
mat!(Mat4x3(Mat2x3Inner): 4 * 3);
mat!(Mat4  (Mat2x4Inner): 4 * 4);
