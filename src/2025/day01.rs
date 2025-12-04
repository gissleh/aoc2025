use common::aoc::Runner;

pub fn main(r: &mut Runner, input: &[u8]) {
    let parsed = r.prep("Parse", || parse(input));
    let _ = (r).part("Part 1", || part_1(&parsed));
    let _ = (r).part("Part 2", || part_2(&parsed));
}

fn part_1(data: &[i32]) -> u32 {
    let mut dial = 50;
    let mut count = 0;

    #[cfg(test)]
    println!("The dial starts by pointing at {}", dial);

    for rotation in data.iter().copied() {
        dial = (100 + dial + (rotation % 100)) % 100;
        if dial == 0 {
            count += 1;
        }

        #[cfg(test)]
        {
            let dir_l = if rotation < 0 { 'L' } else { 'R' };
            println!(
                "The dial is rotated {dir_l}{} to point at {dial}",
                rotation.abs()
            )
        }
    }

    count
}

fn part_2(data: &[i32]) -> u32 {
    let mut dial = 50;
    let mut count = 0;

    #[cfg(test)]
    println!("The dial starts by pointing at {}", dial);

    for mut rotation in data.iter().copied() {
        #[cfg(test)]
        let prev_count = count;
        let prev_dial = dial;

        if rotation > 100 {
            count += (rotation / 100) as u32;
            rotation -= 100 * (rotation / 100);
        }
        if rotation < -100 {
            count += (-rotation / 100) as u32;
            rotation += 100 * (-rotation / 100);
        }

        dial += rotation;
        if dial < 0 {
            dial += 100;
            if prev_dial != 0 {
                count += 1;
            }
        } else if dial > 99 {
            dial -= 100;
            if prev_dial != 0 {
                count += 1;
            }
        } else if dial == 0 {
            count += 1;
        }

        #[cfg(test)]
        {
            let dir_l = if rotation < 0 { 'L' } else { 'R' };
            println!(
                "The dial is rotated {dir_l}{} to point at {dial}",
                rotation.abs()
            );
            if count != prev_count {
                println!("+{}", count - prev_count);
            }
        }
    }

    count
}

fn parse(input: &[u8]) -> Vec<i32> {
    let mut res = Vec::with_capacity(input.len() / 3);
    let mut curr = 0;
    let mut dir = 1;

    for ch in input.iter() {
        match ch {
            b'L' => dir = -1,
            b'R' => dir = 1,
            b'0'..=b'9' => curr = curr * 10 + (ch - b'0') as i32,
            b'\n' => {
                res.push(curr * dir);
                curr = 0;
                dir = 0;
            }
            _ => unreachable!(),
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &[u8] = b"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn parse_example() {
        let parsed = parse(EXAMPLE_1);
        assert_eq!(parsed.len(), 10);
        assert_eq!(parsed[0], -68);
        assert_eq!(parsed[1], -30);
        assert_eq!(parsed[2], 48);
    }

    #[test]
    fn part_1_works_on_example() {
        let parsed = parse(EXAMPLE_1);
        assert_eq!(part_1(&parsed), 3u32);
    }

    #[test]
    fn part_2_works_on_example() {
        let parsed = parse(EXAMPLE_1);
        assert_eq!(part_2(&parsed), 6u32);
    }
}
