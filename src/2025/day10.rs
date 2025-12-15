use common::aoc::Runner;
use common::math::Matrix;
use common::parser::{Parser, base10_digit, uint};
use common::search::{BFS, KS, Search};
use hashbrown::HashMap;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

pub fn main(r: &mut Runner, input: &[u8]) {
    let machines = r.prep("Parse", || parse(input));

    r.part("Part 1", || part_1(&machines));
    r.part("Part 2", || part_2(&machines));
    r.set_tail("Part 1");
    r.part("Part 2 (Alt)", || part_2_alt(&machines));

    r.info("Machines", machines.len());
}

fn part_1(machines: &[Machine]) -> u32 {
    machines.iter().map(|m| m.fewest_presses_il()).sum()
}

fn part_2(machines: &[Machine]) -> u32 {
    machines.par_iter().map(|m| m.fewest_presses_jl()).sum()
}

fn part_2_alt(machines: &[Machine]) -> u32 {
    machines
        .par_iter()
        .map(|m| {
            let mut cache = HashMap::new();
            let mut pattern_cache = HashMap::new();

            let mut remaining = [0u16; 16];
            for (i, j) in m.joltages.iter().enumerate() {
                remaining[i] = *j;
            }

            #[cfg(test)]
            println!("f({:?})", &remaining[..m.joltages.len()]);

            m.fewest_presses_jl_alt(remaining, &mut cache, &mut pattern_cache, 1)
        })
        .sum()
}

fn parse(input: &[u8]) -> Vec<Machine> {
    Machine::parser()
        .delimited_by(b'\n')
        .repeat()
        .run(input)
        .unwrap()
}

#[derive(Eq, PartialEq, Debug)]
struct Machine {
    state: u16,
    buttons: Vec<u16>,
    joltages: Vec<u16>,
}

impl Machine {
    #[inline]
    fn press_il(state: u16, button: u16) -> u16 {
        state ^ button
    }

    #[inline]
    fn press_jl(state: [u16; 16], button: u16, times: u16) -> Option<[u16; 16]> {
        let mut current = button;
        let mut next = state;
        for i in 0..16 {
            if current & 1 == 1 {
                if next[i] < times {
                    return None;
                }

                next[i] -= times;
            }
            current >>= 1;
            if current == 0 {
                break;
            }
        }

        Some(next)
    }

    #[inline]
    fn possible_presses_lj(&self, state: [u16; 16], button: u16) -> u16 {
        let mut current = button;
        let mut min = u16::MAX;
        for i in 0..16 {
            if current & 1 == 1 {
                min = min.min(self.joltages[i] - state[i]);
            }
            current >>= 1;
        }

        if min != u16::MAX { min } else { 0 }
    }

    fn dependencies(&self) -> [u16; 10] {
        let mut res = [0u16; 10];
        for i in 0..10 {
            let bit = 1 << i;
            for (j, button) in self.buttons.iter().enumerate() {
                if *button & bit != 0 {
                    res[i] |= 1 << j;
                }
            }
        }

        res
    }

    fn fewest_presses_il(&self) -> u32 {
        let mut search = BFS::new([0u64; 1024]);
        search.reset();
        search.push(KS(0, (0u16, 0u16)));

        self.fewest_presses_il_next(self.state, &mut search)
            .unwrap()
            .0
    }

    fn fewest_presses_il_next(
        &self,
        target: u16,
        search: &mut BFS<KS<u16, (u16, u16)>, u16, [u64; 1024]>,
    ) -> Option<(u32, u16)> {
        while let Some(KS(presses_log, (presses, state))) = search.pop() {
            if state == target {
                return Some((presses as u32, presses_log));
            }

            for (i, button) in self.buttons.iter().enumerate() {
                let log_bit = 1 << i;
                if presses_log & log_bit == log_bit {
                    continue;
                }

                search.push(KS(
                    presses_log | log_bit,
                    (presses + 1, Self::press_il(state, *button)),
                ));
            }
        }

        None
    }

