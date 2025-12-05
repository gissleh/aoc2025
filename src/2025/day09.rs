use common::aoc::Runner;
use common::parser::{Parser, uint};
use std::cmp::minmax;

pub fn main(r: &mut Runner, input: &[u8]) {
    let input = r.prep("Parse", || parse(input));
    r.part("Part 1", || part_1(&input));
    r.part("Part 2", || part_2(&input));
}

fn part_1(input: &[(u32, u32)]) -> u64 {
    input
        .iter()
        .enumerate()
        .flat_map(move |(i, (x1, y1))| {
            input[i + 1..]
                .iter()
                .map(|(x2, y2)| (x1.abs_diff(*x2) + 1) as u64 * (y1.abs_diff(*y2) + 1) as u64)
        })
        .max()
        .unwrap()
}

fn part_2(input: &[(u32, u32)]) -> u64 {
    let mut lines = input
        .array_windows()
        .map(|[a, b]| Line::new(*a, *b))
        .collect::<Vec<_>>();
    lines.push(Line::new(input[0], input[input.len() - 1]));

    let mut biggest_area = 0;
    for (i, (x1, y1)) in input.iter().copied().enumerate() {
        'j_loop: for (x2, y2) in input[i + 1..].iter().copied() {
            // Find the rectangles corners and skip those that wouldn't have an inner rectangle.
            let [min_x, max_x] = minmax(x1, x2);
            let [min_y, max_y] = minmax(y1, y2);
            if max_x - min_x < 2 || max_y - min_y < 2 {
                continue;
            }

            // Get area and skip if it's smaller than the biggest found.
            let area = (1 + (max_y - min_y)) as u64 * (1 + (max_x - min_x)) as u64;
            if biggest_area > area {
                continue;
            }

            // Check if any red tiles are within the inner rectangle.
            let inner_x1 = min_x + 1;
            let inner_x2 = max_x - 1;
            let inner_y1 = min_y + 1;
            let inner_y2 = max_y - 1;
            for (rx, ry) in input.iter() {
                if *rx >= inner_x1 && *rx <= inner_x2 && *ry >= inner_y1 && *ry <= inner_y2 {
                    continue 'j_loop;
                }
            }

            // Check if the shape touches the sides of the inner rectangle.
            let top = Line::Horizontal(inner_y1, inner_x1, inner_x2);
            let left = Line::Vertical(inner_x1, inner_y1, inner_y2);
            let right = Line::Vertical(inner_x2, inner_y1, inner_y2);
            let bottom = Line::Horizontal(inner_y2, inner_x1, inner_x2);
            let found = lines
                .iter()
                .find(|l| {
                    l.intersects(&top)
                        || l.intersects(&left)
                        || l.intersects(&right)
                        || l.intersects(&bottom)
                })
                .is_some();
            if found {
                continue;
            }

            #[cfg(test)]
            println!("{x1},{y1}, {x2},{y2} {area}");

            biggest_area = area;
        }
    }

    biggest_area
}

#[derive(Eq, PartialEq, Debug)]
enum Line {
    Horizontal(u32, u32, u32),
    Vertical(u32, u32, u32),
}

impl Line {
    #[inline]
    fn intersects(&self, other: &Line) -> bool {
        #[inline]
        fn range_overlaps(a1: u32, a2: u32, b1: u32, b2: u32) -> bool {
            (a1 <= b1 && a2 >= b2) // a-------b---b------a
                || (a1 <= b1 && a2 >= b1) // a-----b-------...
                || (a1 <= b2 && a2 >= b2) // .......-----b-----a
                || (b1 <= a1 && b2 >= a2) // b-------a---a------b
                || (b1 <= a1 && b2 >= a1) // b-----a-------...
                || (b1 <= a2 && b2 >= a2) // .......-----a-----b
        }

        match self {
            Self::Horizontal(sy, sx1, sx2) => match other {
                Self::Horizontal(oy, ox1, ox2) => {
                    sy == oy && range_overlaps(*sx1, *sx2, *ox1, *ox2)
                }
                Self::Vertical(ox, oy1, oy2) => ox >= sx1 && ox <= sx2 && oy1 <= sy && oy2 >= sy,
            },
            Self::Vertical(sx, sy1, sy2) => match other {
                Self::Horizontal(oy, ox1, ox2) => oy >= sy1 && oy <= sy2 && ox1 <= sx && ox2 >= sx,
                Self::Vertical(ox, oy1, oy2) => sx == ox && range_overlaps(*sy1, *sy2, *oy1, *oy2),
            },
        }
    }

    #[inline]
    fn new((x1, y1): (u32, u32), (x2, y2): (u32, u32)) -> Self {
        if x1 == x2 {
            let [y1, y2] = minmax(y1, y2);
            Self::Vertical(x1, y1, y2)
        } else {
            let [x1, x2] = minmax(x1, x2);
            Self::Horizontal(y1, x1, x2)
        }
    }
}

fn parse(input: &[u8]) -> Vec<(u32, u32)> {
    uint()
        .and_skip(b',')
        .and(uint())
        .delimited_by(b'\n')
        .repeat()
        .run(input)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[test]
    fn line_intersections() {
        use Line::*;
        fn run(a: Line, b: Line) -> bool {
            println!("Checking {a:?} and {b:?}");

            assert_eq!(a.intersects(&b), b.intersects(&a));
            a.intersects(&b) && b.intersects(&a)
        }

        assert_eq!(run(Horizontal(10, 1, 15), Horizontal(11, 1, 15)), false);
        assert_eq!(run(Horizontal(10, 1, 15), Horizontal(10, 4, 7)), true);
        assert_eq!(run(Horizontal(10, 1, 15), Vertical(10, 5, 15)), true);
        assert_eq!(run(Horizontal(10, 1, 15), Vertical(10, 5, 9)), false);
        assert_eq!(run(Horizontal(10, 1, 15), Vertical(10, 11, 15)), false);

        assert_eq!(run(Vertical(10, 1, 15), Vertical(11, 1, 15)), false);
        assert_eq!(run(Vertical(10, 1, 15), Vertical(10, 4, 7)), true);
        assert_eq!(run(Vertical(10, 1, 15), Horizontal(10, 5, 15)), true);
        assert_eq!(run(Vertical(10, 1, 15), Horizontal(10, 5, 9)), false);
        assert_eq!(run(Vertical(10, 1, 15), Horizontal(10, 11, 15)), false);
    }

    #[test]
    fn part_1_works_on_example() {
        assert_eq!(part_1(&parse(EXAMPLE)), 50);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_eq!(part_2(&parse(EXAMPLE)), 24);
    }
}
