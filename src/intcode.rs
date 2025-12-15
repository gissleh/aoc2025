use crate::parser;
use crate::parser::Parser;
use hashbrown::HashMap;
use num_traits::{AsPrimitive, ConstOne, ConstZero};
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Mul, Neg};

#[derive(Copy, Clone)]
pub struct VM<M, T> {
    ip: usize,
    rb: T,
    memory: M,
}

impl<M, T> VM<M, T>
where
    M: Memory<T>,
    T: Add<Output = T>
        + AddAssign
        + Mul<Output = T>
        + Copy
        + Default
        + AsPrimitive<usize>
        + AsPrimitive<i32>
        + ConstZero
        + ConstOne
        + Eq
        + PartialOrd,
{
    pub fn new(memory: M) -> Self {
        Self {
            memory,
            ip: 0,
            rb: T::default(),
        }
    }

    pub fn run<I: Iterator<Item = T>>(&mut self, input: &mut I) -> StepResult<T> {
        loop {
            let res = self.step(input);
            if res != StepResult::Continue {
                return res;
            }
        }
    }

    pub fn step<I: Iterator<Item = T>>(&mut self, input: &mut I) -> StepResult<T> {
        let instruction: i32 = self.memory.load(self.ip).as_();

        let opcode = instruction % 100;
        let ma = (instruction / 100) % 10;
        let mb = (instruction / 1000) % 10;
        let mc = (instruction / 10000) % 10;

        match opcode {
            1 => {
                *self.param_mut(3, mc) = self.param(1, ma) + self.param(2, mb);
                self.ip += 4;
                StepResult::Continue
            }
            2 => {
                *self.param_mut(3, mc) = self.param(1, ma) * self.param(2, mb);
                self.ip += 4;
                StepResult::Continue
            }
            3 => {
                if let Some(input) = input.next() {
                    *self.param_mut(1, ma) = input;
                    self.ip += 2;

                    StepResult::Continue
                } else {
                    StepResult::Input
                }
            }
            4 => {
                let a = self.param(1, ma);
                self.ip += 2;
                StepResult::Output(a)
            }
            5 => {
                if self.param(1, ma) != T::ZERO {
                    self.ip = self.param(2, mb).as_();
                } else {
                    self.ip += 3;
                }

                StepResult::Continue
            }
            6 => {
                if self.param(1, ma) == T::ZERO {
                    self.ip = self.param(2, mb).as_();
                } else {
                    self.ip += 3;
                }

                StepResult::Continue
            }
            7 => {
                *self.param_mut(3, mc) = if self.param(1, ma) < self.param(2, mb) {
                    T::ONE
                } else {
                    T::ZERO
                };

                self.ip += 4;
                StepResult::Continue
            }
            8 => {
                *self.param_mut(3, mc) = if self.param(1, ma) == self.param(2, mb) {
                    T::ONE
                } else {
                    T::ZERO
                };

                self.ip += 4;
                StepResult::Continue
            }
            9 => {
                self.rb += self.param(1, ma);
                self.ip += 2;
                StepResult::Continue
            }
            99 => StepResult::Exit,
            _ => unreachable!("unknown opcode {opcode} at position {}", self.ip),
        }
    }

    #[inline]
    fn param(&self, offset: usize, m: i32) -> T {
        match m {
            0 => {
                let i = self.memory.load(self.ip + offset).as_();
                self.memory.load(i)
            }
            1 => self.memory.load(self.ip + offset),
            2 => {
                let i = (self.memory.load(self.ip + offset) + self.rb).as_();
                self.memory.load(i)
            }
            _ => unreachable!(),
        }
    }

    #[inline]
    fn param_mut(&mut self, offset: usize, m: i32) -> &mut T {
        match m {
            0 => {
                let i = self.memory.load(self.ip + offset).as_();
                self.memory.load_writable(i)
            }
            2 => {
                let i = (self.memory.load(self.ip + offset) + self.rb).as_();
                self.memory.load_writable(i)
            }
            _ => unreachable!(),
        }
    }
}

pub trait Memory<T>: Sized {
    fn load(&self, index: usize) -> T;
    fn load_writable(&mut self, index: usize) -> &mut T;
}

impl<const N: usize, T> Memory<T> for [T; N]
where
    T: Copy,
{
    #[inline]
    fn load(&self, index: usize) -> T {
        self[index]
    }

    #[inline]
    fn load_writable(&mut self, index: usize) -> &mut T {
        &mut self[index]
    }
}

impl<T> Memory<T> for Vec<T>
where
    T: Copy + ConstZero,
{
    #[inline]
    fn load(&self, index: usize) -> T {
        match self.get(index) {
            Some(v) => *v,
            None => T::ZERO,
        }
    }

    #[inline]
    fn load_writable(&mut self, index: usize) -> &mut T {
        while self.len() <= index {
            self.push(T::ZERO);
        }

        &mut self[index]
    }
}

