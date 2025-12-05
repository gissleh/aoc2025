use common::aoc::{BothParts, Runner};
use common::parser::{Parser, uint};
use common::sets::UnionFind;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::ops::Mul;
use std::simd::Simd;
use std::simd::prelude::SimdUint;

pub fn main(r: &mut Runner, input: &[u8]) {
    let positions = r.prep("Parse", || parse(input));
    let pairs = r.prep("Sort Pairs", || pairs_by_distance(&positions));
    r.part("Both Parts", || both_parts::<1000>(&positions, &pairs));
    r.set_tail("Sort Pairs");
    r.part("Both Parts (UF)", || {
        both_parts_uf::<1000>(&positions, &pairs)
    });

    r.info("Junction Boxes", positions.len());
}

fn both_parts<const N: usize>(
    positions: &[[u64; 3]],
    sorted_distances: &BinaryHeap<Distance>,
) -> BothParts<u64, u64> {
    let mut circuits_sizes = Vec::<u16>::with_capacity(128);
    let mut circuit_index = vec![0u16; positions.len()];
    let mut p1_answer = 0;

    circuits_sizes.push(0);
    for (i, Distance { from, to, .. }) in sorted_distances.clone().into_iter_sorted().enumerate() {
        let mut grew = 0;

        if circuit_index[from] != 0 && circuit_index[to] != 0 {
            if circuit_index[from] != circuit_index[to] {
                let old = circuit_index[to];
                let new = circuit_index[from];
                for i in 0..positions.len() {
                    if circuit_index[i] == old {
                        circuit_index[i] = new;
                        circuits_sizes[old as usize] -= 1;
                        circuits_sizes[new as usize] += 1;
                        if circuits_sizes[old as usize] == 0 {
                            break;
                        }
                    }
                }
                grew = new as usize;

                #[cfg(test)]
                println!(
                    "{:?} and {:?} expands circuit {} with {}",
                    positions[from], positions[to], new, old
                );
            } else {
                #[cfg(test)]
                println!(
                    "{:?} and {:?} are in the same circuit",
                    positions[from], positions[to]
                );
            }
        } else if circuit_index[from] != 0 {
            circuit_index[to] = circuit_index[from];
            circuits_sizes[circuit_index[from] as usize] += 1;
            grew = circuit_index[from] as usize;

            #[cfg(test)]
            println!(
                "{:?} joins circuits {} with {:?}",
                positions[to], circuit_index[from], positions[from]
            );
        } else if circuit_index[to] != 0 {
            circuit_index[from] = circuit_index[to];
            circuits_sizes[circuit_index[to] as usize] += 1;
            grew = circuit_index[to] as usize;

            #[cfg(test)]
            println!(
                "{:?} joins circuits {} with {:?}",
                positions[from], circuit_index[to], positions[to]
            );
        } else {
            let next = circuits_sizes.len();
            circuits_sizes.push(2);
            circuit_index[from] = next as u16;
            circuit_index[to] = next as u16;
            grew = next;

            #[cfg(test)]
            println!(
                "{:?} forms circuits {} with {:?}",
                positions[to], circuit_index[to], positions[from]
            );
        }

        if i == N - 1 {
            let mut circuits_sizes = circuits_sizes.clone();
            circuits_sizes.sort();

            #[cfg(test)]
            println!("{:?}", circuits_sizes);

            p1_answer = circuits_sizes
                .iter()
                .rev()
                .take(3)
                .map(|v| *v as u64)
                .product()
        }

        if circuits_sizes[grew] == positions.len() as u16 {
            return BothParts(p1_answer, positions[from][0] * positions[to][0]);
        }
    }

    unreachable!("last connection not found")
}

fn both_parts_uf<const N: usize>(
    positions: &[[u64; 3]],
    sorted_distances: &BinaryHeap<Distance>,
) -> BothParts<u64, u64> {
    let mut circuits = UnionFind::new(positions.len());
    let mut iter = sorted_distances.clone().into_iter_sorted();

    for Distance { from, to, .. } in (&mut iter).take(N) {
        circuits.union(from, to);
    }

    circuits.flatten();
    let p1_answer = circuits
        .top_ranks::<3>()
        .iter()
        .map(|(_, size)| *size as u64)
        .product();

    for Distance { from, to, .. } in iter {
        if let Some(new_group) = circuits.union(from, to) {
            if circuits.is_one_group(new_group) {
                return BothParts(p1_answer, positions[from][0] * positions[to][0]);
            }
        }
    }

    unreachable!("last connection not found")
}

#[inline]
fn parse(input: &[u8]) -> Vec<[u64; 3]> {
    uint()
        .delimited_by(b',')
        .repeat::<[u64; 3]>()
        .delimited_by(b'\n')
        .repeat()
        .run(input)
        .unwrap()
}

#[inline]
fn pairs_by_distance(positions: &[[u64; 3]]) -> BinaryHeap<Distance> {
    fn distance_squared(left: &[u64; 3], right: &[u64; 3]) -> u64 {
        let l = Simd::from_array(*left);
        let r = Simd::from_array(*right);
        let d = l.abs_diff(r);
        d.mul(d).reduce_sum()
    }

    let mut distances = BinaryHeap::with_capacity(positions.len() * positions.len());
    (0..positions.len())
        .flat_map(|a| {
            ((a + 1)..positions.len()).map(move |b| Distance {
                from: a,
                to: b,
                value: distance_squared(&positions[a], &positions[b]),
            })
        })
        .collect_into(&mut distances);

    distances
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Distance {
    from: usize,
    to: usize,
    value: u64,
}

impl PartialOrd<Self> for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value).reverse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[test]
    fn both_parts_works_on_example() {
        let positions = parse(EXAMPLE);
        let distances = pairs_by_distance(&positions);
        assert_eq!(
            both_parts::<10>(&positions, &distances),
            BothParts(40, 25272)
        );
        assert_eq!(
            both_parts_uf::<10>(&positions, &distances),
            BothParts(40, 25272)
        );
    }
}
