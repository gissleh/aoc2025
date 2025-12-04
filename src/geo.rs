use num_traits::{ConstOne, WrappingAdd, WrappingSub};
use std::cmp::minmax;
use std::ops::{Add, Sub};

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

    fn up_left(&self) -> Self {
        self.up().left()
    }
    fn up_right(&self) -> Self {
        self.up().right()
    }
    fn down_left(&self) -> Self {
        self.down().left()
    }
    fn down_right(&self) -> Self {
        self.down().right()
    }

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
}

impl<N> Directions2D for (N, N)
where
    N: Copy + ConstOne + WrappingSub<Output = N> + WrappingAdd<Output = N>,
{
    fn up(&self) -> Self {
        (self.0, self.1.wrapping_sub(&N::ONE))
    }

    fn left(&self) -> Self {
        (self.0.wrapping_sub(&N::ONE), self.1)
    }

    fn right(&self) -> Self {
        (self.0.wrapping_add(&N::ONE), self.1)
    }

    fn down(&self) -> Self {
        (self.0, self.1.wrapping_add(&N::ONE))
    }

    fn up_left(&self) -> Self {
        (self.0.wrapping_sub(&N::ONE), self.1.wrapping_sub(&N::ONE))
    }

    fn up_right(&self) -> Self {
        (self.0.wrapping_add(&N::ONE), self.1.wrapping_sub(&N::ONE))
    }

    fn down_left(&self) -> Self {
        (self.0.wrapping_sub(&N::ONE), self.1.wrapping_add(&N::ONE))
    }

    fn down_right(&self) -> Self {
        (self.0.wrapping_add(&N::ONE), self.1.wrapping_add(&N::ONE))
    }
}
