mod and;
mod bytes;
mod delimited;
mod map;
mod number;
mod or;
mod quotedby;
mod repeat;

use std::fmt::{Debug, Formatter};
use std::ops::RangeInclusive;

pub use bytes::n_bytes;
pub use number::{base10_digit, base16_digit, int, int_hex, uint, uint_hex};

use crate::parser::and::AndInstead;
use and::AndSkipAny;
use and::{And, AndSkip};
use delimited::DelimitedBy;
use map::Map;
use or::Or;
use quotedby::QuotedBy;
use repeat::{GatherTarget, Repeat, RepeatFind, RepeatFold};

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Input<'i> {
    pub data: &'i [u8],
    pub index: usize,
}

impl<'i> Debug for Input<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.data.len() > 10 {
            write!(
                f,
                "index={} data[..10]={:?} (len={})",
                self.index,
                String::from_utf8_lossy(&self.data[..10]),
                self.data.len()
            )
        } else {
            write!(
                f,
                "index={} data={:?}",
                self.index,
                String::from_utf8_lossy(self.data)
            )
        }
    }
}

impl<'i> Input<'i> {
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline]
    pub fn with_index(&self, index: usize) -> Self {
        Self {
            data: self.data,
            index,
        }
    }

    #[inline]
    pub fn from(data: &'i [u8]) -> Self {
        Self { data, index: 0 }
    }

    #[inline]
    pub fn advance(&self, offset: usize) -> Self {
        #[cfg(debug_assertions)]
        assert!(offset <= self.data.len());

        Self {
            data: &self.data[offset..],
            index: self.index,
        }
    }
}

pub trait Parser<'i, T>: Sized {
    fn parse(&self, input: Input<'i>) -> Option<(T, Input<'i>)>;

    #[inline]
    fn parse_discard(&self, input: Input<'i>) -> Option<Input<'i>> {
        let (_, input) = self.parse(input)?;
        Some(input)
    }

    fn find_parsable(&self, input: Input<'i>) -> Option<(T, usize, Input<'i>)> {
        let mut input = input;
        let mut offset = 0;
        while !input.is_empty() {
            match self.parse(input) {
                Some((res, input)) => {
                    return Some((res, offset, input));
                }
                None => {
                    input = input.advance(1);
                    offset += 1;
                }
            }
        }

        None
    }

    #[inline]
    fn and<TR, PR>(self, pr: PR) -> And<Self, PR, T, TR> {
        And::new(self, pr)
    }

    #[inline]
    fn and_skip<TR, PR>(self, pr: PR) -> AndSkip<Self, PR, T, TR> {
        AndSkip::new(self, pr)
    }

    #[inline]
    fn and_instead<TR, PR>(self, pr: PR) -> AndInstead<Self, PR, T, TR> {
        AndInstead::new(self, pr)
    }

    #[inline]
    fn and_skip_any<TR, PR>(self, pr: PR) -> AndSkipAny<Self, PR, T, TR> {
        AndSkipAny::new(self, pr)
    }

    #[inline]
    fn quoted_by<PL, PR, TL, TR>(self, pl: PL, pr: PR) -> QuotedBy<Self, T, PL, TL, PR, TR> {
        QuotedBy::new(self, pl, pr)
    }

    #[inline]
    fn or<PR>(self, pr: PR) -> Or<Self, PR, T> {
        Or::new(self, pr)
    }

    #[inline]
    fn delimited_by<TD, PD>(self, pd: PD) -> DelimitedBy<Self, T, PD, TD> {
        DelimitedBy::new(self, pd)
    }

    #[inline]
    fn repeat<C: GatherTarget<T>>(self) -> Repeat<Self, T, C> {
        Repeat::new(self, 1)
    }

    #[inline]
    fn repeat_with_capacity<C: GatherTarget<T>>(self, cap: usize) -> Repeat<Self, T, C> {
        Repeat::new(self, cap)
    }

    #[inline]
    fn repeat_find<C: GatherTarget<T>>(self) -> RepeatFind<Self, T, C> {
        RepeatFind::new(self)
    }

    #[inline]
    fn repeat_fold<FI, FS, TF>(self, fi: FI, fs: FS) -> RepeatFold<Self, T, FI, FS, TF>
    where
        FI: Fn() -> TF,
        FS: Fn(&mut TF, T) -> bool,
    {
        RepeatFold::new(self, fi, fs)
    }

    #[inline]
    fn map<F, TM>(self, f: F) -> Map<Self, T, TM, F>
    where
        F: Fn(T) -> TM,
    {
        Map::new(self, f)
    }

    #[inline]
    fn run(&self, input: &'i [u8]) -> Option<T> {
        match self.parse(Input::from(input)) {
            Some((res, _)) => Some(res),
            None => None,
        }
    }
}

