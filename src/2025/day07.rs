use common::aoc::{BothParts, Runner};
use common::geo::Directions2D;
use common::grid::{Grid, GridCoordinate2D, RawDelimitedGrid};

pub fn main(r: &mut Runner, input: &[u8]) {
    let grid = r.prep("Parse", || parse(input));
    r.part("Part 1", || part_1(grid));
    r.part("Part 2", || part_2(grid));

    r.set_tail("Parse");
    r.part("Both Parts (row-by-row)", || both_parts_row_by_row(grid));

    r.info_debug("Size", grid.size());
}

fn part_1(input: RawDelimitedGrid<(u8, u8), u8>) -> u32 {
    let end_y = input.size().1 - 1;
    let start_pos = input
        .iter()
        .find(|(_, c)| **c == b'S')
        .map(|(p, _)| p)
        .unwrap();
    let mut stack = Vec::with_capacity(64);
    let mut seen = Grid::new_blank(input.size());

    stack.push(start_pos);
    let mut total = 0;
    while let Some(pos) = stack.pop() {
        if pos.1 == end_y {
            continue;
        }

        if seen[pos] {
            continue;
        }
        seen[pos] = true;

        let down = pos.down();
        let c = input[down];
        match c {
            b'.' => stack.push(down),
            b'^' => {
                total += 1;
                stack.push(down.left());
                stack.push(down.right());
            }
            _ => unreachable!("{}", c as char),
        }
    }

    total
}

#[inline]
fn part_2(input: RawDelimitedGrid<(u8, u8), u8>) -> u64 {
    let start_pos = input
        .iter()
        .find(|(_, c)| **c == b'S')
        .map(|(p, _)| p)
        .unwrap();
    let mut cache = Grid::new_blank(input.size());

    part_2_step(input, start_pos, &mut cache)
}

fn part_2_step(
    input: RawDelimitedGrid<(u8, u8), u8>,
    pos: (u8, u8),
    cache: &mut Grid<(u8, u8), u64>,
) -> u64 {
    if pos.1 == input.size().1 - 1 {
        return 1;
    }

    if cache[pos] != 0 {
        cache[pos]
    } else {
        let down = pos.down();
        let c = input[down];
        let res = match c {
            b'.' => part_2_step(input, down, cache),
            b'^' => {
                part_2_step(input, down.left(), cache) + part_2_step(input, down.right(), cache)
            }
            _ => unreachable!("{}", c as char),
        };

        cache[pos] = res;
        res
    }
}

#[inline]
fn both_parts_row_by_row(input: RawDelimitedGrid<(u8, u8), u8>) -> BothParts<u32, u64> {
    let mut line = vec![0; input.size().x()];
    let mut next = line.clone();
    let mut flowing = vec![false; input.size().x()];
    let mut count = 0;
    let start_x = input.data().iter().position(|ch| *ch == b'S').unwrap();
    flowing[start_x] = true;
    line[start_x] = 1;

    for row in input.rows().skip(2).step_by(2) {
        for (i, ch) in row.iter().enumerate() {
            if *ch == b'^' {
                if flowing[i] {
                    count += 1;
                    flowing[i] = false;
                    flowing[i - 1] = true;
                    flowing[i + 1] = true;
                }

                next[i - 1] += line[i];
                next[i + 1] += line[i];
                next[i] = 0;
            }
        }

        line.copy_from_slice(&next);
    }

    BothParts(count, line.iter().sum())
}

fn parse(input: &[u8]) -> RawDelimitedGrid<'_, (u8, u8), u8> {
    RawDelimitedGrid::new(input, b'\n')
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    fn part_1_works_on_example() {
        assert_eq!(part_1(RawDelimitedGrid::new(EXAMPLE, b'\n')), 21);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_eq!(part_2(RawDelimitedGrid::new(EXAMPLE, b'\n')), 40);
    }

    #[test]
    fn both_parts_row_by_row_works_on_example() {
        assert_eq!(
            both_parts_row_by_row(RawDelimitedGrid::new(EXAMPLE, b'\n')),
            BothParts(21, 40)
        );
    }
}
