use common::aoc::Runner;
use common::parser::{Parser, uint};
use std::cmp::max;
use std::fmt::{Debug, Display, Formatter};

pub fn main(r: &mut Runner, input: &[u8]) {
    let (ranges, ids) = r.prep("Parse", || parse(input));
    r.part("Part 1", || part_1(&ids, &ranges));
    r.part("Part 2", || part_2(&ranges));
    r.set_tail("Part 1");
    r.part("Part 2 (Alt)", || part_2_alt(&ranges));

    r.set_tail("Parse");
    let compact = r.prep("Compact", || compact_ranges(&ranges));
    r.part("Part 1 (Compact)", || part_1(&ids, &compact));
    r.part("Part 2 (Compact)", || part_2_compact(&compact));

    r.info("Ranges", ranges.len());
    r.info("Ranges (Compact)", compact.len());
    r.info("IDs", ids.len());
}

#[inline]
fn part_1(ids: &[u64], ranges: &[IDRange]) -> u32 {
    ids.iter()
        .filter(|id| {
            ranges
                .iter()
                .skip_while(|r| r.to < **id)
                .take_while(|r| r.from <= **id)
                .find(|r| **id <= r.to)
                .is_some()
        })
        .count() as u32
}

fn part_2(ranges: &[IDRange]) -> u64 {
    let mut unique_ranges: Vec<IDRange> = Vec::with_capacity(ranges.len() * 4);
    let mut buffer = ranges.iter().rev().copied().collect::<Vec<_>>();

    'main_loop: while let Some(mut range) = buffer.pop() {
        for urange in unique_ranges.iter_mut() {
            if let Some(right) = urange.make_room(&mut range) {
                buffer.push(right);
            }

            if range.from == 0 {
                continue 'main_loop;
            }
        }

        unique_ranges.push(range);
    }

    #[cfg(test)]
    for range in unique_ranges.iter() {
        println!("{range}");
    }

    unique_ranges.iter().map(|r| r.len()).sum()
}

fn part_2_alt(ranges: &[IDRange]) -> u64 {
    let mut total = 0;
    let mut current = ranges[0];

    for range in ranges.iter().skip(1) {
        if !current.try_merge(range) {
            total += current.len();
            current = *range;
        }
    }

    total + current.len()
}

fn part_2_compact(ranges: &[IDRange]) -> u64 {
    ranges.iter().map(|r| r.len()).sum()
}

fn compact_ranges(ranges: &[IDRange]) -> Vec<IDRange> {
    let mut current = ranges[0];
    let mut res = Vec::with_capacity(64);

    for range in ranges.iter().skip(1) {
        if !current.try_merge(range) {
            res.push(current);
            current = *range;
        }
    }

    res.push(current);
    res
}

fn parse(input: &[u8]) -> (Vec<IDRange>, Vec<u64>) {
    IDRange::parser()
        .delimited_by(b'\n')
        .repeat_fold(
            || Vec::with_capacity(128),
            |acc, next| {
                match acc.binary_search(&next) {
                    Ok(_) => {}
                    Err(idx) => acc.insert(idx, next),
                };

                true
            },
        )
        .and_skip(b"\n\n")
        .and(uint().delimited_by(b'\n').repeat())
        .run(input)
        .unwrap()
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct IDRange {
    from: u64,
    to: u64,
}

impl IDRange {
    #[inline]
    fn len(&self) -> u64 {
        (self.to - self.from) + 1
    }

    #[inline]
    fn try_merge(&mut self, other: &Self) -> bool {
        if other.from <= self.to {
            self.to = max(other.to, self.to);
            true
        } else {
            false
        }
    }

    fn make_room(&mut self, other: &mut IDRange) -> Option<IDRange> {
        if other.from > self.to || other.to < self.from {
            // Other is entirely outside
            None
        } else if other.from <= self.from && other.to >= self.to {
            // This is entirely covered
            let third = IDRange {
                from: self.to + 1,
                to: other.to,
            };
            other.to = self.from - 1;
            Some(third)
        } else if other.from >= self.from && other.to <= self.to {
            // Other is entirely covered
            other.from = 0;
            other.to = 0;
            None
        } else if other.from <= self.from && other.to >= self.from {
            // Other eats left
            other.to = self.from - 1;
            None
        } else if other.to >= self.to && other.from <= self.to {
            // Other eats right
            other.from = self.to + 1;
            None
        } else {
            panic!("{self} and {other} overlaps unexpectedly")
        }
    }

    #[inline]
    fn parser<'i>() -> impl Parser<'i, IDRange> {
        uint()
            .and_skip(b'-')
            .and(uint())
            .map(|(from, to)| IDRange { from, to })
    }
}

impl Display for IDRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.from, self.to)
    }
}

impl Debug for IDRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.from, self.to)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[test]
    fn part_2_works_on_example() {
        let (ranges, _) = parse(EXAMPLE);
        assert_eq!(part_2(&ranges), 14)
    }

    #[test]
    fn id_range_overlaps() {
        fn run(r1: &'static [u8], r2: &'static [u8], expected: &'static str) {
            let mut r1 = IDRange::parser().run(r1).unwrap();
            let mut r2 = IDRange::parser().run(r2).unwrap();
            let res = r1.make_room(&mut r2);
            assert_eq!(format!("{r1} {r2} => {res:?}"), expected);
        }

        run(b"1-3", b"4-6", "1-3 4-6 => None");
        run(b"1-7", b"2-3", "1-7 0-0 => None");
        run(b"2-3", b"1-7", "2-3 1-1 => Some(4-7)");
        run(b"1-4", b"2-7", "1-4 5-7 => None");
        run(b"2-7", b"1-4", "2-7 1-1 => None");
    }
}
