#![feature(iter_collect_into)]
#![feature(portable_simd)]
#![feature(binary_heap_into_iter_sorted)]
#![feature(array_windows)]
#![feature(cmp_minmax)]
#![feature(iter_array_chunks)]

use common::aoc;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

fn main() {
    aoc::run(2025, 1, day01::main);
    aoc::run(2025, 2, day02::main);
    aoc::run(2025, 3, day03::main);
    aoc::run(2025, 4, day04::main);
    aoc::run(2025, 5, day05::main);
    aoc::run(2025, 6, day06::main);
    aoc::run(2025, 7, day07::main);
    aoc::run(2025, 8, day08::main);
    aoc::run(2025, 9, day09::main);
    aoc::run(2025, 10, day10::main);
    aoc::run(2025, 11, day11::main);
    aoc::run(2025, 12, day12::main);
}
