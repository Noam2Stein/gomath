use super::*;

pub trait VecNSet<T: Element, const N: usize>: VecNGetMut<T, N> {
    fn set(&mut self, x: usize, value: T) -> Result<(), &'static str> {
        if x >= N {
            Err("x outside of vec bounds")
        } else {
            unsafe {
                self.set_unchecked(x, value);
            }

            Ok(())
        }
    }
    fn set2(&mut self, x: usize, y: usize, value: Vec2<T>) -> Result<(), &'static str> {
        if x >= N {
            Err("x outside of vec bounds")
        } else if y >= N {
            Err("y outside of vec bounds")
        } else if y == x {
            Err("y conflicts with x")
        } else {
            unsafe {
                self.set2_unchecked(x, y, value);
            }

            Ok(())
        }
    }
    fn set3(&mut self, x: usize, y: usize, z: usize, value: Vec3<T>) -> Result<(), &'static str> {
        if x >= N {
            Err("x outside of vec bounds")
        } else if y >= N {
            Err("y outside of vec bounds")
        } else if z >= N {
            Err("z outside of vec bounds")
        } else if y == x {
            Err("y conflicts with x")
        } else if z == x {
            Err("z conflicts with x")
        } else if z == y {
            Err("z conflicts with y")
        } else {
            unsafe {
                self.set3_unchecked(x, y, z, value);
            }

            Ok(())
        }
    }
    fn set4(
        &mut self,
        x: usize,
        y: usize,
        z: usize,
        w: usize,
        value: Vec4<T>,
    ) -> Result<(), &'static str> {
        if x >= N {
            Err("x outside of vec bounds")
        } else if y >= N {
            Err("y outside of vec bounds")
        } else if z >= N {
            Err("z outside of vec bounds")
        } else if w >= N {
            Err("w outside of vec bounds")
        } else if y == x {
            Err("y conflicts with x")
        } else if z == x {
            Err("z conflicts with x")
        } else if w == x {
            Err("w conflicts with x")
        } else if z == y {
            Err("z conflicts with y")
        } else if w == y {
            Err("w conflicts with y")
        } else if w == z {
            Err("w conflicts with z")
        } else {
            unsafe {
                self.set4_unchecked(x, y, z, w, value);
            }

            Ok(())
        }
    }

    #[inline(always)]
    unsafe fn set_unchecked(&mut self, x: usize, value: T) {
        *self.get_unchecked_mut(x) = value;
    }
    #[inline(always)]
    unsafe fn set2_unchecked(&mut self, x: usize, y: usize, value: Vec2<T>) {
        *self.get_unchecked_mut(x) = value.x();
        *self.get_unchecked_mut(y) = value.y();
    }
    #[inline(always)]
    unsafe fn set3_unchecked(&mut self, x: usize, y: usize, z: usize, value: Vec3<T>) {
        *self.get_unchecked_mut(x) = value.x();
        *self.get_unchecked_mut(y) = value.y();
        *self.get_unchecked_mut(z) = value.z();
    }
    #[inline(always)]
    unsafe fn set4_unchecked(&mut self, x: usize, y: usize, z: usize, w: usize, value: Vec4<T>) {
        *self.get_unchecked_mut(x) = value.x();
        *self.get_unchecked_mut(y) = value.y();
        *self.get_unchecked_mut(z) = value.z();
        *self.get_unchecked_mut(w) = value.w();
    }
}

impl<T: Element> VecNSet<T, 2> for Vec2<T> {}
impl<T: Element> VecNSet<T, 3> for Vec3<T> {}
impl<T: Element> VecNSet<T, 4> for Vec4<T> {}
