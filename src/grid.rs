use std::hash::{Hash, Hasher};
use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Grid<C, T> {
    data: Vec<T>,
    size: C,
    width: usize,
}

impl<C, T> PartialEq<Self> for Grid<C, T>
where
    T: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data)
    }
}

impl<C, T> Eq for Grid<C, T> where T: Eq {}

impl<C, T> Hash for Grid<C, T>
where
    Vec<T>: Hash + Eq,
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state)
    }
}

impl<C, T> Grid<C, T>
where
    C: GridCoordinate + GridCoordinate2D,
{
    #[inline]
    pub fn data(&self) -> &[T] {
        &self.data[..self.size.area()]
    }

    #[inline]
    pub fn size(&self) -> C {
        self.size
    }

    #[inline]
    pub fn new(data: Vec<T>, size: C) -> Self {
        #[cfg(debug_assertions)]
        assert!(
            data.len() >= size.area(),
            "data too short ({} < {})",
            data.len(),
            size.area()
        );
        Self {
            data,
            size,
            width: size.x(),
        }
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (C, &T)> {
        self.data.iter().enumerate().map(move |(i, cell)| {
            let y = i / self.width;
            let x = i % self.width;

            (C::from_usize(x, y), cell)
        })
    }

    #[allow(dead_code)]
    pub fn print<F>(&self, format_fn: F)
    where
        F: Fn(&T) -> char,
    {
        for (p, v) in self.iter() {
            if p.x() == 0 && p.y() > 0 {
                println!();
            }
            print!("{}", format_fn(v));
        }
        println!();
    }
}

impl<C, T> Grid<C, T>
where
    C: GridCoordinate + GridCoordinate2D,
    T: Copy,
{
    pub fn parse<F>(input: &[u8], padding: usize, padding_item: T, f: F) -> Option<Self>
    where
        F: Fn(&u8) -> T,
    {
        let width = input.iter().position(|ch| *ch == b'\n')?;
        let height = input.len() / (width + 1);
        let padded_width = width + padding + padding;
        let padded_height = height + padding + padding;
        let top_padding = vec![padding_item; padded_width * padding];
        let mut res = Vec::with_capacity(padded_width * padded_height);

        res.extend_from_slice(&top_padding);

        for chunk in input.chunks(width + 1) {
            for _ in 0..padding {
                res.push(padding_item);
            }
            for ch in &chunk[..chunk.len() - 1] {
                res.push(f(ch));
            }
            for _ in 0..padding {
                res.push(padding_item);
            }
        }

        res.extend_from_slice(&top_padding);

        Some(Self::new(res, C::from_usize(padded_width, padded_height)))
    }
}

impl<C, T> Grid<C, T>
where
    T: Copy,
{
    #[inline]
    pub fn fill(&mut self, t: T) {
        self.data.fill(t);
    }
}

impl<C, T> Grid<C, T>
where
    T: Copy + Default,
{
    #[inline]
    pub fn clear(&mut self) {
        self.data.fill(T::default());
    }
}

impl<C, T> Grid<C, T>
where
    C: GridCoordinate + GridCoordinate2D,
    T: Copy,
{
    pub fn new_with_value(size: C, value: T) -> Self {
        Self {
            data: vec![value; size.area()],
            size,
            width: size.x(),
        }
    }
}

impl<C, T> Grid<C, T>
where
    C: GridCoordinate + GridCoordinate2D,
    T: Copy + Default,
{
    pub fn new_blank(size: C) -> Self {
        Self {
            data: vec![T::default(); size.area()],
            size,
            width: size.x(),
        }
    }
}

impl<C, T> Grid<C, T>
where
    C: GridCoordinate2D,
{
    #[inline]
    pub fn row(&self, y: usize) -> &[T] {
        let offset = y * self.width;
        &self.data[offset..offset + self.width]
    }

    #[inline]
    pub fn row_mut(&mut self, y: usize) -> &mut [T] {
        let offset = y * self.width;
        &mut self.data[offset..offset + self.width]
    }
}

impl<C, T> Index<C> for Grid<C, T>
where
    C: GridCoordinate,
{
    type Output = T;

    #[inline]
    fn index(&self, index: C) -> &Self::Output {
        self.data.index(index.index(&self.size))
    }
}

impl<C, T> IndexMut<C> for Grid<C, T>
where
    C: GridCoordinate,
{
    #[inline]
    fn index_mut(&mut self, index: C) -> &mut Self::Output {
        self.data.index_mut(index.index(&self.size))
    }
}

#[derive(Clone, Copy)]
pub struct RawDelimitedGrid<'r, C, T> {
    data: &'r [T],
    size: C,
    raw_size: C,
}

