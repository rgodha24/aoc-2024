use std::ops::{Add, Mul, Neg, Sub};

use num::{BigInt, Num, NumCast, Signed};
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

impl<N: Num + Clone + Copy> GenericPoint<N> {
    pub fn new(x: N, y: N) -> Self {
        Self { x, y }
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

    pub fn manhattan_distance(&self, rhs: &Self) -> N
    where
        N: Signed,
    {
        num::abs(self.x - rhs.x) + num::abs(self.y - rhs.y)
    }

    pub fn as_point(self) -> Option<Point>
    where
        N: NumCast,
    {
        let x: usize = num::cast(self.x)?;
        let y: usize = num::cast(self.y)?;

        Some(Point { x, y })
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
    N: Num + Clone + Copy + Neg<Output = N>,
{
    type Output = GenericPoint<N>;

    fn mul(self, rhs: N) -> Self::Output {
        GenericPoint::new(self.x * rhs, self.y * rhs)
    }
}
