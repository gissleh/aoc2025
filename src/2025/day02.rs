use common::aoc::Runner;
use common::parser::{Parser, uint};
use hashbrown::HashSet;
use std::cmp::max;
use std::fmt::{Debug, Formatter};

pub fn main(r: &mut Runner, input: &[u8]) {
    let parsed = r.prep("Parse", || parse(input));
    (r).part("Part 1", || part_1(&parsed));
    (r).part("Part 2", || part_2(&parsed));

    r.info(
        "IDs",
        &parsed.iter().map(|v| (v.to - v.from) + 1).sum::<u64>(),
    );

    r.info("Ranges", &parsed.len());
}

fn part_1(ranges: &[IDRange]) -> u64 {
    ranges
        .iter()
        .map(|r| r.repeating_ids(1).map(|p| p.as_number()).sum::<u64>())
        .sum()
}

fn part_2(ranges: &[IDRange]) -> u64 {
    let mut seen = HashSet::new();

    ranges
        .iter()
        .flat_map(|r| (1..7).flat_map(|reps| r.repeating_ids(reps)))
        .map(move |p| {
            // TODO: Find a less hacky way of doing this that won't count 222222 as 2x6, 22x3 and 222x2.
            if !seen.insert(p.data) {
                return 0;
            }

            #[cfg(test)]
            println!("{p:?}");

            p.as_number()
        })
        .sum()
}

fn parse(input: &[u8]) -> Vec<IDRange> {
    IDRange::parser()
        .delimited_by(b',')
        .repeat::<Vec<_>>()
        .run(input)
        .unwrap()
}

struct IDRange {
    from: u64,
    to: u64,
}

impl IDRange {
    #[inline]
    fn repeating_ids(&self, reps: u32) -> RepeatingIDs {
        RepeatingIDs::new(self.from, self.to, reps)
    }

    #[inline]
    fn parser<'i>() -> impl Parser<'i, IDRange> {
        uint()
            .and_skip(b'-')
            .and(uint())
            .map(|(from, to)| IDRange { from, to })
    }
}

#[derive(Debug, Eq, PartialEq)]
struct RepeatingIDs {
    from: u64,
    to: u64,
    curr: u64,
    reps: u32,
}

impl Iterator for RepeatingIDs {
    type Item = ProductID;

    fn next(&mut self) -> Option<Self::Item> {
        let power = 10u64.pow(self.curr.ilog10() + 1);
        let mut n = self.curr;
        for _ in 0..self.reps {
            n *= power;
            n += self.curr;

            if n > self.to {
                return None;
            }
        }

        if n < self.from {
            self.curr += 1;
            return self.next();
        }
        if n > self.to {
            return None;
        }

        let product_id = ProductID::from(n);
        self.curr += 1;

        Some(product_id)
    }
}

impl RepeatingIDs {
    fn new(from: u64, to: u64, reps: u32) -> Self {
        let curr_pid = ProductID::from(from);
        let power = 10u64.pow(max(1, (curr_pid.len as u32 + 1) / (reps + 1)));

        let curr = max(from / power.pow(reps), 1);

        Self {
            from,
            to,
            reps,
            curr,
        }
    }
}

#[derive(Clone, Copy)]
struct ProductID {
    data: [u8; 15],
    len: u8,
}

impl Debug for ProductID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in (0..self.len as usize).rev() {
            write!(f, "{}", (self.data[i] + b'0') as char)?;
        }

        Ok(())
    }
}

impl ProductID {
    fn as_number(self) -> u64 {
        let mut res = 0;
        for i in (0..self.len as usize).rev() {
            res *= 10;
            res += self.data[i] as u64;
        }

        res
    }
}

impl From<u64> for ProductID {
    fn from(value: u64) -> Self {
        let mut value = value;
        let mut pos = 0;
        let mut data = [0u8; 15];

        while value > 0 {
            data[pos] = (value % 10) as u8;

            value /= 10;
            pos += 1;
        }

        Self {
            data,
            len: pos as u8,
        }
    }
}

impl PartialEq<Self> for ProductID {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }

        let len = self.len as usize;
        &self.data[..len] == &other.data[..len]
    }
}

impl Eq for ProductID {}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124\n";

    #[test]
    fn product_id_from_works() {
        assert_eq!(
            ProductID::from(22),
            ProductID {
                data: [2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                len: 2,
            }
        );
        assert_eq!(
            ProductID::from(123124),
            ProductID {
                data: [4, 2, 1, 3, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                len: 6,
            }
        );
    }

    #[test]
    fn product_id_equality() {
        assert_eq!(ProductID::from(123), ProductID::from(123));
        assert_ne!(ProductID::from(123), ProductID::from(321));
        assert_ne!(ProductID::from(123), ProductID::from(1123));
    }

    #[test]
    fn id_range_repeats_just_enough() {
        fn run(from: u64, to: u64, reps: u32) -> u64 {
            let r = (IDRange { from, to }).repeating_ids(reps);
            r.curr
        }

        assert_eq!(run(1001, 3007, 1), 10);
        assert_eq!(run(999, 1140, 1), 9);
        assert_eq!(run(1000, 1140, 1), 10);
        assert_eq!(run(100000, 114000, 2), 10);
    }

    #[test]
    fn id_range_repeats_repetitiously() {
        fn run(from: u64, to: u64, reps: u32) -> Vec<u64> {
            (IDRange { from, to })
                .repeating_ids(reps)
                .map(|p| p.as_number())
                .collect::<Vec<_>>()
        }

        assert_eq!(run(101, 307, 1), vec![]);
        assert_eq!(run(1001, 1407, 1), vec![1010, 1111, 1212, 1313]);
        assert_eq!(run(75, 150, 1), vec![77, 88, 99]);
        assert_eq!(run(11, 22, 1), vec![11, 22]);
        assert_eq!(run(998, 1012, 1), vec![1010]);
        assert_eq!(
            run(7, 2020, 1),
            vec![
                11, 22, 33, 44, 55, 66, 77, 88, 99, 1010, 1111, 1212, 1313, 1414, 1515, 1616, 1717,
                1818, 1919, 2020
            ]
        );
        assert_eq!(run(1188511880, 1188511890, 1), vec![1188511885]);
        assert_eq!(run(222220, 222224, 1), vec![222222]);
        assert_eq!(run(446443, 446449, 1), vec![446446]);
        assert_eq!(run(38593856, 38593862, 1), vec![38593859]);
        assert_eq!(
            run(999398, 110110110, 2),
            vec![
                999999, 100100100, 101101101, 102102102, 103103103, 104104104, 105105105,
                106106106, 107107107, 108108108, 109109109, 110110110,
            ]
        );
    }

    #[test]
    fn part_1_works_on_example() {
        let parsed = parse(EXAMPLE);
        assert_eq!(part_1(&parsed), 1227775554);
    }

    #[test]
    fn part_2_works_on_example() {
        let parsed = parse(EXAMPLE);
        assert_eq!(part_2(&parsed), 4174379265);
    }
}