    fn il_patterns(&self, il: u16) -> Vec<u16> {
        (0..1 << self.buttons.len())
            .filter(|v| {
                let mut v = *v;
                let mut res = 0;
                for b in self.buttons.iter() {
                    if v & 1 == 1 {
                        res = Self::press_il(res, *b);
                    }
                    v >>= 1;
                }

                res == il
            })
            .collect()
    }

    fn fewest_presses_jl_alt(
        &self,
        remaining: [u16; 16],
        cache: &mut HashMap<[u16; 16], u32>,
        pattern_cache: &mut HashMap<u16, Vec<u16>>,
        level: usize,
    ) -> u32 {
        if remaining == [0u16; 16] {
            return 0;
        } else if let Some(res) = cache.get(&remaining) {
            #[cfg(test)]
            println!("{: <1$}cached {2}", "", level * 2, cache.len(),);

            return *res;
        }

        let mut il = 0;
        for i in 0..remaining.len() {
            if remaining[i] & 1 == 1 {
                il |= 1 << i;
            }
        }

        let patterns = pattern_cache
            .entry(il)
            .or_insert_with(|| self.il_patterns(il))
            .clone();
        let mut best = 10000;
        'check_loop: for mut il_log in patterns {
            let il_presses = il_log.count_ones();
            let mut remaining = remaining;
            for b in self.buttons.iter() {
                if il_log & 1 == 1 {
                    if let Some(next) = Self::press_jl(remaining, *b, 1) {
                        remaining = next;
                    } else {
                        continue 'check_loop;
                    }
                }
                il_log >>= 1;
            }

            let remaining_half = remaining.map(|v| v / 2);

            #[cfg(test)]
            println!(
                "{: <1$}{il_presses} + (2 * f({2:?}))",
                "",
                level * 2,
                &remaining_half[..self.joltages.len()],
            );

            let res = il_presses
                + (2 * self.fewest_presses_jl_alt(remaining_half, cache, pattern_cache, level + 1));
            if res < best {
                best = res;

                if best == 0 {
                    break;
                }
            }
        }