impl<'r, C, T> RawDelimitedGrid<'r, C, T>
where
    C: GridCoordinate + GridCoordinate2D,
    T: Eq,
{
    #[inline]
    pub fn data(&self) -> &[T] {
        &self.data[..self.raw_size.area()]
    }

    #[inline]
    pub fn size(&self) -> C {
        self.size
    }

    #[inline]
    pub fn new(data: &'r [T], delim: T) -> Self {
        let width = data.iter().position(|c| *c == delim).unwrap() + 1;
        let height = data.len() / width;
        assert_eq!(data.len() % width, 0);

        Self {
            data,
            size: C::from_usize(width - 1, height),
            raw_size: C::from_usize(width, height),
        }
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (C, &T)> {
        let width = self.size.x();
        self.data
            .chunks(self.raw_size.x())
            .flat_map(|r| r[..r.len() - 1].iter())
            .enumerate()
            .map(move |(i, cell)| {
                let y = i / width;
                let x = i % width;

                (C::from_usize(x, y), cell)
            })
    }

    #[inline]
    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        self.data
            .chunks(self.raw_size.x())
            .map(|r| &r[..r.len() - 1])
    }
}

impl<'r, C, T> RawDelimitedGrid<'r, C, T>
where
    C: GridCoordinate + GridCoordinate2D,
    T: Eq + Copy,
{
    #[inline]
    pub fn to_editable(&self) -> Grid<C, T> {
        let mut res = Vec::with_capacity(self.size.area());
        let row_width = self.size.x();
        for i in 0..self.size.y() {
            let offset = self.raw_size.x() * i;
            res.extend(self.data[offset..offset + row_width].iter().copied());
        }

        Grid::new(res, self.size)
    }
}

impl<'i, C, T> Index<C> for RawDelimitedGrid<'i, C, T>
where
    C: GridCoordinate,
{
    type Output = T;

    #[inline]
    fn index(&self, index: C) -> &Self::Output {
        self.data.index(index.index(&self.raw_size))
    }
}

pub trait GridCoordinate2D: Default + Copy {
    fn x(&self) -> usize;
    fn y(&self) -> usize;
    fn from_usize(x: usize, y: usize) -> Self;
}

pub trait GridCoordinate: Default + Copy {
    fn zero() -> Self;
    fn area(&self) -> usize;
    fn in_bounds(&self, size: &Self) -> bool;
    fn index(&self, size: &Self) -> usize;
    fn next(&self, size: &Self) -> Self;

    fn iter(&self) -> impl Iterator<Item = Self> {
        GridCoordinateIter::new(*self)
    }
}

