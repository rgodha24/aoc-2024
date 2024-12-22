#![allow(dead_code)]

use crate::helpers::*;
use num::NumCast;
use smallvec::{smallvec, SmallVec};
use std::{
    collections::{HashMap, HashSet},
    default::Default,
    fmt::{Debug, Display},
    ops::{Index, IndexMut},
};

#[derive(Clone, PartialEq, Hash)]
/// 2D grid of data stored in Row-Major order by default
pub struct Grid<T, const W: usize = 0> {
    // because the "union" feature on SmallVec is enabled, when N = 0, this takes up the same space
    // as a normal std::Vec
    data: Vec<SmallVec<[T; W]>>,
}

impl<T, const W: usize> Grid<T, W> {
    pub fn new(data: Vec<SmallVec<[T; W]>>) -> Self {
        Self { data }
    }

    pub fn into_inner(self) -> Vec<SmallVec<[T; W]>> {
        self.data
    }

    pub fn flat_iter(&self) -> impl Iterator<Item = (&T, Point)> {
        self.data.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, col)| (col, Point::new(x, y)))
        })
    }

    pub fn points<P>(&self) -> impl Iterator<Item = GenericPoint<P>>
    where
        P: Num + Clone + Copy + NumCast,
    {
        itertools::iproduct!(0..self.width(), 0..self.height())
            .map(|(x, y)| Point::new(x, y).cast())
    }

    pub fn width(&self) -> usize {
        self.data[0].len()
    }
    pub fn height(&self) -> usize {
        self.data.len()
    }
    pub fn dimensions(&self) -> Point {
        Point::new(self.width(), self.height())
    }
    pub fn from_lines(input: &str, line: impl Fn(&str) -> SmallVec<[T; W]>) -> Self {
        let data = input.lines().map(line).collect();
        Self { data }
    }

    pub fn contains_point<P>(&self, point: GenericPoint<P>) -> bool
    where
        P: Num + Clone + Copy + Ord + NumCast,
    {
        point.x >= P::zero()
            && point.y >= P::zero()
            && point.x < num::cast(self.width()).unwrap()
            && point.y < num::cast(self.height()).unwrap()
    }

    /// create a Grid from a string of characters
    /// the bottom left corner is (0, 0)
    pub fn from_chars(input: &str) -> Self
    where
        T: TryFrom<char>,
        <T as TryFrom<char>>::Error: Debug,
    {
        let data = input
            .lines()
            .map(|line| line.chars().map(|c| T::try_from(c).unwrap()).collect())
            .collect();
        Self { data }
    }

    /// iterates over the grid, allowing for mutations
    pub fn for_each_mut(&mut self, f: impl Fn(&mut T, Point)) {
        for (y, row) in self.data.iter_mut().enumerate() {
            for (x, col) in row.iter_mut().enumerate() {
                f(col, Point::new(x, y));
            }
        }
    }

    /// iterates over grid, immutably
    pub fn for_each(&self, mut f: impl FnMut(&T, Point)) {
        for (y, row) in self.data.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                f(col, Point::new(x, y));
            }
        }
    }

    pub fn neighbors_of(&self, p: Point) -> SmallVec<[Point; 4]> {
        let mut neighbors = SmallVec::new();
        let Point { x, y } = p;
        if x > 0 {
            neighbors.push(Point::new(x - 1, y));
        }
        if y > 0 {
            neighbors.push(Point::new(x, y - 1));
        }

        let width = self.width();
        let height = self.height();

        if x < width - 1 {
            neighbors.push(Point::new(x + 1, y));
        }
        if y < height - 1 {
            neighbors.push(Point::new(x, y + 1));
        }
        neighbors
    }

    pub fn neighbors_of_diagonal(&self, p: Point) -> SmallVec<[Point; 8]> {
        let mut neighbors = SmallVec::new();
        let width = self.width();
        let height = self.height();
        let Point { x, y } = p;
        if x > 0 {
            neighbors.push(Point::new(x - 1, y));
        }
        if y > 0 {
            neighbors.push(Point::new(x, y - 1));
        }
        if x < width - 1 {
            neighbors.push(Point::new(x + 1, y));
        }
        if y < height - 1 {
            neighbors.push(Point::new(x, y + 1));
        }
        if x > 0 && y > 0 {
            neighbors.push(Point::new(x - 1, y - 1));
        }
        if x < width - 1 && y > 0 {
            neighbors.push(Point::new(x + 1, y - 1));
        }
        if x > 0 && y < height - 1 {
            neighbors.push(Point::new(x - 1, y + 1));
        }
        if x < width - 1 && y < height - 1 {
            neighbors.push(Point::new(x + 1, y + 1));
        }

        neighbors
    }

    pub fn neighbors_of_filtered(&self, p: Point, f: impl Fn(&T, &Point) -> bool) -> Vec<Point> {
        self.neighbors_of(p)
            .into_iter()
            .filter(|p| f(&self[*p], p))
            .collect()
    }

    pub fn neighbors_of_diagonal_filtered(
        &self,
        p: Point,
        f: impl Fn(&T, &Point) -> bool,
    ) -> Vec<Point> {
        self.neighbors_of_diagonal(p)
            .into_iter()
            .filter(|p| f(&self[*p], p))
            .collect()
    }

    /// combine 2 grids into a grid of tuples
    pub fn combine2<T1, T2>(g1: Grid<T1>, g2: Grid<T2>) -> Grid<(T1, T2)> {
        let mut data = Vec::new();
        for (row1, row2) in g1.data.into_iter().zip(g2.data.into_iter()) {
            let mut row = SmallVec::new();
            for (col1, col2) in row1.into_iter().zip(row2.into_iter()) {
                row.push((col1, col2));
            }
            data.push(row);
        }
        Grid { data }
    }

    /// combine 3 grids into a grid of tuples
    pub fn combine3<T1, T2, T3>(g1: Grid<T1>, g2: Grid<T2>, g3: Grid<T3>) -> Grid<(T1, T2, T3)> {
        let mut data = Vec::new();
        for ((row1, row2), row3) in g1
            .data
            .into_iter()
            .zip(g2.data.into_iter())
            .zip(g3.data.into_iter())
        {
            let mut row = SmallVec::new();
            for ((col1, col2), col3) in row1.into_iter().zip(row2.into_iter()).zip(row3.into_iter())
            {
                row.push((col1, col2, col3));
            }
            data.push(row);
        }
        Grid { data }
    }

    pub fn rows(&self) -> impl Iterator<Item = &SmallVec<[T; W]>> {
        self.data.iter()
    }

    pub fn cols(&self) -> impl Iterator<Item = Vec<&T>> {
        let width = self.width();
        (0..width).map(move |i| self.data.iter().map(move |row| &row[i]).collect())
    }

    pub fn row(&self, y: usize) -> &SmallVec<[T; W]> {
        &self.data[y]
    }

    pub fn col(&self, x: usize) -> Vec<&T> {
        self.data.iter().map(|row| &row[x]).collect()
    }

    /// returns an iterator of the points in the grid at the specified x coordinate
    pub fn y_points_at(&self, x: usize) -> impl Iterator<Item = Point> {
        (0..self.height()).map(move |y| Point::new(x, y))
    }

    /// returns an iterator of the points in the grid at the specified y coordinate
    pub fn x_points_at(&self, y: usize) -> impl Iterator<Item = Point> {
        (0..self.width()).map(move |x| Point::new(x, y))
    }

    /// returns the count of the number of items in the grid that match the predicate
    pub fn count(&self, f: impl Fn(&T, &Point) -> bool) -> usize {
        self.flat_iter().filter(|(t, p)| f(t, p)).count()
    }

    pub fn get_wrapping(&self, p: Point) -> &T {
        let Point { x, y } = p;
        let width = self.width();
        let height = self.height();
        &self.data[y.rem_euclid(height)][x.rem_euclid(width)]
    }

    pub fn get<P: Num + Clone + Copy + NumCast>(&self, p: GenericPoint<P>) -> Option<&T> {
        let point = p.as_point()?;
        if self.contains_point(point) {
            Some(&(self[point]))
        } else {
            None
        }
    }

    pub fn fill(&mut self, with: T)
    where
        T: Copy,
    {
        for v in self.data.iter_mut() {
            v.fill(with)
        }
    }

    pub fn swap(&mut self, p1: Point, p2: Point)
    where
        T: Clone,
    {
        // TODO: get this to work with std::mem::swap somehow
        let (d1, d2) = (self[p1].clone(), self[p2].clone());
        self[p2] = d1;
        self[p1] = d2;
    }

    pub fn find(&self, value: T) -> impl Iterator<Item = Point> + '_
    where
        T: Eq,
    {
        self.flat_iter()
            .filter_map(move |(v, p)| (*v == value).then_some(p))
    }
}

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl<T: Clone + Default, const W: usize> Grid<T, W> {
    /// swaps from Row-Major to Column-Major order
    /// and vice-versa
    pub fn swap_order(&mut self) {
        let rows = self.data.len();
        let cols = self.data[0].len();

        // Create a new grid with reversed order
        let mut new_data = self.empty_sized();

        for i in 0..rows {
            for j in 0..cols {
                std::mem::swap(&mut new_data.data[j][i], &mut self.data[i][j]);
            }
        }

        // Update the grid with the new order
        self.data = new_data.data;
    }

    pub fn insert_empty_row(&mut self, index: usize) {
        self.data
            .insert(index, smallvec![Default::default(); self.width()]);
    }

    pub fn insert_empty_col(&mut self, index: usize) {
        for row in self.data.iter_mut() {
            row.insert(index, Default::default());
        }
    }

    pub fn print<K: Display>(&self, f: impl Fn(&T) -> K) {
        for row in self.data.iter() {
            for col in row.iter() {
                print!("{}", f(col));
            }
            println!();
        }
    }
}

