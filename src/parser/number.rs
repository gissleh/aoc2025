use crate::parser::{Input, Parser};
use std::marker::PhantomData;
use std::ops::{Add, Mul, Neg};

#[inline]
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

#[inline]
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

#[inline]
pub fn int<'i, T>() -> impl Parser<'i, T>
where
    T: From<u8> + Neg<Output = T> + Mul<Output = T> + Add<Output = T> + Copy + Default,
{
    SignedIntParser {
        digit_parer: Base10Digit,
        radix: T::from(10u8),
        spooky_ghost: PhantomData,
    }
}

#[inline]
pub fn int_hex<'i, T>() -> impl Parser<'i, T>
where
    T: From<u8> + Neg<Output = T> + Mul<Output = T> + Add<Output = T> + Copy + Default,
{
    SignedIntParser {
        digit_parer: Base16Digit,
        radix: T::from(16u8),
        spooky_ghost: PhantomData,
    }
}

#[inline]
pub fn base10_digit<'i>() -> impl Parser<'i, u8> {
    Base10Digit
}

#[inline]
pub fn base16_digit<'i>() -> impl Parser<'i, u8> {
    Base16Digit
}

struct Base10Digit;

impl<'i> Parser<'i, u8> for Base10Digit {
    #[inline]
    fn parse(&self, input: Input<'i>) -> Option<(u8, Input<'i>)> {
        if !input.is_empty() && input.data[0] >= b'0' && input.data[0] <= b'9' {
            Some((input.data[0] - b'0', input.advance(1)))
        } else {
            None
        }
    }

    #[inline]
    fn parse_discard(&self, input: Input<'i>) -> Option<Input<'i>> {
        if !input.is_empty() && input.data[0] >= b'0' && input.data[0] <= b'9' {
            Some(input.advance(1))
        } else {
            None
        }
    }

    #[inline]
    fn find_parsable(&self, input: Input<'i>) -> Option<(u8, usize, Input<'i>)> {
        match input.data.iter().position(|c| *c >= b'0' && *c <= b'9') {
            Some(index) => Some((input.data[index] - b'0', index, input.advance(index + 1))),
            None => None,
        }
    }
}

struct Base16Digit;

impl<'i> Parser<'i, u8> for Base16Digit {
    #[inline]
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

    #[inline]
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
        let (first_digit, mut input) = self.digit_parer.parse(input)?;

        let mut res = T::from(first_digit);
        while let Some((next_digit, next)) = self.digit_parer.parse(input) {
            res = (res * self.radix) + T::from(next_digit);
            input = next;
        }

        Some((res, input))
    }

    fn find_parsable(&self, input: Input<'i>) -> Option<(T, usize, Input<'i>)> {
        let (first_digit, offset, mut input) = self.digit_parer.find_parsable(input)?;

        let mut res = T::from(first_digit);
        while let Some((next_digit, next)) = self.digit_parer.parse(input) {
            res = (res * self.radix) + T::from(next_digit);
            input = next;
        }

        Some((res, offset, input))
    }
}

pub struct SignedIntParser<DP, DT, T> {
    digit_parer: DP,
    radix: T,
    spooky_ghost: PhantomData<(DT, T)>,
}

impl<'i, DP, DT, T> Parser<'i, T> for SignedIntParser<DP, DT, T>
where
    DP: Parser<'i, DT>,
    T: From<DT> + Mul<Output = T> + Neg<Output = T> + Add<Output = T> + Copy + Default,
{
    fn parse(&self, mut input: Input<'i>) -> Option<(T, Input<'i>)> {
        let mut negative = false;
        if !input.is_empty() && input.data[0] == b'-' {
            negative = true;
            input = input.advance(1);
        }

        let (first_digit, mut input) = self.digit_parer.parse(input)?;
        let mut res = T::from(first_digit);

        while let Some((next_digit, next)) = self.digit_parer.parse(input) {
            res = (res * self.radix) + T::from(next_digit);
            input = next;
        }

        if negative {
            res = -res;
        }

        Some((res, input))
    }

    fn parse_discard(&self, input: Input<'i>) -> Option<Input<'i>> {
        match self.digit_parer.parse_discard(input) {
            Some(mut input) => {
                while let Some(next) = self.digit_parer.parse_discard(input) {
                    input = next;
                }

                Some(input)
            }
            None => None,
        }
    }

    fn find_parsable(&self, input: Input<'i>) -> Option<(T, usize, Input<'i>)> {
        let (_, mut offset, _) = self.digit_parer.find_parsable(input)?;

        if offset > 0 && input.data[offset - 1] == b'-' {
            offset -= 1;
        }

        let (result, next) = self.parse(input.advance(offset))?;
        Some((result, offset, next))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_parser_works() {
        fn run<N: Sized, P: Parser<'static, N> + Sized>(
            parser: P,
            input: &'static [u8],
        ) -> Option<(N, &'static [u8])> {
            parser
                .parse(Input::from(input))
                .map(|(v, next)| (v, next.data))
        }

        assert_eq!(run(uint(), b"321"), Some((321u32, b"".as_slice())));
        assert_eq!(
            run(uint(), b"99001-99999"),
            Some((99001u64, b"-99999".as_slice()))
        );
    }

    #[test]
    fn digit_parser_works() {
        fn run<P: Parser<'static, u8> + Sized>(
            parser: P,
            input: &'static [u8],
        ) -> Option<(u8, &'static [u8])> {
            parser
                .parse(Input::from(input))
                .map(|(v, next)| (v, next.data))
        }

        assert_eq!(run(base10_digit(), b"321"), Some((3u8, b"21".as_slice())));
        assert_eq!(run(base10_digit(), b"9"), Some((9u8, b"".as_slice())));
        assert_eq!(run(base10_digit(), b"0"), Some((0u8, b"".as_slice())));
        assert_eq!(
            run(base16_digit(), b"dead"),
            Some((13u8, b"ead".as_slice()))
        );
        assert_eq!(run(base16_digit(), b"0"), Some((0u8, b"".as_slice())));
        assert_eq!(run(base16_digit(), b"gg"), None);
        assert_eq!(run(base10_digit(), b"gg"), None);
        assert_eq!(run(base10_digit(), b"aa"), None);
    }
}
