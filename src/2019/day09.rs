use common::aoc::Runner;
use common::intcode::{VM, parse_incode};

pub fn main(r: &mut Runner, input: &[u8]) {
    let input = r.prep("Parse", || parse_incode(input));
    r.part("Part 1", || part_1(&input));
    r.part("Part 2", || part_2(&input));

    r.info("Program Size", input.len());
}

fn part_1(program: &[i64]) -> i64 {
    VM::new(program.to_vec())
        .run(&mut [1i64].into_iter())
        .unwrap_output()
}

fn part_2(program: &[i64]) -> i64 {
    VM::new(program.to_vec())
        .run(&mut [2i64].into_iter())
        .unwrap_output()
}