        cache.insert(remaining, best);
        best
    }

    fn fewest_presses_jl(&self) -> u32 {
        let mut matrix = Matrix::<17, 16, f64>([[0.0; 17]; 16]);
        let dependencies = self.dependencies();
        for (i, dep) in dependencies[..self.joltages.len()].iter().enumerate() {
            for j in 0..self.buttons.len() {
                if dep & 1 << j != 0 {
                    matrix[(j, i)] = 1.0;
                }
            }

            matrix[(16, i)] = self.joltages[i] as f64;
        }

        let gauss = matrix.gauss_jordan_elimination(|z| z.abs() < 0.01);
        let mut equations = gauss.equations::<16>();

        let frees = (0..self.buttons.len())
            .filter(|v| equations[*v].is_none())
            .collect::<Vec<_>>();

        let search_length = self
            .buttons
            .iter()
            .map(|b| self.possible_presses_lj([0; 16], *b))
            .max()
            .unwrap();

        if frees.len() == 0 {
            calc_eqs(&equations[..self.buttons.len()]).unwrap().round() as u32
        } else if frees.len() == 1 {
            let mut lowest_found = 1000.0f64;
            let mut was_good = false;
            for n in 0..search_length {
                equations[frees[0]] = Some((n as f64, [0.0; 16]));

                match calc_eqs(&equations[..self.buttons.len()]) {
                    Ok(value) => {
                        lowest_found = lowest_found.min(value);
                        was_good = true;
                    }
                    Err(true) => {
                        if was_good {
                            break;
                        }
                    }
                    Err(false) => {}
                }
            }

            lowest_found.round() as u32
        } else if frees.len() == 2 {
            let mut lowest_found = 1000.0f64;
            let mut last_good_m = 0;
            for m in 0..search_length {
                let mut was_good = false;
                for n in 0..search_length {
                    equations[frees[0]] = Some((m as f64, [0.0; 16]));
                    equations[frees[1]] = Some((n as f64, [0.0; 16]));

                    match calc_eqs(&equations[..self.buttons.len()]) {
                        Ok(value) => {
                            lowest_found = lowest_found.min(value);
                            was_good = true;
                            last_good_m = m;
                        }
                        Err(true) => {
                            if was_good {
                                break;
                            }
                        }
                        Err(false) => {}
                    }
                }

                if last_good_m > 0 && m - last_good_m > 2 {
                    break;
                }
            }

            lowest_found.round() as u32
        } else if frees.len() == 3 {
            let mut lowest_found = 1000.0f64;
            let mut last_good_o = 0;

            for o in 0..search_length {
                let mut last_good_m = 0;
                for m in 0..search_length {
                    let mut was_good = false;
                    for n in 0..search_length {
                        equations[frees[0]] = Some((o as f64, [0.0; 16]));
                        equations[frees[1]] = Some((m as f64, [0.0; 16]));
                        equations[frees[2]] = Some((n as f64, [0.0; 16]));

                        match calc_eqs(&equations[..self.buttons.len()]) {
                            Ok(value) => {
                                lowest_found = lowest_found.min(value);
                                was_good = true;
                                last_good_m = m;
                                last_good_o = o;
                            }
                            Err(true) => {
                                if was_good {
                                    break;
                                }
                            }
                            Err(false) => {}
                        }
                    }

                    if last_good_m > 0 && m - last_good_m > 2 {
                        break;
                    }
                }

                if last_good_o > 0 && o - last_good_o > 2 {
                    break;
                }
            }

            lowest_found.round() as u32
        } else {
            let mut lowest_found = 1000.0f64;
            for mut n in 0..(search_length as u64).pow(frees.len() as u32) {
                for free in frees.iter().copied() {
                    equations[free] = Some(((n % 256) as f64, [0.0; 16]));
                    n /= 256;
                }

                if let Ok(value) = calc_eqs(&equations[..self.buttons.len()]) {
                    lowest_found = lowest_found.min(value);
                }
            }

            lowest_found.round() as u32
        }
    }

    #[inline]
    fn parser<'i>() -> impl Parser<'i, Machine> {
        b'#'.or(b'.')
            .repeat_fold(
                || (0u16, 0u8),
                |acc, curr| {
                    if curr == b'#' {
                        acc.0 |= 1 << acc.1
                    }
                    acc.1 += 1;
                    true
                },
            )
            .map(|(a, _)| a)
            .quoted_by(b'[', b']')
            .and_skip(b' ')
            .and(
                base10_digit()
                    .delimited_by(b',')
                    .repeat_fold(
                        || 0u16,
                        |acc, curr| {
                            *acc |= 1 << curr;
                            true
                        },
                    )
                    .quoted_by(b'(', b')')
                    .delimited_by(b' ')
                    .repeat::<Vec<_>>(),
            )
            .and_skip(b' ')
            .and(
                uint::<u16>()
                    .delimited_by(b',')
                    .repeat::<Vec<_>>()
                    .quoted_by(b'{', b'}'),
            )
            .map(|((state, buttons), joltages)| Self {
                state,
                buttons,
                joltages,
            })
    }
}

fn calc_eqs(equations: &[Option<(f64, [f64; 16])>]) -> Result<f64, bool> {
    let mut sum = 0.0;
    for i in 0..equations.len() {
        let curr = calc_eqs_index(equations, i);

        if curr < -0.02 {
            return Err(true);
        }

        let v = sum % 1.0;
        if v > 0.02 && v < 0.98 {
            return Err(false);
        }

        sum += curr;
    }

    Ok(sum)
}