impl<'i> Parser<'i, u8> for u8 {
    #[inline]
    fn parse(&self, input: Input<'i>) -> Option<(u8, Input<'i>)> {
        if input.data.get(0) == Some(self) {
            Some((*self, input.advance(1)))
        } else {
            None
        }
    }

    #[inline]
    fn find_parsable(&self, input: Input<'i>) -> Option<(u8, usize, Input<'i>)> {
        input
            .data
            .iter()
            .position(|v| v.eq(self))
            .map(|i| (*self, i, input.advance(i + 1)))
    }
}

impl<'i> Parser<'i, u8> for RangeInclusive<u8> {
    #[inline]
    fn parse(&self, input: Input<'i>) -> Option<(u8, Input<'i>)> {
        let first = input.data.get(0)?;
        if self.contains(first) {
            Some((*first, input.advance(1)))
        } else {
            None
        }
    }

    #[inline]
    fn find_parsable(&self, input: Input<'i>) -> Option<(u8, usize, Input<'i>)> {
        input
            .data
            .iter()
            .enumerate()
            .find(|(_, v)| self.contains(v))
            .map(|(i, v)| (*v, i, input.advance(i + 1)))
    }
}

impl<'i> Parser<'i, &'static [u8]> for &'static [u8] {
    #[inline]
    fn parse(&self, input: Input<'i>) -> Option<(&'static [u8], Input<'i>)> {
        if input.data.starts_with(self) {
            Some((self, input.advance(self.len())))
        } else {
            None
        }
    }

    #[inline]
    fn find_parsable(&self, input: Input<'i>) -> Option<(&'static [u8], usize, Input<'i>)> {
        if input.data.len() < self.len() {
            return None;
        }

        (0..input.data.len() - self.len())
            .find(|i| input.data[*i..].starts_with(self))
            .map(|i| (*self, i, input.advance(i + self.len())))
    }
}

impl<'i, const N: usize> Parser<'i, &'static [u8]> for &'static [u8; N] {
    fn parse(&self, input: Input<'i>) -> Option<(&'static [u8], Input<'i>)> {
        if input.data.starts_with(self.as_slice()) {
            Some((self.as_slice(), input.advance(self.len())))
        } else {
            None
        }
    }

