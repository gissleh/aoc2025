use common::aoc::Runner;
use common::geo::Directions2D;
use common::grid::{Grid, GridCoordinate, GridCoordinate2D};
use hashbrown::HashSet;
use std::mem;

const BLANK: u8 = b'.';
const ROLL: u8 = b'@';

pub fn main(r: &mut Runner, input: &[u8]) {
    let parsed = r.prep("Parse", || parse(input));
    r.part("Part 1", || part_1(&parsed));
    r.part("Part 2", || part_2(&parsed));

    r.info("Width", &parsed.size().0);
    r.info("Height", &parsed.size().1);

    r.set_tail("Parse");
    r.part("Part 1 (Offsets)", || part_1_offsets(&parsed));
    r.part("Part 2 (Offsets, Limited Search)", || {
        part_2_offsets_limited_search(&parsed)
    });

    r.clear_tail();
    let parsed_set = r.prep("Parse (HS)", || parse_as_set(input));
    r.part("Part 1 (HS)", || part_1_set(&parsed_set));
    r.part("Part 2 (HS)", || part_2_set(&parsed_set));

    r.info("HS Length", &parsed_set.len());
}

fn part_1(grid: &Grid<(u8, u8), u8>) -> u32 {
    grid.iter()
        .filter(|(_, c)| **c == ROLL)
        .filter(|(p, _)| {
            p.eight_neighbors()
                .iter()
                .filter(|v| grid[**v] == ROLL)
                .take(4)
                .count()
                < 4
        })
        .count() as u32
}

fn neighbor_offsets(i: usize, w: usize) -> [usize; 8] {
    let up = i - w;
    let down = i + w;

    [up - 1, up, up + 1, i - 1, i + 1, down - 1, down, down + 1]
}

fn part_1_offsets(grid: &Grid<(u8, u8), u8>) -> u32 {
    let width = grid.size().x();
    let data = grid.data();
    let padding = width + 1;

    (padding..data.len() - padding)
        .filter(|i| data[*i] == ROLL)
        .filter(|i| {
            neighbor_offsets(*i, width)
                .iter()
                .filter(|c| data[**c] == ROLL)
                .take(4)
                .count()
                < 4
        })
        .count() as u32
}

fn part_2_offsets_limited_search(grid: &Grid<(u8, u8), u8>) -> u32 {
    let width = grid.size().x();
    let padding = width + 1;
    let mut grid = grid.data().to_vec();
    let mut total = 0;
    let mut search = HashSet::with_capacity(2048);
    let mut next_search = HashSet::with_capacity(2048);

    for i in padding..grid.len() - padding {
        if grid[i] != ROLL {
            continue;
        }

        let neighbors = neighbor_offsets(i, width);

        let neighbors_count = neighbors
            .iter()
            .filter(|i| grid[**i] == ROLL)
            .take(4)
            .count();

        if neighbors_count < 4 {
            total += 1;
            grid[i] = BLANK;

            for n in neighbors.into_iter() {
                search.insert(n);
            }
        }
    }

    while search.len() > 0 {
        next_search.clear();

        for i in search.iter() {
            if grid[*i] != ROLL {
                continue;
            }

            let neighbors = neighbor_offsets(*i, width);

            let neighbors_count = neighbors
                .iter()
                .filter(|i| grid[**i] == ROLL)
                .take(4)
                .count();

            if neighbors_count < 4 {
                total += 1;
                grid[*i] = BLANK;

                for n in neighbors.into_iter() {
                    next_search.insert(n);
                }
            }
        }

        mem::swap(&mut next_search, &mut search);
    }

    total as u32
}

fn part_2(grid: &Grid<(u8, u8), u8>) -> u32 {
    let size = grid.size();
    let mut res = 0;

    let mut grid = grid.clone();

    loop {
        let mut found_any = false;

        for p in size.iter() {
            if grid[p] != ROLL {
                continue;
            }

            let count = p
                .eight_neighbors()
                .iter()
                .filter(|v| grid[**v] == ROLL)
                .take(4)
                .count();

            if count < 4 {
                grid[p] = BLANK;
                res += 1;
                found_any = true;
            }
        }

        if !found_any {
            break;
        }
    }

    res
}

fn parse(input: &[u8]) -> Grid<(u8, u8), u8> {
    Grid::parse(input, 1, BLANK, |c| *c).unwrap()
}

fn part_1_set(grid: &HashSet<(u8, u8)>) -> u32 {
    grid.iter()
        .filter(|p| {
            p.eight_neighbors()
                .iter()
                .filter(|v| grid.contains(*v))
                .take(4)
                .count()
                < 4
        })
        .count() as u32
}

fn part_2_set(grid: &HashSet<(u8, u8)>) -> u32 {
    let mut total = 0;

    let mut grid = grid.clone();
    let mut grabbed = Vec::with_capacity(2048);
    loop {
        grabbed.clear();

        for p in grid.iter() {
            let count = p
                .eight_neighbors()
                .iter()
                .filter(|v| grid.contains(*v))
                .take(4)
                .count();
            if count < 4 {
                total += 1;
                grabbed.push(*p);
            }
        }

        if grabbed.len() == 0 {
            break;
        }

        for p in grabbed.drain(0..) {
            grid.remove(&p);
        }
    }

    total
}

fn parse_as_set(input: &[u8]) -> HashSet<(u8, u8)> {
    let mut set = HashSet::with_capacity(input.len() / 8);
    let mut x = 1u8;
    let mut y = 1u8;

    for ch in input.iter().cloned() {
        match ch {
            b'@' => {
                set.insert((x, y));
                x += 1;
            }
            b'.' => {
                x += 1;
            }
            b'\n' => {
                y += 1;
                x = 1;
            }
            _ => unreachable!(),
        }
    }

    set
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn part_1_works_on_example() {
        let parsed = parse(EXAMPLE);
        assert_eq!(part_1(&parsed), 13);
    }
}