fn calc_eqs_index(equations: &[Option<(f64, [f64; 16])>], index: usize) -> f64 {
    let (mut value, deps) = equations[index].unwrap();
    for i in 0..equations.len() {
        if deps[i] != 0.0 {
            value += calc_eqs_index(equations, i) * deps[i];
        }
    }

    value
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn parse_works() {
        let machines = parse(EXAMPLE);
        assert_eq!(machines.len(), 3);
        assert_eq!(
            machines[0],
            Machine {
                state: 0b0110,
                buttons: vec![0b1000, 0b1010, 0b0100, 0b1100, 0b101, 0b11],
                joltages: vec![3, 5, 4, 7],
            }
        );
        assert_eq!(
            machines[1],
            Machine {
                state: 0b01000,
                buttons: vec![0b11101, 0b1100, 0b10001, 0b111, 0b11110],
                joltages: vec![7, 5, 12, 7, 2],
            }
        );
        assert_eq!(
            machines[2],
            Machine {
                state: 0b101110,
                buttons: vec![0b11111, 0b11001, 0b110111, 0b110],
                joltages: vec![10, 11, 11, 5, 10, 5],
            }
        );
    }

    #[test]
    fn il_button_presses_work() {
        assert_eq!(Machine::press_il(0, 0b1000), 0b1000);
        assert_eq!(Machine::press_il(0b1000, 0b1010), 0b0010);
        assert_eq!(Machine::press_il(0b0010, 0b100), 0b0110);
    }

    #[test]
    fn dependencies_works() {
        let machines = parse(EXAMPLE);
        assert_eq!(machines.len(), 3);

        assert_eq!(
            machines[0].dependencies(),
            [0b110000, 0b100010, 0b11100, 0b1011, 0, 0, 0, 0, 0, 0]
        )
    }

    #[test]
    fn fewest_presses_il_works() {
        let machines = parse(EXAMPLE);
        assert_eq!(machines.len(), 3);

        assert_eq!(machines[0].fewest_presses_il(), 2);
        assert_eq!(machines[1].fewest_presses_il(), 3);
        assert_eq!(machines[2].fewest_presses_il(), 2);
    }

    #[test]
    fn fewest_presses_il_next_works() {
        let machines = parse(EXAMPLE);
        assert_eq!(machines.len(), 3);

        let mut search = BFS::new([0u64; 1024]);
        search.push(KS(0b0000, (0, 0)));

        // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        assert_eq!(
            machines[0].fewest_presses_il_next(0b1011, &mut search),
            Some((2, 0b100001))
        );
        assert_eq!(
            machines[0].fewest_presses_il_next(0b1011, &mut search),
            Some((3, 0b010110))
        );
        assert_eq!(
            machines[0].fewest_presses_il_next(0b1011, &mut search),
            Some((3, 0b101100))
        );
        assert_eq!(
            machines[0].fewest_presses_il_next(0b1011, &mut search),
            Some((4, 0b011011))
        );
        assert_eq!(
            machines[0].fewest_presses_il_next(0b1011, &mut search),
            None
        );
    }

    #[test]
    fn fewest_presses_il_next_works_2() {
        let machines = parse(EXAMPLE);
        assert_eq!(machines.len(), 3);

        assert_eq!(
            machines[0].il_patterns(0b1011),
            vec![0b010110, 0b011011, 0b100001, 0b101100,]
        )
    }

    #[test]
    fn fewest_presses_jl_works() {
        let machines = parse(EXAMPLE);
        assert_eq!(machines.len(), 3);

        assert_eq!(machines[0].fewest_presses_jl(), 10);
        assert_eq!(machines[1].fewest_presses_jl(), 12);
        assert_eq!(machines[2].fewest_presses_jl(), 11);
    }

    #[test]
    fn part_2_alt_works_on_examples() {
        let machines = parse(EXAMPLE);
        assert_eq!(machines.len(), 3);
        assert_eq!(part_2_alt(&machines), 33);
    }
}
