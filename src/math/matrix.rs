use num_traits::{ConstZero, Signed};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{AddAssign, Index, IndexMut, Neg};

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Matrix<const W: usize, const H: usize, T>(pub [[T; W]; H]);

impl<const W: usize, const S: usize, T> Debug for Matrix<W, S, T>
where
    Self: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        Display::fmt(self, f)
    }
}

impl<const W: usize, const H: usize, T> Display for Matrix<W, H, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..H {
            for x in 0..W {
                write!(f, "{:4}", self[(x, y)])?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl<const W: usize, const H: usize, T> Matrix<W, H, T> {
    #[inline]
    pub fn map<F: Fn(T) -> T2, T2>(self, f: F) -> Matrix<W, H, T2> {
        Matrix::<W, H, T2>(self.0.map(|a| a.map(|v| f(v))))
    }

    #[inline]
    pub fn rows(&self) -> impl Iterator<Item = &[T; W]> {
        self.0.iter()
    }
}

impl<const W: usize, const H: usize, T> Matrix<W, H, T>
where
    T: Neg<Output = T> + Copy + PartialOrd + AddAssign + ConstZero + Signed,
{
    pub fn equations<const V: usize>(&self) -> [Option<(T, [T; V])>; V] {
        assert_eq!(V, W - 1);

        let mut res = [None; V];
        let mut pivot_y = 0;
        for x in 0..(W - 1) {
            if self[(x, pivot_y)] != T::ZERO {
                let mut arr = [T::ZERO; V];
                let has_other = (0..H)
                    .find(|y| *y != pivot_y && self[(x, *y)] != T::ZERO)
                    .is_some();
                if has_other {
                    continue;
                }

                for x2 in 0..(W - 1) {
                    if x2 == x {
                        continue;
                    }

                    arr[x2] = -self[(x2, pivot_y)];
                }

                res[x] = Some((self[(W - 1, pivot_y)], arr));
                pivot_y += 1;
                if pivot_y == H {
                    break;
                }
            }
        }

        res
    }

    pub fn gauss_jordan_elimination<ZF: Fn(T) -> bool>(self, zf: ZF) -> Self {
        // Thanks for the pseudocode, Google AI (– and Wikipedia as well)

        let mut res = self;
        let mut sc = 0;
        let mut sr = 0;

        while sc < W && sr < H {
            let max_row = (sr..H).fold(sr, |by, y| {
                if res[(sc, y)].abs() < res[(sc, by)].abs() {
                    by
                } else {
                    y
                }
            });
            if !zf(res[(sc, max_row)]) {
                res = res.swap_rows(sr, max_row);

                // Normalize pivot row
                let factor = res[(sc, sr)];
                res[sr] = res[sr].map(|v| v / factor);

                // Eliminate other entries in pivot column
                for y in 0..H {
                    if y == sr {
                        continue;
                    }

                    let f = res[(sc, y)];
                    res = res.add_row(y, sr, -f);
                }

                sr += 1;
            }

            sc += 1;
        }

        res
    }

    pub fn add_row(self, dr: usize, sr: usize, fac: T) -> Self {
        if fac == T::ZERO {
            self
        } else {
            let mut copy = self;
            for i in 0..W {
                copy[dr][i] += self[sr][i] * fac;
            }
            copy
        }
    }

    pub fn swap_rows(self, y1: usize, y2: usize) -> Self {
        if y1 == y2 {
            self
        } else {
            let mut copy = self;
            copy[y1] = self[y2];
            copy[y2] = self[y1];
            copy
        }
    }
}

impl<const W: usize, const H: usize, T> Index<(usize, usize)> for Matrix<W, H, T> {
    type Output = T;

    #[inline]
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.0[y][x]
    }
}

impl<const W: usize, const H: usize, T> IndexMut<(usize, usize)> for Matrix<W, H, T> {
    #[inline]
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.0[y][x]
    }
}

impl<const W: usize, const H: usize, T> Index<usize> for Matrix<W, H, T> {
    type Output = [T; W];

