use num_traits::{ConstOne, ConstZero, WrappingAdd, WrappingSub};
use std::cmp::minmax;
use std::ops::{Add, AddAssign, Sub};

pub trait ManhattanDistance {
    type Output;

    fn manhattan_distance(&self, other: &Self) -> Self::Output;
}

impl<N> ManhattanDistance for (N, N)
where
    N: Add<N, Output = N> + Sub<N, Output = N> + Ord + Copy,
{
    type Output = N;

    fn manhattan_distance(&self, other: &Self) -> Self::Output {
        let [x1, x2] = minmax(self.0, other.0);
        let [y1, y2] = minmax(self.1, other.1);
        (x2 - x1) + (y2 - y1)
    }
}

pub trait Directions2D
where
    Self: Sized,
{
    fn up(&self) -> Self;
    fn left(&self) -> Self;
    fn right(&self) -> Self;
    fn down(&self) -> Self;

    #[inline]
    fn up_left(&self) -> Self {
        self.up().left()
    }
    #[inline]
    fn up_right(&self) -> Self {
        self.up().right()
    }
    #[inline]
    fn down_left(&self) -> Self {
        self.down().left()
    }
    #[inline]
    fn down_right(&self) -> Self {
        self.down().right()
    }

    #[inline]
    fn eight_neighbors(&self) -> [Self; 8] {
        [
            self.up_left(),
            self.up(),
            self.up_right(),
            self.left(),
            self.right(),
            self.down_left(),
            self.down(),
            self.down_right(),
        ]
    }

    #[inline]
    fn next_in_direction(&self, dir: Direction) -> Self {
        match dir {
            Direction::UpLeft => self.up_left(),
            Direction::Up => self.up(),
            Direction::UpRight => self.up_right(),
            Direction::Left => self.left(),
            Direction::Right => self.right(),
            Direction::DownLeft => self.down_left(),
            Direction::Down => self.down(),
            Direction::DownRight => self.down_right(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Direction {
    UpLeft,
    Up,
    UpRight,
    Left,
    Right,
    DownLeft,
    Down,
    DownRight,
}

pub const EIGHT_DIRECTIONS: [Direction; 8] = [
    Direction::UpLeft,
    Direction::Up,
    Direction::UpRight,
    Direction::Left,
    Direction::Right,
    Direction::DownLeft,
    Direction::Down,
    Direction::DownRight,
];

pub const CARDINAL_DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Left,
    Direction::Right,
    Direction::Down,
];

pub const DIAGONAL_DIRECTIONS: [Direction; 4] = [
    Direction::UpLeft,
    Direction::UpRight,
    Direction::DownLeft,
    Direction::DownRight,
];

impl<N> Directions2D for (N, N)
where
    N: Copy + ConstOne + WrappingSub<Output = N> + WrappingAdd<Output = N>,
{
    #[inline]
    fn up(&self) -> Self {
        (self.0, self.1.wrapping_sub(&N::ONE))
    }

    #[inline]
    fn left(&self) -> Self {
        (self.0.wrapping_sub(&N::ONE), self.1)
    }

    #[inline]
    fn right(&self) -> Self {
        (self.0.wrapping_add(&N::ONE), self.1)
    }

    #[inline]
    fn down(&self) -> Self {
        (self.0, self.1.wrapping_add(&N::ONE))
    }

    #[inline]
    fn up_left(&self) -> Self {
        (self.0.wrapping_sub(&N::ONE), self.1.wrapping_sub(&N::ONE))
    }

    #[inline]
    fn up_right(&self) -> Self {
        (self.0.wrapping_add(&N::ONE), self.1.wrapping_sub(&N::ONE))
    }

    #[inline]
    fn down_left(&self) -> Self {
        (self.0.wrapping_sub(&N::ONE), self.1.wrapping_add(&N::ONE))
    }

    #[inline]
    fn down_right(&self) -> Self {
        (self.0.wrapping_add(&N::ONE), self.1.wrapping_add(&N::ONE))
    }
}

pub fn compress_coordinates<T>(list: &mut [(T, T)])
where
    T: Clone + Copy + Ord + Eq + ConstZero + ConstOne + AddAssign,
    (T, T): Copy,
{
    let mut prev_value = T::ZERO;
    let mut compressed_value = T::ZERO;
    let mut sorted = (0..list.len()).collect::<Vec<_>>();
    sorted.sort_unstable_by_key(|i| list[*i].0);
    for i in sorted.iter() {
        let curr = &mut list[*i as usize];
        if curr.0 != prev_value {
            compressed_value += T::ONE;
        }
        prev_value = curr.0;
        curr.0 = compressed_value;
    }
    sorted.sort_unstable_by_key(|i| list[*i].1);
    compressed_value = T::ZERO;
    prev_value = T::ZERO;
    for i in sorted.iter() {
        let curr = &mut list[*i];
        if curr.1 != prev_value {
            compressed_value += T::ONE;
        }
        prev_value = curr.1;
        curr.1 = compressed_value;
    }
}

#[cfg(test)]
mod tests {
    use crate::geo::compress_coordinates;

    #[test]
    fn compress_coordinates_works() {
        fn run(input: &[(u32, u32)]) -> Vec<(u32, u32)> {
            let mut res = input.to_vec();
            compress_coordinates(&mut res);
            res
        }

        assert_eq!(
            run(&[(15, 75), (30, 15), (50, 25)]),
            vec![(1, 3), (2, 1), (3, 2)]
        )
    }
}
