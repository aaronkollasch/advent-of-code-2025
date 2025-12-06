extern crate num;

use num::{Num, PrimInt, Signed, Unsigned};
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, Sub, SubAssign};
use std::{
    fmt::{Binary, Display},
    iter::{Product, Sum},
    mem::size_of,
    ops::{BitAnd, BitOrAssign, Shl, Shr},
};

#[inline]
pub fn parse_u8(b: &[u8]) -> u8 {
    b.iter().fold(0, |acc, x| acc * 10 + (x - b'0'))
}

#[inline]
pub fn parse<T>(b: &[u8]) -> T
where
    T: PrimInt + Sum + Product,
{
    b.iter().fold(T::zero(), |acc, x| {
        acc * T::from(10).unwrap() + T::from(x - b'0').unwrap()
    })
}

#[inline]
pub fn parse_iter<T, I>(iter: I) -> T
where
    I: Iterator<Item = u8>,
    T: PrimInt + Sum + Product,
{
    iter.fold(T::zero(), |acc, x| {
        acc * T::from(10).unwrap() + T::from(x - b'0').unwrap()
    })
}

// TODO: make parse_iter into a generic method for any iterator,
// so you can do I.parse::<T>() -> T
// pub trait Parser {
//     fn parse<T>(self) -> T;
// }
// pub trait Parser<A = Self>: Sized {
//     /// Takes an iterator and generates `Self` from the elements by parsing
//     /// the items.
//     fn parse<I: Iterator<Item = u8>>(iter: I) -> Self;
// }
//
// macro_rules! integer_sum_product {
//     (@impls $zero:expr, $one:expr, #[$attr:meta], $($a:ty)*) => ($(
//         #[$attr]
//         impl Parser for $a {
//             fn parse<I: Iterator<Item=u8>>(iter: I) -> Self {
//                 iter.fold($zero, |acc, x| {
//                     acc * $a::from(10).unwrap() + $a::from(x - b'0').unwrap()
//                 })
//             }
//         }
//     )*);
//     ($($a:ty)*) => (
//         integer_sum_product!(@impls 0, 1,
//                 $($a)*);
//         integer_sum_product!(@impls Wrapping(0), Wrapping(1),
//                 $(Wrapping<$a>)*);
//     );
// }
//
// integer_sum_product! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
//
// // impl Parser for T
// // where
// //     T: PrimInt + Sum + Product,
// // {
// //     fn parse<I: Iterator<Item=u8>>(iter: I) -> Self {
// //         iter.fold(T::zero(), |acc, x| {
// //             acc * T::from(10).unwrap() + T::from(x - b'0').unwrap()
// //         })
// //     }
// // }
//
//
// impl<I> Parser for I
// where
//     I: Iterator<Item = u8>,
// {
//     fn parse<T>(self) -> T
//     where
//         Self: Sized,
//         T: Parser<Self::Item>,
//     {
//         Parser::parse(self)
//     }
//     // #[inline]
//     // fn parse<T>(self) -> T
//     // where
//     //     Self: Sized,
//     //     T: PrimInt + Sum + Product,
//     // {
//     //     // This is too aggressive to turn on for everything all the time, but PR#137908
//     //     // accidentally noticed that some rustc iterators had malformed `size_hint`s,
//     //     // so this will help catch such things in debug-assertions-std runners,
//     //     // even if users won't actually ever see it.
//     //     if cfg!(debug_assertions) {
//     //         let hint = self.size_hint();
//     //         assert!(hint.1.is_none_or(|high| high >= hint.0), "Malformed size_hint {hint:?}");
//     //     }
//     //
//     //     FromIterator::from_iter(self)
//     // }
//
//     // fn parse<T>(self) -> T
//     // where
//     //     Self: Sized,
//     //     T: PrimInt + Sum + Product,
//     // {
//     //     self.fold(T::zero(), |acc, x| {
//     //         acc * T::from(10).unwrap() + T::from(x - b'0').unwrap()
//     //     })
//     // }
// }