    #[inline]
    fn index(&self, y: usize) -> &Self::Output {
        &self.0[y]
    }
}

impl<const W: usize, const H: usize, T> IndexMut<usize> for Matrix<W, H, T> {
    #[inline]
    fn index_mut(&mut self, y: usize) -> &mut Self::Output {
        &mut self.0[y]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_index() {
        let m = Matrix::<5, 4, i16>([
            [1, 2, 3, 4, 5],
            [101, 102, 103, 104, 105],
            [201, 202, 203, 204, 205],
            [301, 302, 303, 304, 305],
        ]);

        assert_eq!(m[(1, 2)], 202);
        assert_eq!(m[(4, 0)], 5);
        assert_eq!(m[(2, 2)], 203);
        assert_eq!(m[(3, 0)], 4);
        assert_eq!(m[(3, 1)], 104);
        assert_eq!(m[(3, 2)], 204);
        assert_eq!(m[(3, 3)], 304);
    }

    #[test]
    fn matrix_gaussian_elimination() {
        let m = Matrix::<7, 4, i16>([
            [0, 0, 0, 0, 1, 1, 3],
            [0, 1, 0, 0, 0, 1, 5],
            [0, 0, 1, 1, 1, 0, 4],
            [1, 1, 0, 1, 0, 0, 7],
        ])
        .map(|v| v as f32);

        println!("input: {m:?}");

        assert_eq!(
            m.gauss_jordan_elimination(|v| v == 0.0)
                .map(|v| v.round() as i16),
            Matrix::<7, 4, i16>([
                [1, 0, 0, 1, 0, -1, 2],
                [0, 1, 0, 0, 0, 1, 5],
                [0, 0, 1, 1, 0, -1, 1],
                [0, 0, 0, 0, 1, 1, 3],
            ])
        );
    }

    #[test]
    fn matrix_swap_rows() {
        let m = Matrix::<8, 4, i16>([
            [0, 0, 0, 0, 0, 1, 1, 3],
            [0, 1, 0, 0, 0, 0, 1, 5],
            [0, 0, 0, 1, 1, 1, 0, 4],
            [1, 1, 0, 1, 0, 0, 0, 7],
        ]);

        assert_eq!(
            m.swap_rows(0, 3),
            Matrix::<8, 4, i16>([
                [1, 1, 0, 1, 0, 0, 0, 7],
                [0, 1, 0, 0, 0, 0, 1, 5],
                [0, 0, 0, 1, 1, 1, 0, 4],
                [0, 0, 0, 0, 0, 1, 1, 3],
            ])
        );
    }

    #[test]
    fn matrix_add_rows() {
        let m = Matrix::<8, 4, i16>([
            [0, 0, 0, 0, 0, 1, 1, 3],
            [0, 1, 0, 0, 0, 0, 1, 5],
            [0, 0, 0, 1, 1, 1, 0, 4],
            [1, 1, 0, 1, 0, 0, 0, 7],
        ]);

        assert_eq!(
            m.add_row(0, 2, 15),
            Matrix::<8, 4, i16>([
                [0, 0, 0, 15, 15, 16, 1, 63],
                [0, 1, 0, 0, 0, 0, 1, 5],
                [0, 0, 0, 1, 1, 1, 0, 4],
                [1, 1, 0, 1, 0, 0, 0, 7],
            ])
        );
    }

    #[test]
    fn matrix_equations() {
        let m = Matrix::<7, 4, i16>([
            [1, 0, 0, 1, 0, -1, 2],
            [0, 1, 0, 0, 0, 1, 5],
            [0, 0, 1, 1, 0, -1, 1],
            [0, 0, 0, 0, 1, 1, 3],
        ]);

        println!("input: {m:?}");

        assert_eq!(
            m.equations::<6>(),
            [
                Some((2, [0, 0, 0, -1, 0, 1])),
                Some((5, [0, 0, 0, 0, 0, -1])),
                Some((1, [0, 0, 0, -1, 0, 1])),
                None,
                Some((3, [0, 0, 0, 0, 0, -1])),
                None,
            ]
        );
    }
}
