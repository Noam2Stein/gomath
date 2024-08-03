use std::fmt::{Debug, Display};
use private::Sealed;

pub trait Element:
Sealed +
Send +
Sync +
Debug +
Clone +
Copy +
PartialEq +
PartialOrd +
Display +
Default +
{
    
}
pub trait Num:
Element +
num::traits::NumAssign +
{
    const ZERO: Self;
    const ONE: Self;
}
pub trait SignedNum:
Num +
num::Signed +
{
    const NEG_ONE: Self;
}
pub trait UnsignedNum:
Num +
num::Unsigned +
{

}
pub trait Int:
Num +
num::Integer +
{
    
}
pub trait UnsignedInt:
Int +
UnsignedNum +
{
    
}
pub trait SignedInt:
Int +
SignedNum +
{
    
}
pub trait Float:
SignedNum +
num::Float +
{

}

mod private {
    pub trait Sealed {
        
    }
}

impl Sealed for bool {}
impl Sealed for u8 {}
impl Sealed for u16 {}
impl Sealed for u32 {}
impl Sealed for u64 {}
impl Sealed for u128 {}
impl Sealed for usize {}
impl Sealed for i8 {}
impl Sealed for i16 {}
impl Sealed for i32 {}
impl Sealed for i64 {}
impl Sealed for i128 {}
impl Sealed for isize {}
impl Sealed for f32 {}
impl Sealed for f64 {}

impl Element for bool {

}
impl Element for u8 {

}
impl Element for u16 {

}
impl Element for u32 {

}
impl Element for u64 {

}
impl Element for u128 {

}
impl Element for usize {

}
impl Element for i8 {

}
impl Element for i16 {

}
impl Element for i32 {

}
impl Element for i64 {

}
impl Element for i128 {

}
impl Element for isize {

}
impl Element for f32 {

}
impl Element for f64 {

}
impl Num for u8 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}
impl Num for u16 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}
impl Num for u32 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}
impl Num for u64 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}
impl Num for u128 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}
impl Num for usize {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}
impl Num for i8 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}
impl Num for i16 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}
impl Num for i32 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}
impl Num for i64 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}
impl Num for i128 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}
impl Num for isize {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}
impl Num for f32 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}
impl Num for f64 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}
impl SignedNum for i8 {
    const NEG_ONE: Self = -1;
}
impl SignedNum for i16 {
    const NEG_ONE: Self = -1;
}
impl SignedNum for i32 {
    const NEG_ONE: Self = -1;
}
impl SignedNum for i64 {
    const NEG_ONE: Self = -1;
}
impl SignedNum for i128 {
    const NEG_ONE: Self = -1;
}
impl SignedNum for isize {
    const NEG_ONE: Self = -1;
}
impl SignedNum for f32 {
    const NEG_ONE: Self = -1.0;
}
impl SignedNum for f64 {
    const NEG_ONE: Self = -1.0;
}
impl UnsignedNum for u8 {

}
impl UnsignedNum for u16 {

}
impl UnsignedNum for u32 {

}
impl UnsignedNum for u64 {

}
impl UnsignedNum for u128 {

}
impl UnsignedNum for usize {

}
impl Int for u8 {

}
impl Int for u16 {

}
impl Int for u32 {

}
impl Int for u64 {

}
impl Int for u128 {

}
impl Int for usize {

}
impl Int for i8 {

}
impl Int for i16 {

}
impl Int for i32 {

}
impl Int for i64 {

}
impl Int for i128 {

}
impl Int for isize {

}
impl UnsignedInt for u8 {

}
impl UnsignedInt for u16 {

}
impl UnsignedInt for u32 {

}
impl UnsignedInt for u64 {

}
impl UnsignedInt for u128 {

}
impl UnsignedInt for usize {

}
impl SignedInt for i8 {

}
impl SignedInt for i16 {

}
impl SignedInt for i32 {

}
impl SignedInt for i64 {

}
impl SignedInt for i128 {

}
impl SignedInt for isize {

}
impl Float for f32 {

}
impl Float for f64 {

}