macro_rules! impl_coord {
    ($int: tt) => {
        impl GridCoordinate for $int {
            #[inline]
            fn zero() -> Self {
                0
            }

            #[inline]
            fn area(&self) -> usize {
                *self as usize
            }

            #[inline]
            fn in_bounds(&self, size: &Self) -> bool {
                *self < *size
            }

            #[inline]
            fn index(&self, size: &Self) -> usize {
                (*self % *size) as usize
            }

            #[inline]
            fn next(&self, size: &Self) -> Self {
                (*self + 1) % *size
            }
        }

        impl GridCoordinate for ($int, $int) {
            #[inline]
            fn zero() -> Self {
                (0, 0)
            }

            #[inline]
            fn area(&self) -> usize {
                (self.0 as usize * self.1 as usize)
            }

            #[inline]
            fn in_bounds(&self, size: &Self) -> bool {
                self.0 < size.0 && self.1 < size.1
            }

            #[inline]
            fn index(&self, size: &Self) -> usize {
                ((self.1 as usize * size.0 as usize) + self.0 as usize)
            }

            #[inline]
            fn next(&self, size: &Self) -> Self {
                let nx = self.0 + 1;
                if nx == size.0 {
                    (0, self.1 + 1)
                } else {
                    (nx, self.1)
                }
            }
        }

        impl GridCoordinate2D for ($int, $int) {
            #[inline]
            fn x(&self) -> usize {
                self.0 as usize
            }

            #[inline]
            fn y(&self) -> usize {
                self.1 as usize
            }

            #[inline]
            fn from_usize(x: usize, y: usize) -> Self {
                (x as $int, y as $int)
            }
        }

        impl GridCoordinate for ($int, $int, $int) {
            #[inline]
            fn zero() -> Self {
                (0, 0, 0)
            }

            #[inline]
            fn area(&self) -> usize {
                (self.0 * self.1 * self.2) as usize
            }

            #[inline]
            fn in_bounds(&self, size: &Self) -> bool {
                self.0 < size.0 && self.1 < size.1 && self.2 < size.2
            }

            #[inline]
            fn index(&self, size: &Self) -> usize {
                ((self.2 as usize * (size.0 as usize * size.1 as usize))
                    + (self.1 as usize * size.0 as usize)
                    + self.0 as usize) as usize
            }

            #[inline]
            fn next(&self, size: &Self) -> Self {
                let nx = self.0 + 1;
                if nx == size.0 {
                    let ny = self.1 + 1;
                    if ny == size.1 {
                        (0, 0, self.2 + 1)
                    } else {
                        (0, ny, self.2)
                    }
                } else {
                    (nx, self.1, self.2)
                }
            }
        }

        impl GridCoordinate for ($int, $int, $int, $int) {
            #[inline]
            fn zero() -> Self {
                (0, 0, 0, 0)
            }

            #[inline]
            fn area(&self) -> usize {
                (self.0 * self.1 * self.2 * self.3) as usize
            }

            #[inline]
            fn in_bounds(&self, size: &Self) -> bool {
                self.0 < size.0 && self.1 < size.1 && self.2 < size.2 && self.3 < size.3
            }

            #[inline]
            fn index(&self, size: &Self) -> usize {
                let w = size.0 as usize;
                let wh = w * size.1 as usize;
                let whd = wh * size.2 as usize;

                ((self.3 as usize * whd)
                    + (self.2 as usize * wh)
                    + (self.1 as usize * size.0 as usize)
                    + self.0 as usize) as usize
            }

            #[inline]
            fn next(&self, size: &Self) -> Self {
                let nx = self.0 + 1;
                if nx == size.0 {
                    let ny = self.1 + 1;
                    if ny == size.1 {
                        let nz = self.2 + 1;
                        if nz == size.2 {
                            (0, 0, 0, self.3 + 1)
                        } else {
                            (0, 0, nz, self.3)
                        }
                    } else {
                        (0, ny, self.2, self.3)
                    }
                } else {
                    (nx, self.1, self.2, self.3)
                }
            }
        }
    };
}

