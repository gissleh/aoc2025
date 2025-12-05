use common::aoc::Runner;
use common::grid::GridCoordinate;
use common::parser;
use common::parser::Parser;
use std::fmt::{Debug, Formatter};

pub fn main(r: &mut Runner, input: &[u8]) {
    let (presents, region_specs) = r.prep("Parse", || parse(input));

    r.part("Part 1 (Dirty)", || part_1_dirty(&presents, &region_specs));

    r.info("Presents", presents.len());
    r.info_debug(
        "Present Variations",
        presents.iter().map(|p| p.0.len()).collect::<Vec<_>>(),
    );
    r.info("Regions", region_specs.len());
}

fn part_1_dirty(presents: &[PresentSet], specs: &[RegionSpec]) -> usize {
    let mut areas = [0; 6];
    for i in 0..6 {
        areas[i] = presents[i].0[0].area();
    }

    specs
        .iter()
        .filter(|s| {
            s.presents
                .iter()
                .zip(areas.iter())
                .map(|(p, a)| (*p as u32) * a)
                .sum::<u32>()
                < (s.size.0 - (s.size.0 % 3), s.size.1 - (s.size.1 % 3)).area() as u32
        })
        .count()
}

fn parse(input: &[u8]) -> (Vec<PresentSet>, Vec<RegionSpec>) {
    PresentSet::parser()
        .map(|(_, ps)| ps)
        .delimited_by(b"\n\n")
        .repeat()
        .and_skip(b"\n\n")
        .and(RegionSpec::parser().delimited_by(b'\n').repeat())
        .run(input)
        .unwrap()
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct Present([u8; 3]);

impl Present {
    #[inline]
    fn flip_h(self) -> Self {
        Self([
            (self.0[0] & 0b010) | ((self.0[0] & 0b100) >> 2) | ((self.0[0] & 0b001) << 2),
            (self.0[1] & 0b010) | ((self.0[1] & 0b100) >> 2) | ((self.0[1] & 0b001) << 2),
            (self.0[2] & 0b010) | ((self.0[2] & 0b100) >> 2) | ((self.0[2] & 0b001) << 2),
        ])
    }

    #[inline]
    fn flip_v(self) -> Self {
        Self([self.0[2], self.0[1], self.0[0]])
    }

    #[inline]
    fn area(&self) -> u32 {
        self.0[0].count_ones() + self.0[1].count_ones() + self.0[2].count_ones()
    }

    fn rotate_right(self) -> Self {
        let mut rotated = [0u8; 3];

        for i in 0..3 {
            let bit_i = 1 << i;
            for j in 0..3 {
                if self.0[i] & 1 << j != 0 {
                    rotated[2 - j] |= bit_i;
                }
            }
        }

        Self(rotated)
    }

    #[inline]
    fn parser<'i>() -> impl Parser<'i, (usize, Present)> {
        parser::uint().and_skip(b":\n").and(
            b'#'.or(b'.')
                .repeat_fold(
                    || 0u8,
                    |acc, curr| {
                        *acc <<= 1;
                        if curr == b'#' {
                            *acc |= 1;
                        }
                        true
                    },
                )
                .delimited_by(b'\n')
                .repeat::<[u8; 3]>()
                .map(|v| Present(v)),
        )
    }
}

impl Debug for Present {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for y in 0..3 {
            for x in 0..3 {
                if self.0[y] & 1 << x != 0 {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Eq, PartialEq, Debug)]
struct PresentSet(Vec<Present>);

impl PresentSet {
    #[allow(dead_code)]
    fn build(present: Present) -> Self {
        let mut vec = Vec::with_capacity(4 * 2 * 2);
        vec.push(present);

        let rotated = present.rotate_right();
        let rotated2 = rotated.rotate_right();
        let rotated3 = rotated2.rotate_right();
        for curr in [present, rotated, rotated2, rotated3] {
            for curr in [curr, curr.flip_h(), curr.flip_v()] {
                if !vec.contains(&curr) {
                    vec.push(curr);
                }
            }
        }

        Self(vec)
    }

    #[inline]
    #[allow(dead_code)]
    fn parser<'i>() -> impl Parser<'i, (usize, Self)> {
        Present::parser().map(|(index, present)| (index, PresentSet::build(present)))
    }
}

struct RegionSpec {
    size: (usize, usize),
    presents: [u8; 6],
}

impl RegionSpec {
    #[inline]
    pub fn parser<'i>() -> impl Parser<'i, RegionSpec> {
        parser::uint()
            .and_skip(b'x')
            .and(parser::uint())
            .and_skip(b": ")
            .and(parser::uint().delimited_by(b' ').repeat())
            .map(|((w, h), presents)| RegionSpec {
                size: (w, h),
                presents,
            })
    }

    pub fn try_dfs(&self, set: &[PresentSet]) -> Option<Space> {
        let mut stack = Vec::with_capacity(512);
        stack.push((Space::new(self.size), self.presents));

        while let Some((space, remaining)) = stack.pop() {
            let mut any_remaining = false;
            for i in 0..6 {
                if remaining[i] > 0 {
                    any_remaining = true;
                }
            }

            if !any_remaining {
                return Some(space);
            }
        }

        None
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
struct Space {
    data: [u64; 50],
    size: (usize, usize),
}

impl Space {
    #[inline]
    #[allow(dead_code)]
    pub fn new(size: (usize, usize)) -> Self {
        Self {
            data: [0; 50],
            size,
        }
    }

    #[allow(dead_code)]
    fn put_present(&self, x: usize, y: usize, present: Present) -> Option<Space> {
        if x > self.size.0 - 3 || y > self.size.1 - 3 {
            return None;
        }

        let mut masks = [0u64; 3];
        for i in 0..3 {
            let mask = (present.0[i] as u64) << x;
            if self.data[y + i] & mask != 0 {
                return None;
            }
            masks[i] = mask;
        }

        let mut copy = *self;
        for i in 0..3 {
            copy.data[y + i] |= masks[i]
        }
        Some(copy)
    }
}

impl Debug for Space {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self.size)?;
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                if self.data[y] & 1 << x != 0 {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn present_flips_h() {
        assert_eq!(
            Present([0b100, 0b010, 0b001]).flip_h(),
            Present([0b001, 0b010, 0b100])
        );
        assert_eq!(
            Present([0b001, 0b010, 0b100]).flip_h(),
            Present([0b100, 0b010, 0b001])
        );
        assert_eq!(
            Present([0b110, 0b011, 0b110]).flip_h(),
            Present([0b011, 0b110, 0b011])
        );
        assert_eq!(
            Present([0b101, 0b010, 0b101]).flip_h(),
            Present([0b101, 0b010, 0b101])
        );
    }

    #[test]
    fn present_flips_v() {
        assert_eq!(
            Present([0b100, 0b010, 0b001]).flip_v(),
            Present([0b001, 0b010, 0b100])
        );
        assert_eq!(
            Present([0b001, 0b010, 0b100]).flip_v(),
            Present([0b100, 0b010, 0b001])
        );
        assert_eq!(
            Present([0b110, 0b101, 0b011]).flip_v(),
            Present([0b011, 0b101, 0b110])
        );
        assert_eq!(
            Present([0b101, 0b010, 0b101]).flip_v(),
            Present([0b101, 0b010, 0b101])
        );
    }

    #[test]
    fn present_rotate_right() {
        assert_eq!(
            Present([0b100, 0b010, 0b001]).rotate_right(),
            Present([0b001, 0b010, 0b100])
        );
        assert_eq!(
            Present([0b001, 0b010, 0b100]).rotate_right(),
            Present([0b100, 0b010, 0b001])
        );
        assert_eq!(
            Present([0b110, 0b101, 0b011]).rotate_right(),
            Present([0b011, 0b101, 0b110])
        );
    }

    #[test]
    fn present_set() {
        assert_eq!(
            PresentSet::build(Present([0b100, 0b000, 0b000])),
            PresentSet(vec![
                Present([0b100, 0b000, 0b000]),
                Present([0b001, 0b000, 0b000]),
                Present([0b000, 0b000, 0b100]),
                Present([0b000, 0b000, 0b001]),
            ])
        );

        assert_eq!(
            PresentSet::build(Present([0b000, 0b010, 0b000])),
            PresentSet(vec![Present([0b000, 0b010, 0b000]),])
        );

        assert_eq!(
            PresentSet::build(Present([0b101, 0b111, 0b101])),
            PresentSet(vec![
                Present([0b101, 0b111, 0b101]),
                Present([0b111, 0b010, 0b111]),
            ])
        );
    }

    #[test]
    fn yem_stxelit_ngipsìn() {
        let space = Space::new((8, 8));
        let space2 = Space {
            data: [
                0b011, 0b001, 0b011, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            size: (8, 8),
        };
        let space3 = Space {
            data: [
                0b111, 0b111, 0b111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            size: (8, 8),
        };

        assert_eq!(
            space.put_present(0, 0, Present([0b011, 0b001, 0b011])),
            Some(space2)
        );
        assert_eq!(
            space2.put_present(0, 0, Present([0b100, 0b110, 0b100])),
            Some(space3)
        );
        assert_eq!(
            space3.put_present(0, 0, Present([0b000, 0b010, 0b000])),
            None
        );
        assert_eq!(
            space.put_present(6, 0, Present([0b000, 0b000, 0b000])),
            None
        );
        assert_eq!(
            space.put_present(0, 6, Present([0b000, 0b000, 0b000])),
            None
        );
    }
}