impl<T, const W: usize> Grid<T, W> {
    /// creates an empty grid with the same dimensions as self
    pub fn empty_sized<K: Clone + Default>(&self) -> Grid<K, W> {
        let data = vec![smallvec![Default::default(); self.width()]; self.height()];

        Grid { data }
    }

    pub fn empty(width: usize, height: usize) -> Self
    where
        T: Clone + Default,
    {
        let data = vec![smallvec![Default::default(); width]; height];

        Grid { data }
    }

    /// maps over self to create a new Grid
    pub fn map<K: Clone>(&self, f: impl Fn(&T, Point) -> K) -> Grid<K, W> {
        let data = self
            .data
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, col)| f(col, Point::new(x, y)))
                    .collect()
            })
            .collect();
        Grid { data }
    }
}

impl<T, const W: usize> Index<Point> for Grid<T, W> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self.data[index.y][index.x]
    }
}

impl<T, const W: usize> IndexMut<Point> for Grid<T, W> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.data[index.y][index.x]
    }
}

impl<T, const W: usize> Display for Grid<T, W>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.data.iter() {
            for c in row.iter() {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T, const W: usize> From<HashMap<Point, T>> for Grid<T, W>
where
    T: Default + Clone,
{
    fn from(value: HashMap<Point, T>) -> Self {
        let min_x = value.keys().map(|p| p.x).min().unwrap_or(0);
        let min_y = value.keys().map(|p| p.y).min().unwrap_or(0);
        let max_x = value.keys().map(|p| p.x).max().unwrap_or(0);
        let max_y = value.keys().map(|p| p.y).max().unwrap_or(0);

        let min = Point::new(min_x, min_y);

        let mut data = Grid::empty(max_x - min_x + 1, max_y - min_y + 1);
        for (k, v) in value.into_iter() {
            data[k - min] = v;
        }

        data
    }
}

impl<const W: usize> From<HashSet<Point>> for Grid<bool, W> {
    fn from(value: HashSet<Point>) -> Self {
        let min_x = value.iter().map(|p| p.x).min().unwrap_or(0);
        let min_y = value.iter().map(|p| p.y).min().unwrap_or(0);
        let max_x = value.iter().map(|p| p.x).max().unwrap_or(0);
        let max_y = value.iter().map(|p| p.y).max().unwrap_or(0);

        let min = Point::new(min_x, min_y);

        let mut data: Grid<bool, W> = Grid::empty(max_x - min_x + 1, max_y - min_y + 1);
        for p in value.into_iter() {
            data[p - min] = true;
        }

        data
    }
}
