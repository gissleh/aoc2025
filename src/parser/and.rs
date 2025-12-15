use crate::parser::{Input, Parser};
use std::marker::PhantomData;

pub struct And<PL, PR, TL, TR> {
    left: PL,
    right: PR,
    spooky_ghost: PhantomData<(TL, TR)>,
}

impl<PL, PR, TL, TR> And<PL, PR, TL, TR> {
    #[inline]
    pub(crate) fn new(left: PL, right: PR) -> Self {
        Self {
            left,
            right,
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, PL, PR, TL, TR> Parser<'i, (TL, TR)> for And<PL, PR, TL, TR>
where
    PL: Parser<'i, TL>,
    PR: Parser<'i, TR>,
{
    #[inline]
    fn parse(&self, input: Input<'i>) -> Option<((TL, TR), Input<'i>)> {
        let (vl, input) = self.left.parse(input)?;
        let (vr, input) = self.right.parse(input)?;
        Some(((vl, vr), input))
    }

    fn find_parsable(&self, input: Input<'i>) -> Option<((TL, TR), usize, Input<'i>)> {
        let mut input = input;
        let mut total_offset = 0usize;

        while !input.is_empty() {
            let (vl, offset, next) = self.left.find_parsable(input)?;
            total_offset += offset;
            if let Some((vr, input)) = self.right.parse(next) {
                return Some(((vl, vr), total_offset, input));
            }

            input = next;
        }

        None
    }
}

pub struct AndSkip<PL, PR, TL, TR> {
    left: PL,
    right: PR,
    spooky_ghost: PhantomData<(TL, TR)>,
}

impl<PL, PR, TL, TR> AndSkip<PL, PR, TL, TR> {
    #[inline]
    pub(crate) fn new(left: PL, right: PR) -> Self {
        Self {
            left,
            right,
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, PL, PR, TL, TR> Parser<'i, TL> for AndSkip<PL, PR, TL, TR>
where
    PL: Parser<'i, TL>,
    PR: Parser<'i, TR>,
{
    #[inline]
    fn parse(&self, input: Input<'i>) -> Option<(TL, Input<'i>)> {
        let (vl, input) = self.left.parse(input)?;
        let input = self.right.parse_discard(input)?;
        Some((vl, input))
    }

    fn find_parsable(&self, input: Input<'i>) -> Option<(TL, usize, Input<'i>)> {
        let mut input = input;
        let mut total_offset = 0usize;

        while !input.is_empty() {
            let (vl, offset, next) = self.left.find_parsable(input)?;
            total_offset += offset;
            if let Some(input) = self.right.parse_discard(input) {
                return Some((vl, total_offset, input));
            }

            input = next;
        }

        None
    }
}

pub struct AndSkipAny<PL, PR, TL, TR> {
    left: PL,
    right: PR,
    spooky_ghost: PhantomData<(TL, TR)>,
}

impl<PL, PR, TL, TR> AndSkipAny<PL, PR, TL, TR> {
    #[inline]
    pub(crate) fn new(left: PL, right: PR) -> Self {
        Self {
            left,
            right,
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, PL, PR, TL, TR> Parser<'i, TL> for AndSkipAny<PL, PR, TL, TR>
where
    PL: Parser<'i, TL>,
    PR: Parser<'i, TR>,
{
    fn parse(&self, input: Input<'i>) -> Option<(TL, Input<'i>)> {
        let (vl, mut input) = self.left.parse(input)?;
        while let Some(next) = self.right.parse_discard(input) {
            input = next;
        }
        Some((vl, input))
    }

    fn find_parsable(&self, input: Input<'i>) -> Option<(TL, usize, Input<'i>)> {
        let (vl, offset, mut input) = self.left.find_parsable(input)?;
        while let Some(next) = self.right.parse_discard(input) {
            input = next;
        }
        Some((vl, offset, input))
    }
}

pub struct AndInstead<PL, PR, TL, TR> {
    left: PL,
    right: PR,
    spooky_ghost: PhantomData<(TL, TR)>,
}

impl<PL, PR, TL, TR> AndInstead<PL, PR, TL, TR> {
    #[inline]
    pub(crate) fn new(left: PL, right: PR) -> Self {
        Self {
            left,
            right,
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, PL, PR, TL, TR> Parser<'i, TR> for AndInstead<PL, PR, TL, TR>
where
    PL: Parser<'i, TL>,
    PR: Parser<'i, TR>,
{
    #[inline]
    fn parse(&self, input: Input<'i>) -> Option<(TR, Input<'i>)> {
        let input = self.left.parse_discard(input)?;
        let (vr, input) = self.right.parse(input)?;
        Some((vr, input))
    }

    fn find_parsable(&self, input: Input<'i>) -> Option<(TR, usize, Input<'i>)> {
        let mut input = input;
        let mut total_offset = 0usize;

        while !input.is_empty() {
            let (_, offset, next) = self.left.find_parsable(input)?;
            total_offset += offset;
            if let Some((vr, input)) = self.right.parse(input) {
                return Some((vr, total_offset, input));
            }

            input = next;
        }

        None
    }
}