impl<T> Memory<T> for HashMap<usize, T>
where
    T: Copy + ConstZero,
{
    #[inline]
    fn load(&self, index: usize) -> T {
        match self.get(&index) {
            Some(v) => *v,
            None => T::ZERO,
        }
    }

    #[inline]
    fn load_writable(&mut self, index: usize) -> &mut T {
        self.entry(index).or_insert(T::ZERO)
    }
}

impl<T> Memory<T> for Vec<(usize, T)>
where
    T: Copy + ConstZero,
{
    #[inline]
    fn load(&self, index: usize) -> T {
        match self.binary_search_by_key(&index, |(i, _)| *i) {
            Ok(i) => self[i].1,
            Err(_) => T::ZERO,
        }
    }

    #[inline]
    fn load_writable(&mut self, index: usize) -> &mut T {
        match self.binary_search_by_key(&index, |(i, _)| *i) {
            Ok(i) => &mut self[i].1,
            Err(i) => {
                self.insert(i, (index, T::ZERO));
                &mut self[i].1
            }
        }
    }
}

impl<const N: usize, T, M2> Memory<T> for ([T; N], M2)
where
    T: Copy,
    M2: Memory<T>,
{
    fn load(&self, index: usize) -> T {
        if index < N {
            self.0.load(index)
        } else {
            self.1.load(index - N)
        }
    }

    fn load_writable(&mut self, index: usize) -> &mut T {
        if index < N {
            self.0.load_writable(index)
        } else {
            self.1.load_writable(index - N)
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum StepResult<T> {
    Continue,
    Output(T),
    Input,
    Exit,
}

impl<T> StepResult<T>
where
    T: Debug,
{
    pub fn unwrap_output(self) -> T {
        match self {
            StepResult::Output(v) => v,
            _ => panic!("unwrap called on {self:?}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::intcode::{Memory, StepResult, VM};
    use std::iter::empty;

    #[test]
    fn y2019_examples() {
        fn run<M: Memory<i32>>(input: M) -> M {
            let mut vm = VM::new(input);
            vm.run(&mut empty());
            vm.memory
        }

        assert_eq!(run([1, 0, 0, 0, 99]), [2, 0, 0, 0, 99]);
        assert_eq!(run([2, 3, 0, 3, 99]), [2, 3, 0, 6, 99]);
        assert_eq!(run([2, 4, 4, 5, 99, 0]), [2, 4, 4, 5, 99, 9801]);
        assert_eq!(
            run([1, 1, 1, 4, 99, 5, 6, 0, 99]),
            [30, 1, 1, 4, 2, 5, 6, 0, 99],
        );
        assert_eq!(
            run([1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]),
            [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
        assert_eq!(run([1101, 100, -1, 4, 0]), [1101, 100, -1, 4, 99]);
    }

    #[test]
    fn y2019_io_examples() {
        const D05_LONG_EXAMPLE: [i64; 47] = [
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        fn run<M: Memory<i64>>(program: M, input: &[i64]) -> Vec<i64> {
            let mut vm = VM::new(program);
            let mut res = Vec::with_capacity(1);
            let mut iter = input.iter().copied();

            loop {
                match vm.run(&mut iter) {
                    StepResult::Continue => {}
                    StepResult::Output(o) => {
                        res.push(o);
                    }
                    _ => {
                        break;
                    }
                }
            }

            res
        }

        assert_eq!(run([3, 0, 4, 0, 99], &[16]), vec![16]);
        assert_eq!(run([3, 0, 4, 0, 99], &[4324]), vec![4324]);
        assert_eq!(run([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], &[8]), vec![1]);
        assert_eq!(run([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], &[7]), vec![0]);
        assert_eq!(run([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], &[9]), vec![0]);

        assert_eq!(run(D05_LONG_EXAMPLE, &[7]), vec![999]);
        assert_eq!(run(D05_LONG_EXAMPLE, &[8]), vec![1000]);
        assert_eq!(run(D05_LONG_EXAMPLE, &[9]), vec![1001]);
        assert_eq!(run(D05_LONG_EXAMPLE, &[10]), vec![1001]);

        assert_eq!(
            run(
                vec![
                    109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99
                ],
                &[]
            ),
            vec![
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99
            ]
        );
        assert_eq!(
            run([104, 1125899906842624, 99], &[]),
            vec![1125899906842624]
        );
    }
}

pub fn parse_incode<T>(input: &[u8]) -> Vec<T>
where
    T: From<u8> + Neg<Output = T> + Mul<Output = T> + Add<Output = T> + Copy + Default,
{
    parser::int()
        .delimited_by(b',')
        .repeat_with_capacity(1024)
        .run(input)
        .unwrap()
}
