extern crate num;

use num::{Num, PrimInt, Signed, Unsigned};
use primitive_types::U256;
use std::{
    fmt::{self, Binary, Display},
    iter::{Product, Sum},
    mem::size_of,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOrAssign, Div, DivAssign, Mul, MulAssign, Rem,
        Shl, Shr, Sub, SubAssign,
    },
};
use u256_literal::u256;

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

pub trait ParseExt: Iterator {
    fn parse<P>(self) -> P
    where
        P: Parsed,
        Self: Sized + Iterator<Item = u8>,
    {
        P::parse(self)
    }
}

impl<I: Iterator> ParseExt for I {}

pub trait Parsed<A = Self> {
    fn parse<I>(iter: I) -> Self
    where
        I: Iterator<Item = u8>;
}

macro_rules! parser {
    (@impls $zero:expr, $one:expr, $($a:ty)*) => ($(
        impl Parsed for $a {
            fn parse<I: Iterator<Item=u8>>(iter: I) -> Self {
                iter.fold($zero, |acc, x: u8| {
                    acc * <$a>::from(10u8) + <$a>::from(x - b'0')
                })
            }
        }
    )*);
    ($($a:ty)*) => (
        parser!(@impls 0, 1,
                $($a)*);
    );
}

parser! { i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }

impl Parsed for i8 {
    fn parse<I: Iterator<Item = u8>>(iter: I) -> Self {
        iter.fold(0i8, |acc, x: u8| {
            acc * 10i8 + <i8>::try_from(x - b'0').unwrap()
        })
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
        + BitAndAssign
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
    pub fn unset_bit(&mut self, pos: T) {
        self.vec &= !(T::one() << pos);
    }

    #[inline]
    pub fn get_bit(&self, pos: T) -> T {
        (self.vec >> pos) & T::one()
    }

    #[inline]
    pub fn iter_set(&self) -> impl Iterator<Item = T> + '_ {
        num::iter::range(T::zero(), T::from(self.size).unwrap())
            .filter(|i| self.get_bit(*i) == T::one())
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
pub struct BitVec256 {
    vec: U256,
    size: usize,
}

impl BitVec256 {
    pub fn new(size: usize) -> Self {
        if size > size_of::<U256>() * 8 {
            panic!("too many bits for BitVec: {}", size);
        }
        Self {
            vec: u256!(0),
            size,
        }
    }

    #[inline]
    pub fn set_bit(&mut self, pos: U256) {
        self.vec |= u256!(1) << pos;
    }

    #[inline]
    pub fn unset_bit(&mut self, pos: usize) {
        self.vec &= !(u256!(1) << pos);
    }

    #[inline]
    pub fn get_bit(&self, pos: usize) -> U256 {
        (self.vec >> pos) & u256!(1)
    }

    #[inline]
    pub fn iter_set(&self) -> impl Iterator<Item = usize> + '_ {
        num::iter::range(0, self.size).filter(|i| self.get_bit(*i) == u256!(1))
    }

    #[inline]
    pub fn iter_unset(&self) -> impl Iterator<Item = usize> + '_ {
        num::iter::range(0, self.size).filter(|i| self.get_bit(*i) == u256!(0))
    }
}

impl fmt::Display for BitVec256 {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "BitVec256({})",
            self.vec
                .to_little_endian()
                .iter()
                .map(|b| format!("{:08b}", b.reverse_bits()))
                .collect::<String>()
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
