#![feature(iter_collect_into)]

use common::aoc;

mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    aoc::run(2025, 1, day01::main);
    aoc::run(2025, 2, day02::main);
    aoc::run(2025, 3, day03::main);
    aoc::run(2025, 4, day04::main);
}
