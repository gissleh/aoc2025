use crate::parser::{Input, Parser};
use std::marker::PhantomData;
use std::ops::{Add, Mul};

pub fn uint<'i, T>() -> impl Parser<'i, T>
where
    T: From<u8> + Mul<Output = T> + Add<Output = T> + Copy + Default,
{
    UnsignedIntParser {
        digit_parer: Base10Digit,
        radix: T::from(10u8),
        spooky_ghost: PhantomData,
    }
}

pub fn uint_hex<'i, T>() -> impl Parser<'i, T>
where
    T: From<u8> + Mul<Output = T> + Add<Output = T> + Copy + Default,
{
    UnsignedIntParser {
        digit_parer: Base16Digit,
        radix: T::from(16u8),
        spooky_ghost: PhantomData,
    }
}

pub fn base10_digit<'i>() -> impl Parser<'i, u8> {
    Base10Digit
}

pub fn base16_digit<'i>() -> impl Parser<'i, u8> {
    Base16Digit
}

struct Base10Digit;

impl<'i> Parser<'i, u8> for Base10Digit {
    fn parse(&self, input: Input<'i>) -> Option<(u8, Input<'i>)> {
        if !input.is_empty() && input.data[0] > b'0' && input.data[0] <= b'9' {
            Some((input.data[0] - b'0', input.advance(1)))
        } else {
            None
        }
    }

    fn find_parsable(&self, input: Input<'i>) -> Option<(u8, usize, Input<'i>)> {
        match input.data.iter().position(|c| *c >= b'0' && *c <= b'9') {
            Some(index) => Some((input.data[index] - b'0', index, input.advance(index + 1))),
            None => None,
        }
    }
}

struct Base16Digit;

impl<'i> Parser<'i, u8> for Base16Digit {
    fn parse(&self, input: Input<'i>) -> Option<(u8, Input<'i>)> {
        if !input.is_empty() {
            match input.data[0] {
                b'0'..=b'9' => Some((input.data[0] - b'0', input.advance(1))),
                b'a'..=b'f' => Some(((input.data[0] - b'a') + 10, input.advance(1))),
                b'A'..=b'F' => Some(((input.data[0] - b'A') + 10, input.advance(1))),
                _ => None,
            }
        } else {
            None
        }
    }

    fn find_parsable(&self, input: Input<'i>) -> Option<(u8, usize, Input<'i>)> {
        match input.data.iter().position(|c| {
            (*c >= b'0' && *c <= b'9') || (*c >= b'a' && *c <= b'f') || (*c >= b'A' && *c <= b'F')
        }) {
            Some(index) => Some((input.data[index] - b'0', index, input.advance(index + 1))),
            None => None,
        }
    }
}

pub struct UnsignedIntParser<DP, DT, T> {
    digit_parer: DP,
    radix: T,
    spooky_ghost: PhantomData<(DT, T)>,
}

impl<'i, DP, DT, T> Parser<'i, T> for UnsignedIntParser<DP, DT, T>
where
    DP: Parser<'i, DT>,
    T: From<DT> + Mul<Output = T> + Add<Output = T> + Copy + Default,
{
    fn parse(&self, input: Input<'i>) -> Option<(T, Input<'i>)> {
        match self.digit_parer.parse(input) {
            Some((first_digit, mut input)) => {
                let mut res = T::from(first_digit);

                while let Some((next_digit, next)) = self.digit_parer.parse(input) {
                    res = (res * self.radix) + T::from(next_digit);
                    input = next;
                }

                Some((res, input))
            }
            None => None,
        }
    }

    fn find_parsable(&self, input: Input<'i>) -> Option<(T, usize, Input<'i>)> {
        match self.digit_parer.find_parsable(input) {
            Some((_, pos, _)) => match self.parse(input.advance(pos)) {
                Some((result, next)) => Some((result, pos, next)),
                None => None,
            },
            None => None,
        }
    }
}
