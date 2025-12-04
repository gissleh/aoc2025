use common::aoc::Runner;

pub fn main(r: &mut Runner, input: &[u8]) {
    let parsed = r.prep("Parse", || BatteryBanks::parse(input));
    r.part("Part 1", || part_1(&parsed));
    r.part("Part 2", || part_2(&parsed));

    r.info("Width", &parsed.width);
    r.info("Banks", &parsed.len());
}

fn part_1(banks: &BatteryBanks) -> u32 {
    banks.iter().map(|v| v.highest_joltage_2() as u32).sum()
}

fn part_2(banks: &BatteryBanks) -> u64 {
    banks.iter().map(|v| v.highest_joltage::<12>()).sum()
}

struct BatteryBank<'s> {
    joltages: &'s [u8],
}

impl<'s> BatteryBank<'s> {
    fn highest_joltage_2(&self) -> u8 {
        let mut best_1 = 0u8;
        let mut best_1_index = 0usize;

        for (i, joltage_1) in self.joltages[..self.joltages.len() - 1].iter().enumerate() {
            if *joltage_1 > best_1 {
                best_1 = *joltage_1;
                best_1_index = i;
            }
        }

        let best_2 = *self.joltages[best_1_index + 1..].iter().max().unwrap();

        (best_1 * 10) + best_2
    }

    fn highest_joltage<const N: usize>(&self) -> u64 {
        let mut best = [0u8; N];
        let mut start_at = 0usize;

        for current in 0..N {
            let end_at = self.joltages.len() - (N - current);

            for i in start_at..=end_at {
                let joltage = self.joltages[i];
                if joltage > best[current] {
                    best[current] = joltage;
                    start_at = i + 1;
                }
            }
        }

        best.iter().fold(0u64, |p, c| p * 10 + (*c as u64))
    }
}

impl<'s> From<&'s [u8]> for BatteryBank<'s> {
    fn from(value: &'s [u8]) -> Self {
        BatteryBank { joltages: value }
    }
}

struct BatteryBanks {
    data: Vec<u8>,
    width: usize,
}

impl BatteryBanks {
    fn len(&self) -> usize {
        self.data.len() / self.width
    }

    fn iter(&self) -> impl Iterator<Item = BatteryBank<'_>> {
        self.data.chunks(self.width).map(|c| BatteryBank::from(c))
    }

    fn parse(input: &[u8]) -> Self {
        let mut data = Vec::with_capacity(input.len());
        let mut width = 0;

        for ch in input.iter().copied() {
            match ch {
                b'0'..=b'9' => {
                    data.push(ch - b'0');
                }
                b'\n' => {
                    if width == 0 {
                        width = data.len();
                    }
                }
                _ => unreachable!(),
            }
        }

        #[cfg(test)]
        assert_eq!(data.len() % width, 0);

        Self { data, width }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn highest_joltage_p1() {
        fn run(s: &[u8]) -> u8 {
            let mut s = Vec::from(s);
            s.push(b'\n');
            let bb = BatteryBanks::parse(&s);
            bb.iter().next().unwrap().highest_joltage_2()
        }

        assert_eq!(run(b"987654321111111"), 98);
        assert_eq!(run(b"811111111111119"), 89);
        assert_eq!(run(b"234234234234278"), 78);
        assert_eq!(run(b"818181911112111"), 92);
    }

    #[test]
    fn highest_joltage_p2() {
        fn run(s: &[u8]) -> u64 {
            let mut s = Vec::from(s);
            s.push(b'\n');
            let bb = BatteryBanks::parse(&s);
            bb.iter().next().unwrap().highest_joltage::<12>()
        }

        assert_eq!(run(b"987654321111111"), 987654321111);
        assert_eq!(run(b"811111111111119"), 811111111119);
        assert_eq!(run(b"234234234234278"), 434234234278);
        assert_eq!(run(b"818181911112111"), 888911112111);
    }
}