macro_rules! impl_coord_signed {
    ($int: tt) => {
        impl GridCoordinate for $int {
            #[inline]
            fn zero() -> Self {
                0
            }

            #[inline]
            fn area(&self) -> usize {
                *self as usize
            }

            #[inline]
            fn in_bounds(&self, size: &Self) -> bool {
                *self >= 0 && *self < *size
            }

            #[inline]
            fn index(&self, size: &Self) -> usize {
                (*self % *size) as usize
            }

            #[inline]
            fn next(&self, size: &Self) -> Self {
                (*self + 1) % *size
            }
        }

        impl GridCoordinate for ($int, $int) {
            #[inline]
            fn zero() -> Self {
                (0, 0)
            }

            #[inline]
            fn area(&self) -> usize {
                (self.0 as usize * self.1 as usize)
            }

            #[inline]
            fn in_bounds(&self, size: &Self) -> bool {
                self.0 >= 0 && self.0 < size.0 && self.1 >= 0 && self.1 < size.1
            }

            #[inline]
            fn index(&self, size: &Self) -> usize {
                ((self.1 as usize * size.0 as usize) + self.0 as usize)
            }

            #[inline]
            fn next(&self, size: &Self) -> Self {
                let nx = self.0 + 1;
                if nx == size.0 {
                    (0, self.1 + 1)
                } else {
                    (nx, self.1)
                }
            }
        }

        impl GridCoordinate2D for ($int, $int) {
            #[inline]
            fn x(&self) -> usize {
                self.0 as usize
            }

            #[inline]
            fn y(&self) -> usize {
                self.1 as usize
            }

            #[inline]
            fn from_usize(x: usize, y: usize) -> Self {
                (x as $int, y as $int)
            }
        }

        impl GridCoordinate for ($int, $int, $int) {
            #[inline]
            fn zero() -> Self {
                (0, 0, 0)
            }

            #[inline]
            fn area(&self) -> usize {
                (self.0 * self.1 * self.2) as usize
            }

            #[inline]
            fn in_bounds(&self, size: &Self) -> bool {
                self.0 >= 0
                    && self.0 < size.0
                    && self.1 >= 0
                    && self.1 < size.1
                    && self.2 >= 0
                    && self.2 < size.2
            }

            #[inline]
            fn index(&self, size: &Self) -> usize {
                ((self.2 as usize * (size.0 as usize * size.1 as usize))
                    + (self.1 as usize * size.0 as usize)
                    + self.0 as usize) as usize
            }

            #[inline]
            fn next(&self, size: &Self) -> Self {
                let nx = self.0 + 1;
                if nx == size.0 {
                    let ny = self.1 + 1;
                    if ny == size.1 {
                        (0, 0, self.2 + 1)
                    } else {
                        (0, ny, self.2)
                    }
                } else {
                    (nx, self.1, self.2)
                }
            }
        }

        impl GridCoordinate for ($int, $int, $int, $int) {
            #[inline]
            fn zero() -> Self {
                (0, 0, 0, 0)
            }

            #[inline]
            fn area(&self) -> usize {
                (self.0 * self.1 * self.2 * self.3) as usize
            }

            #[inline]
            fn in_bounds(&self, size: &Self) -> bool {
                self.0 >= 0
                    && self.0 < size.0
                    && self.1 >= 0
                    && self.1 < size.1
                    && self.2 >= 0
                    && self.2 < size.2
                    && self.3 >= 0
                    && self.3 < size.3
            }

            #[inline]
            fn index(&self, size: &Self) -> usize {
                let w = size.0 as usize;
                let wh = w * size.1 as usize;
                let whd = wh * size.2 as usize;

                ((self.3 as usize * whd)
                    + (self.2 as usize * wh)
                    + (self.1 as usize * size.0 as usize)
                    + self.0 as usize) as usize
            }

            #[inline]
            fn next(&self, size: &Self) -> Self {
                let nx = self.0 + 1;
                if nx == size.0 {
                    let ny = self.1 + 1;
                    if ny == size.1 {
                        let nz = self.2 + 1;
                        if nz == size.2 {
                            (0, 0, 0, self.3 + 1)
                        } else {
                            (0, 0, nz, self.3)
                        }
                    } else {
                        (0, ny, self.2, self.3)
                    }
                } else {
                    (nx, self.1, self.2, self.3)
                }
            }
        }
    };
}

impl_coord!(u8);
impl_coord_signed!(i8);
impl_coord!(u16);
impl_coord_signed!(i16);
impl_coord!(u32);
impl_coord_signed!(i32);
impl_coord!(u64);
impl_coord_signed!(i64);
impl_coord!(u128);
impl_coord_signed!(i128);
impl_coord!(usize);
impl_coord_signed!(isize);

struct GridCoordinateIter<C: GridCoordinate> {
    curr: C,
    size: C,
    remaining: usize,
}

impl<C: GridCoordinate> GridCoordinateIter<C>
where
    C: GridCoordinate,
{
    fn new(size: C) -> Self {
        Self {
            size,
            curr: C::zero(),
            remaining: size.area(),
        }
    }
}

impl<C: GridCoordinate> Iterator for GridCoordinateIter<C>
where
    C: GridCoordinate,
{
    type Item = C;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining > 0 {
            let next = self.curr;
            self.curr = self.curr.next(&self.size);
            self.remaining -= 1;

            Some(next)
        } else {
            None
        }
    }
}
