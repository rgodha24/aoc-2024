use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
    str::FromStr,
};

use num::{BigInt, Bounded, Num, NumCast, Signed};

/// a point in a 2d space
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GenericPoint<N: Num + Clone + Copy> {
    pub x: N,
    pub y: N,
}

/// 2d point backed by i64
pub type SignedPoint = GenericPoint<i64>;
/// 2d point backed by usize (which is basically always a u64)
pub type Point = GenericPoint<usize>;
/// 2d point backed by num::BigInt for arbitrary sized numbers
pub type BIPoint = GenericPoint<BigInt>;

impl<N: Num + Clone + Copy + Bounded> GenericPoint<N> {
    pub fn max() -> Self {
        Self::new(N::max_value(), N::max_value())
    }
}

impl<N: Num + Clone + Copy> GenericPoint<N> {
    pub const fn new(x: N, y: N) -> Self {
        Self { x, y }
    }

    pub fn from_delimited(str: &str, delimiter: &str) -> Option<Self>
    where
        N: FromStr,
    {
        let (x, y) = str.split_once(delimiter)?;
        let x = x.parse().ok()?;
        let y = y.parse().ok()?;
        Some(Self::new(x, y))
    }

    pub fn neighbors(&self) -> [Self; 4] {
        [
            Self::new(self.x - N::one(), self.y),
            Self::new(self.x + N::one(), self.y),
            Self::new(self.x, self.y - N::one()),
            Self::new(self.x, self.y + N::one()),
        ]
    }

    pub fn neighbors_diag(&self) -> [Self; 8] {
        [
            Self::new(self.x - N::one(), self.y),
            Self::new(self.x + N::one(), self.y),
            Self::new(self.x, self.y - N::one()),
            Self::new(self.x, self.y + N::one()),
            Self::new(self.x - N::one(), self.y - N::one()),
            Self::new(self.x + N::one(), self.y - N::one()),
            Self::new(self.x - N::one(), self.y + N::one()),
            Self::new(self.x + N::one(), self.y + N::one()),
        ]
    }

    pub fn neighbors_just_diag(&self) -> [Self; 4] {
        [
            Self::new(self.x + N::one(), self.y - N::one()),
            Self::new(self.x + N::one(), self.y + N::one()),
            Self::new(self.x - N::one(), self.y + N::one()),
            Self::new(self.x - N::one(), self.y - N::one()),
        ]
    }

    pub fn manhattan_distance(&self, rhs: &Self) -> N
    where
        N: Signed,
    {
        // abs_sub is a weird function..
        num::abs_sub(self.x, rhs.x)
            + num::abs_sub(rhs.x, self.x)
            + num::abs_sub(self.y, rhs.y)
            + num::abs_sub(rhs.y, self.y)
    }

    pub fn as_point(self) -> Option<Point>
    where
        N: NumCast,
    {
        let x: usize = num::cast(self.x)?;
        let y: usize = num::cast(self.y)?;

        Some(Point { x, y })
    }
    pub fn as_signed_point(self) -> SignedPoint
    where
        N: NumCast,
    {
        {
            let x: i64 = num::cast(self.x).unwrap();
            let y: i64 = num::cast(self.y).unwrap();

            SignedPoint { x, y }
        }
    }

    pub fn cast<T>(self) -> GenericPoint<T>
    where
        N: NumCast,
        T: Num + Clone + Copy + NumCast,
    {
        let x: T = num::cast(self.x).unwrap();
        let y: T = num::cast(self.y).unwrap();

        GenericPoint::new(x, y)
    }

    /// the cross product of two 2d vectors is always in the third dimension,
    /// but this still achieves the goal of having p.cross(other) == 0 only when
    /// the 2 points are parallel
    pub fn cross(self, other: &Self) -> N {
        (self.x * other.y) - (self.y * other.x)
    }

    /// min is inclusive, max is exclusive (just like how grid.contains_point works)
    pub fn is_contained_by(&self, min: &Self, max: &Self) -> bool
    where
        N: Ord,
    {
        self.x >= min.x && self.y >= min.y && self.x < max.x && self.y < max.y
    }
}

impl<N: Num + Clone + Copy + Neg<Output = N>> Neg for GenericPoint<N> {
    type Output = GenericPoint<N>;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl<N: Num + Clone + Copy> Add for GenericPoint<N> {
    type Output = GenericPoint<N>;

    fn add(self, rhs: Self) -> Self::Output {
        GenericPoint {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<N: Num + Clone + Copy> Sub for GenericPoint<N> {
    type Output = GenericPoint<N>;

    fn sub(self, rhs: Self) -> Self::Output {
        GenericPoint {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<N> Mul<N> for GenericPoint<N>
where
    N: Num + Clone + Copy,
{
    type Output = GenericPoint<N>;

    fn mul(self, rhs: N) -> Self::Output {
        GenericPoint::new(self.x * rhs, self.y * rhs)
    }
}
impl<N> Div<N> for GenericPoint<N>
where
    N: Num + Clone + Copy,
{
    type Output = GenericPoint<N>;

    fn div(self, rhs: N) -> Self::Output {
        GenericPoint::new(self.x / rhs, self.y / rhs)
    }
}

impl<N> Display for GenericPoint<N>
where
    N: Num + Clone + Copy + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl<N> From<(N, N)> for GenericPoint<N>
where
    N: Num + Clone + Copy,
{
    fn from((x, y): (N, N)) -> Self {
        GenericPoint::new(x, y)
    }
}
