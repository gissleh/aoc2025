use common::aoc::{BothParts, Runner};
use common::geo::Directions2D;
use hashbrown::HashMap;
use hashbrown::hash_map::Entry;
use std::cmp::min;

pub fn main(r: &mut Runner, input: &[u8]) {
    r.part("Both Parts", || both_parts(&input[..input.len() - 1]));
}

fn both_parts(input: &[u8]) -> BothParts<u32, u32> {
    let mut map = HashMap::with_capacity(16);
    traverse((0, 0), 0, &input[1..input.len() - 1], &mut map);
    map.iter()
        .map(|(_, v)| *v)
        .fold(BothParts(0, 0), |BothParts(max, faraway_rooms), curr| {
            BothParts(
                max.max(curr),
                if curr >= 1000 {
                    faraway_rooms + 1
                } else {
                    faraway_rooms
                },
            )
        })
}

fn traverse(
    pos: (i16, i16),
    steps: u32,
    expression: &[u8],
    shortest_distances: &mut HashMap<(i16, i16), u32>,
) -> ((i16, i16), u32) {
    let mut curr_pos = pos;
    let mut curr_steps = steps;

    let mut i = 0;
    while i < expression.len() {
        match expression[i] {
            b'|' => {
                curr_pos = pos;
                curr_steps = steps;
                i += 1;
                continue;
            }
            b'(' => {
                let inner = find_closer(&expression[i + 1..]);
                (curr_pos, curr_steps) = traverse(curr_pos, curr_steps, inner, shortest_distances);
                i += inner.len() + 2;
                continue;
            }
            b'N' => curr_pos = curr_pos.up(),
            b'E' => curr_pos = curr_pos.right(),
            b'W' => curr_pos = curr_pos.left(),
            b'S' => curr_pos = curr_pos.down(),
            _ => unreachable!(
                "Unknown char at {} ({:?}) in {}",
                i,
                expression[i] as char,
                if expression.len() > 20 {
                    String::from_utf8_lossy(&expression[i..min(i + 20, expression.len())]) + "..."
                } else {
                    String::from_utf8_lossy(expression)
                },
            ),
        }

        curr_steps += 1;
        i += 1;
        match shortest_distances.entry(curr_pos) {
            Entry::Vacant(e) => {
                e.insert(curr_steps);
            }
            Entry::Occupied(e) => {
                curr_steps = *e.get();
            }
        }
    }

    (curr_pos, curr_steps)
}

pub fn find_closer(data: &[u8]) -> &[u8] {
    let mut level = 0;
    for (i, ch) in data.iter().enumerate() {
        match *ch {
            b'(' => level += 1,
            b')' => {
                if level == 0 {
                    return &data[..i];
                } else {
                    level -= 1
                }
            }
            _ => {}
        }
    }

    panic!("unclosed (");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works_on_examples() {
        assert_eq!(both_parts(b"^WNE$").0, 3);
        assert_eq!(both_parts(b"^ENWWW(NEEE|SSE(EE|N))$").0, 10);
        assert_eq!(
            both_parts(b"^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$").0,
            18
        );
        assert_eq!(
            both_parts(b"^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$").0,
            23
        );
        assert_eq!(
            both_parts(b"^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$").0,
            31
        );
    }
}