    fn find_parsable(&self, input: Input<'i>) -> Option<(&'static [u8], usize, Input<'i>)> {
        if input.data.len() < self.len() {
            return None;
        }

        (0..input.data.len() - self.len())
            .find(|i| input.data[*i..].starts_with(self.as_slice()))
            .map(|i| (self.as_slice(), i, input.advance(i + self.len())))
    }
}
impl<'i, P1, T1, P2, T2> Parser<'i, (T1, T2)> for (P1, P2)
where
    P1: Parser<'i, T1>,
    P2: Parser<'i, T2>,
{
    #[inline]
    fn parse(&self, input: Input<'i>) -> Option<((T1, T2), Input<'i>)> {
        let (t1, input) = self.0.parse(input)?;
        let (t2, input) = self.1.parse(input)?;
        Some(((t1, t2), input))
    }

    #[inline]
    fn parse_discard(&self, input: Input<'i>) -> Option<Input<'i>> {
        let input = self.0.parse_discard(input)?;
        let input = self.1.parse_discard(input)?;
        Some(input)
    }

    #[inline]
    fn find_parsable(&self, input: Input<'i>) -> Option<((T1, T2), usize, Input<'i>)> {
        let (t1, offset, input) = self.0.find_parsable(input)?;
        let (t2, input) = self.1.parse(input)?;
        Some(((t1, t2), offset, input))
    }
}

impl<'i, P1, T1, P2, T2, P3, T3> Parser<'i, (T1, T2, T3)> for (P1, P2, P3)
where
    P1: Parser<'i, T1>,
    P2: Parser<'i, T2>,
    P3: Parser<'i, T3>,
{
    #[inline]
    fn parse(&self, input: Input<'i>) -> Option<((T1, T2, T3), Input<'i>)> {
        let (t1, input) = self.0.parse(input)?;
        let (t2, input) = self.1.parse(input)?;
        let (t3, input) = self.2.parse(input)?;
        Some(((t1, t2, t3), input))
    }

    #[inline]
    fn parse_discard(&self, input: Input<'i>) -> Option<Input<'i>> {
        let input = self.0.parse_discard(input)?;
        let input = self.1.parse_discard(input)?;
        let input = self.2.parse_discard(input)?;
        Some(input)
    }

    #[inline]
    fn find_parsable(&self, input: Input<'i>) -> Option<((T1, T2, T3), usize, Input<'i>)> {
        let (t1, offset, input) = self.0.find_parsable(input)?;
        let (t2, input) = self.1.parse(input)?;
        let (t3, input) = self.2.parse(input)?;
        Some(((t1, t2, t3), offset, input))
    }
}

impl<'i, P1, T1, P2, T2, P3, T3, P4, T4> Parser<'i, (T1, T2, T3, T4)> for (P1, P2, P3, P4)
where
    P1: Parser<'i, T1>,
    P2: Parser<'i, T2>,
    P3: Parser<'i, T3>,
    P4: Parser<'i, T4>,
{
    #[inline]
    fn parse(&self, input: Input<'i>) -> Option<((T1, T2, T3, T4), Input<'i>)> {
        let (t1, input) = self.0.parse(input)?;
        let (t2, input) = self.1.parse(input)?;
        let (t3, input) = self.2.parse(input)?;
        let (t4, input) = self.3.parse(input)?;
        Some(((t1, t2, t3, t4), input))
    }

    #[inline]
    fn parse_discard(&self, input: Input<'i>) -> Option<Input<'i>> {
        let input = self.0.parse_discard(input)?;
        let input = self.1.parse_discard(input)?;
        let input = self.2.parse_discard(input)?;
        let input = self.3.parse_discard(input)?;
        Some(input)
    }

    #[inline]
    fn find_parsable(&self, input: Input<'i>) -> Option<((T1, T2, T3, T4), usize, Input<'i>)> {
        let (t1, offset, input) = self.0.find_parsable(input)?;
        let (t2, input) = self.1.parse(input)?;
        let (t3, input) = self.2.parse(input)?;
        let (t4, input) = self.3.parse(input)?;
        Some(((t1, t2, t3, t4), offset, input))
    }
}

impl<'i, P1, T1, P2, T2, P3, T3, P4, T4, P5, T5> Parser<'i, (T1, T2, T3, T4, T5)>
    for (P1, P2, P3, P4, P5)
where
    P1: Parser<'i, T1>,
    P2: Parser<'i, T2>,
    P3: Parser<'i, T3>,
    P4: Parser<'i, T4>,
    P5: Parser<'i, T5>,
{
    #[inline]
    fn parse(&self, input: Input<'i>) -> Option<((T1, T2, T3, T4, T5), Input<'i>)> {
        let (t1, input) = self.0.parse(input)?;
        let (t2, input) = self.1.parse(input)?;
        let (t3, input) = self.2.parse(input)?;
        let (t4, input) = self.3.parse(input)?;
        let (t5, input) = self.4.parse(input)?;
        Some(((t1, t2, t3, t4, t5), input))
    }

    #[inline]
    fn parse_discard(&self, input: Input<'i>) -> Option<Input<'i>> {
        let input = self.0.parse_discard(input)?;
        let input = self.1.parse_discard(input)?;
        let input = self.2.parse_discard(input)?;
        let input = self.3.parse_discard(input)?;
        let input = self.4.parse_discard(input)?;
        Some(input)
    }

    #[inline]
    fn find_parsable(&self, input: Input<'i>) -> Option<((T1, T2, T3, T4, T5), usize, Input<'i>)> {
        let (t1, offset, input) = self.0.find_parsable(input)?;
        let (t2, input) = self.1.parse(input)?;
        let (t3, input) = self.2.parse(input)?;
        let (t4, input) = self.3.parse(input)?;
        let (t5, input) = self.4.parse(input)?;
        Some(((t1, t2, t3, t4, t5), offset, input))
    }
}
