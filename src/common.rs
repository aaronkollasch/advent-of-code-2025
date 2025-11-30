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
    T: PrimInt + Unsigned + Sum + Product,
{
    b.iter().fold(T::zero(), |acc, x| {
        acc * T::from(10).unwrap() + T::from(x - b'0').unwrap()
    })
}

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
