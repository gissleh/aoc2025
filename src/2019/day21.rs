use common::aoc::Runner;
use common::intcode::{StepResult, VM, parse_incode};

pub fn main(r: &mut Runner, input: &[u8]) {
    let program = r.prep("Parse", || parse_incode(input));
    r.part("Part 1", || run_spring_droid(&program, SCRIPT_1));
    r.part("Part 2", || run_spring_droid(&program, SCRIPT_2));

    r.info("Program Size", input.len());
}

const SCRIPT_1: &[u8] = b"NOT A T
OR T J
NOT B T
OR T J
NOT C T
OR T J
AND D J
WALK
";

const SCRIPT_2: &[u8] = b"NOT A T
OR T J
NOT B T
OR T J
NOT C T
OR T J
AND D J
NOT T T
OR H T
OR E T
AND T J
RUN
";

#[inline]
fn run_spring_droid(program: &[i64], input: &[u8]) -> i64 {
    let mut input = input.iter().map(|ch| *ch as i64);
    let mut vm = VM::new(program.to_vec());

    while let StepResult::Output(v) = vm.run(&mut input) {
        if v > 256 {
            return v;
        } else {
            #[cfg(debug_assertions)]
            print!("{}", v as u8 as char);
        }
    }

    0
}
