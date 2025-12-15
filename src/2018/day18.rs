use common::aoc::Runner;
use common::grid::{Grid, RawDelimitedGrid};
use hashbrown::HashMap;

pub fn main(r: &mut Runner, input: &[u8]) {
    let grid = r.prep("Parse", || parse(input));
    r.part("Part 1", || part_1(&grid));
    r.part("Part 2", || part_2(&grid));
}

fn part_1(grid: &Grid<(usize, usize), u8>) -> u32 {
    #[cfg(test)]
    {
        println!("Initial state:");
        grid.print(|v| *v as char);
    }

    let mut grid = next_map(grid);
    for _minutes in 1..10 {
        #[cfg(test)]
        {
            println!();
            println!("After {_minutes} minutes:");
            grid.print(|v| *v as char);
        }

        grid = next_map(&grid);
    }

    #[cfg(test)]
    {
        println!();
        println!("Final state:");
        grid.print(|v| *v as char);
    }

    resource_value(&grid)
}

fn part_2(grid: &Grid<(usize, usize), u8>) -> u32 {
    let mut last_seen = HashMap::with_capacity(1024);

    let mut grid = next_map(&grid);
    let mut log = Vec::with_capacity(512);
    for minute in 1..1000000000 {
        log.push(resource_value(&grid));
        let next_grid = next_map(&grid);
        if let Some(prev) = last_seen.insert(grid, minute) {
            let slice = &log[prev..];
            return slice[(1000000000 - minute - 1) % slice.len()];
        }

        grid = next_grid;
    }

    resource_value(&grid)
}

fn parse(data: &[u8]) -> Grid<(usize, usize), u8> {
    RawDelimitedGrid::new(data, b'\n').to_editable()
}

fn resource_value(grid: &Grid<(usize, usize), u8>) -> u32 {
    let (trees, yards) = grid
        .data()
        .iter()
        .fold((0, 0), |(trees, yards), c| match c {
            b'#' => (trees, yards + 1),
            b'|' => (trees + 1, yards),
            _ => (trees, yards),
        });

    trees * yards
}

fn next_map(grid: &Grid<(usize, usize), u8>) -> Grid<(usize, usize), u8> {
    let (w, h) = grid.size();
    let mut next = grid.clone();

    #[inline]
    fn count(ch: u8, trees: &mut u32, yards: &mut u32) {
        match ch {
            b'#' => *yards += 1,
            b'|' => *trees += 1,
            _ => {}
        }
    }

    for y in 0..h {
        for x in 0..w {
            let mut trees = 0;
            let mut yards = 0;

            if x > 0 && y > 0 {
                count(grid[(x - 1, y - 1)], &mut trees, &mut yards);
            }
            if x > 0 {
                count(grid[(x - 1, y)], &mut trees, &mut yards);
            }
            if x > 0 && y < h - 1 {
                count(grid[(x - 1, y + 1)], &mut trees, &mut yards);
            }
            if y > 0 {
                count(grid[(x, y - 1)], &mut trees, &mut yards);
            }
            if y < w - 1 {
                count(grid[(x, y + 1)], &mut trees, &mut yards);
            }
            if x < w - 1 && y > 0 {
                count(grid[(x + 1, y - 1)], &mut trees, &mut yards);
            }
            if x < w - 1 {
                count(grid[(x + 1, y)], &mut trees, &mut yards);
            }
            if x < w - 1 && y < h - 1 {
                count(grid[(x + 1, y + 1)], &mut trees, &mut yards);
            }

            match grid[(x, y)] {
                b'.' => {
                    if trees >= 3 {
                        next[(x, y)] = b'|';
                    }
                }
                b'|' => {
                    if yards >= 3 {
                        next[(x, y)] = b'#';
                    }
                }
                b'#' => {
                    if yards < 1 || trees < 1 {
                        next[(x, y)] = b'.';
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    next
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.
";

    #[test]
    fn part_1_works_on_example() {
        let grid = parse(EXAMPLE);
        assert_eq!(part_1(&grid), 1147);
    }
}
