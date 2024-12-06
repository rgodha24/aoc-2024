use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

use num::Num;
use Direction::*;

use super::*;

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Right => Left,
            Left => Right,
            Up => Down,
            Down => Up,
        }
    }

    /// returns every direction except for self
    pub fn except_self(&self) -> Vec<Self> {
        match self {
            Right => vec![Up, Down, Left],
            Left => vec![Up, Down, Right],
            Up => vec![Left, Right, Down],
            Down => vec![Left, Right, Up],
        }
    }

    pub fn all() -> Vec<Self> {
        vec![Right, Left, Up, Down]
    }

    pub fn except_self_and_opposite(&self) -> Vec<Self> {
        match self {
            Right => vec![Up, Down],
            Left => vec![Up, Down],
            Up => vec![Left, Right],
            Down => vec![Left, Right],
        }
    }

    pub fn except_opposite(&self) -> Vec<Self> {
        match self {
            Right => vec![Up, Down, Right],
            Left => vec![Up, Down, Left],
            Up => vec![Left, Right, Down],
            Down => vec![Left, Right, Up],
        }
    }

    pub fn from_number(num: usize) -> Self {
        match num % 4 {
            0 => Up,
            1 => Right,
            2 => Down,
            3 => Left,
            _ => unreachable!(),
        }
    }

    pub fn to_number(self) -> usize {
        match self {
            Up => 0,
            Right => 1,
            Down => 2,
            Left => 3,
        }
    }

    pub fn as_point<N>(&self) -> GenericPoint<N>
    where
        N: Num + Clone + Copy + Neg<Output = N>,
    {
        match self {
            Right => GenericPoint::new(N::one(), N::zero()),
            Left => GenericPoint::new(-N::one(), N::zero()),
            Up => GenericPoint::new(N::zero(), -N::one()),
            Down => GenericPoint::new(N::zero(), N::one()),
        }
    }
}

impl Add<Direction> for Direction {
    type Output = Direction;

    fn add(self, rhs: Direction) -> Self::Output {
        Direction::from_number(self.to_number() + rhs.to_number())
    }
}

impl Sub<Direction> for Direction {
    type Output = Direction;

    fn sub(self, rhs: Direction) -> Self::Output {
        Direction::from_number(self.to_number() - rhs.to_number())
    }
}

impl<N: Num + Clone + Copy> Add<Direction> for GenericPoint<N> {
    type Output = GenericPoint<N>;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Up => GenericPoint::new(self.x, self.y - N::one()),
            Direction::Down => GenericPoint::new(self.x, self.y + N::one()),
            Direction::Left => GenericPoint::new(self.x - N::one(), self.y),
            Direction::Right => GenericPoint::new(self.x + N::one(), self.y),
        }
    }
}

impl<N: Num + Clone + Copy> Sub<Direction> for GenericPoint<N> {
    type Output = GenericPoint<N>;

    fn sub(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Up => GenericPoint::new(self.x, self.y + N::one()),
            Direction::Down => GenericPoint::new(self.x, self.y - N::one()),
            Direction::Left => GenericPoint::new(self.x + N::one(), self.y),
            Direction::Right => GenericPoint::new(self.x - N::one(), self.y),
        }
    }
}

impl<N: Num + Clone + Copy + AddAssign + SubAssign> AddAssign<Direction> for GenericPoint<N> {
    fn add_assign(&mut self, rhs: Direction) {
        match rhs {
            Direction::Up => self.y -= N::one(),
            Direction::Down => self.y += N::one(),
            Direction::Left => self.x -= N::one(),
            Direction::Right => self.x += N::one(),
        }
    }
}

impl<N: Num + Clone + Copy + AddAssign + SubAssign> SubAssign<Direction> for GenericPoint<N> {
    fn sub_assign(&mut self, rhs: Direction) {
        match rhs {
            Direction::Up => self.y += N::one(),
            Direction::Down => self.y -= N::one(),
            Direction::Left => self.x += N::one(),
            Direction::Right => self.x -= N::one(),
        }
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'R' | 'r' | '>' => Right,
            'L' | 'l' | '<' => Left,
            'U' | 'u' | '^' => Up,
            'D' | 'd' | 'v' => Down,
            c => panic!("invalid direction char: {}", c),
        }
    }
}

impl From<Direction> for char {
    fn from(val: Direction) -> Self {
        match val {
            Right => '>',
            Left => '<',
            Up => '^',
            Down => 'v',
        }
    }
}