#[inline]
pub fn parse_signed<T>(b: &[u8]) -> T
where
    T: PrimInt + Signed + Sum + Product,
{
    match b[0] {
        b'-' => {
            b[1..].iter().fold(T::zero(), |acc, x| {
                acc * T::from(10).unwrap() + T::from(x - b'0').unwrap()
            }) * T::from(-1).unwrap()
        }
        _ => b.iter().fold(T::zero(), |acc, x| {
            acc * T::from(10).unwrap() + T::from(x - b'0').unwrap()
        }),
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct BitVec<T> {
    vec: T,
    size: usize,
}

impl<T> BitVec<T>
where
    T: PrimInt
        + Unsigned
        + BitOrAssign
        + Shr<Output = T>
        + Shl<Output = T>
        + BitAnd<Output = T>
        + Display,
{
    pub fn new(size: usize) -> Self {
        if size > size_of::<T>() * 8 {
            panic!("too many bits for BitVec: {}", size);
        }
        Self {
            vec: T::zero(),
            size,
        }
    }

    #[inline]
    pub fn set_bit(&mut self, pos: T) {
        self.vec |= T::one() << pos;
    }

    #[inline]
    pub fn get_bit(&self, pos: T) -> T {
        (self.vec >> pos) & T::one()
    }

    #[inline]
    pub fn iter_unset(&self) -> impl Iterator<Item = T> + '_ {
        num::iter::range(T::zero(), T::from(self.size).unwrap())
            .filter(|i| self.get_bit(*i) == T::zero())
    }
}

impl<T> fmt::Display for BitVec<T>
where
    T: PrimInt + Binary,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "BitVec({})",
            format!("{:064b}", self.vec.reverse_bits())
                .split_at(self.size)
                .0
        )
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Vec2<T: Num> {
    pub x: T,
    pub y: T,
}

impl<T: Num> Add<(T, T)> for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: (T, T)) -> Vec2<T> {
        Vec2::<T> {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl<T: Num> Add<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2::<T> {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> AddAssign<(T, T)> for Vec2<T>
where
    T: Num + AddAssign,
{
    fn add_assign(&mut self, rhs: (T, T)) {
        self.x += rhs.0;
        self.y += rhs.1;
    }
}

impl<T> AddAssign<Vec2<T>> for Vec2<T>
where
    T: Num + AddAssign,
{
    fn add_assign(&mut self, rhs: Vec2<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Num> Sub<(T, T)> for Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: (T, T)) -> Vec2<T> {
        Vec2::<T> {
            x: self.x - rhs.0,
            y: self.y - rhs.1,
        }
    }
}

impl<T: Num> Sub<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2::<T> {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> SubAssign<(T, T)> for Vec2<T>
where
    T: Num + SubAssign,
{
    fn sub_assign(&mut self, rhs: (T, T)) {
        self.x -= rhs.0;
        self.y -= rhs.1;
    }
}

impl<T> SubAssign<Vec2<T>> for Vec2<T>
where
    T: Num + SubAssign,
{
    fn sub_assign(&mut self, rhs: Vec2<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Num> Mul<(T, T)> for Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: (T, T)) -> Vec2<T> {
        Vec2::<T> {
            x: self.x * rhs.0,
            y: self.y * rhs.1,
        }
    }
}
impl<T: Num> Mul<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2::<T> {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T> MulAssign<(T, T)> for Vec2<T>
where
    T: Num + MulAssign,
{
    fn mul_assign(&mut self, rhs: (T, T)) {
        self.x *= rhs.0;
        self.y *= rhs.1;
    }
}

impl<T> MulAssign<Vec2<T>> for Vec2<T>
where
    T: Num + MulAssign,
{
    fn mul_assign(&mut self, rhs: Vec2<T>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T: Num> Div<(T, T)> for Vec2<T> {
    type Output = Vec2<T>;

    fn div(self, rhs: (T, T)) -> Vec2<T> {
        Vec2::<T> {
            x: self.x / rhs.0,
            y: self.y / rhs.1,
        }
    }
}

impl<T: Num> Div<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn div(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2::<T> {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T> DivAssign<(T, T)> for Vec2<T>
where
    T: Num + DivAssign,
{
    fn div_assign(&mut self, rhs: (T, T)) {
        self.x /= rhs.0;
        self.y /= rhs.1;
    }
}

impl<T> DivAssign<Vec2<T>> for Vec2<T>
where
    T: Num + DivAssign,
{
    fn div_assign(&mut self, rhs: Vec2<T>) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl<T: Num> Rem<(T, T)> for Vec2<T> {
    type Output = Vec2<T>;

    fn rem(self, rhs: (T, T)) -> Vec2<T> {
        Vec2::<T> {
            x: self.x.rem(rhs.0),
            y: self.y.rem(rhs.1),
        }
    }
}

impl<T: Num> Rem<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn rem(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2::<T> {
            x: self.x.rem(rhs.x),
            y: self.y.rem(rhs.y),
        }
    }
}

impl<T: Num> From<(T, T)> for Vec2<T> {
    fn from(value: (T, T)) -> Self {
        Vec2 {
            x: value.0,
            y: value.1,
        }
    }
}

impl<T> PartialEq<(T, T)> for Vec2<T>
where
    T: Num,
{
    fn eq(&self, other: &(T, T)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}

impl<T> fmt::Display for Vec2<T>
where
    T: Num + Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "({}, {})", self.x, self.y)
    }
}
