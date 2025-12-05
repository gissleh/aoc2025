use common::aoc::Runner;
use common::parser::{Parser, uint};

pub fn main(r: &mut Runner, input: &[u8]) {
    let (numbers, ops) = r.prep("Parse", || parse(input));

    r.part("Part 1", || part_1(&numbers, &ops));
    r.clear_tail();
    r.part("Part 1 (No Parse)", || part_1_no_parse(&input));
    r.part("Part 2 (No Parse)", || part_2(&input));

    r.link("Part 1", "Part 2 (No Parse)");

    r.info("Numbers", numbers.len());
    r.info("Ops", ops.len());
}

fn part_1(numbers: &[u64], ops: &[u8]) -> u64 {
    ops.iter()
        .enumerate()
        .map(|(i, op)| {
            let iter = (i..numbers.len()).step_by(ops.len()).map(|i| numbers[i]);

            match *op {
                b'*' => iter.product::<u64>(),
                b'+' => iter.sum::<u64>(),
                _ => unreachable!(),
            }
        })
        .sum()
}

fn part_1_no_parse(input: &[u8]) -> u64 {
    let line_width = input.iter().position(|v| *v == b'\n').unwrap() + 1;
    let line_count = input.len() / line_width;
    let ops_line = &input[line_width * (line_count - 1)..];

    #[cfg(test)]
    assert_eq!(line_count * line_width, input.len());

    let mut grand_total = 0;

    let mut offset = 0;
    while offset < (line_width - 1) {
        let op = ops_line[offset];
        let op_len = ops_line[offset + 1..]
            .iter()
            .position(|p| *p != b' ')
            .unwrap()
            + 1;
        let iter = (0..line_count - 1)
            .map(|i| &input[(i * line_width) + offset..(i * line_width) + offset + op_len])
            .map(|i| {
                i.iter().fold(0u64, |prev, curr| {
                    if *curr != b' ' {
                        (prev * 10) + (*curr - b'0') as u64
                    } else {
                        prev
                    }
                })
            });

        #[cfg(test)]
        {
            for (i, v) in iter.clone().enumerate() {
                if i > 0 {
                    print!(" {} ", op as char);
                }
                print!("{v}")
            }

            if op == b'*' {
                println!(" = {}", iter.clone().product::<u64>());
            } else {
                println!(" = {}", iter.clone().sum::<u64>());
            }
        }

        if op == b'*' {
            grand_total += iter.product::<u64>();
        } else {
            grand_total += iter.sum::<u64>();
        }

        offset += op_len;
    }

    grand_total
}

fn part_2(input: &[u8]) -> u64 {
    let line_width = input.iter().position(|v| *v == b'\n').unwrap() + 1;
    let line_count = input.len() / line_width;
    let ops_offset = line_width * (line_count - 1);

    #[cfg(test)]
    assert_eq!(line_count * line_width, input.len());

    let mut current_op = b'+';
    let mut grand_total = 0u64;
    let mut curr2 = 0u64;
    for i in 0..(line_width - 1) {
        let mut seen = false;
        if input[ops_offset + i] != b' ' {
            #[cfg(test)]
            println!("{}= {curr2}", current_op as char);
            grand_total += curr2;
            current_op = input[ops_offset + i];
            if current_op == b'*' {
                curr2 = 1;
            } else {
                curr2 = 0;
            }
        }

        let mut curr = 0u64;
        for j in 0..line_count - 1 {
            let ch = input[(j * line_width) + i];
            if ch != b' ' {
                curr = (curr * 10) + (ch - b'0') as u64;
                seen = true;
            }
        }

        if seen {
            if current_op == b'*' {
                #[cfg(test)]
                print!("{curr} ");

                curr2 *= curr;
            } else {
                #[cfg(test)]
                print!("{curr} ");

                curr2 += curr;
            }
        }
    }

    #[cfg(test)]
    println!("{}= {curr2}", current_op as char);

    grand_total + curr2
}

fn parse(input: &[u8]) -> (Vec<u64>, Vec<u8>) {
    uint()
        .and_skip_any(b' '.or(b'\n'))
        .repeat::<Vec<_>>()
        .and(b'*'.or(b'+').and_skip_any(b' ').repeat::<Vec<_>>())
        .run(input)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  \n";

    #[test]
    fn part_1_no_parse_works_on_example() {
        assert_eq!(part_1_no_parse(EXAMPLE), 4277556);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_eq!(part_2(EXAMPLE), 3263827);
    }
}
