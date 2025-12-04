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
    fn parse(&self, input: Input<'i>) -> Option<((TL, TR), Input<'i>)> {
        let (vl, input) = self.left.parse(input)?;
        let (vr, input) = self.right.parse(input)?;
        Some(((vl, vr), input))
    }

    fn find_parsable(&self, input: Input<'i>) -> Option<((TL, TR), usize, Input<'i>)> {
        let (vl, offset, input) = self.left.find_parsable(input)?;
        let (vr, input) = self.right.parse(input)?;
        Some(((vl, vr), offset, input))
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
    fn parse(&self, input: Input<'i>) -> Option<(TL, Input<'i>)> {
        let (vl, input) = self.left.parse(input)?;
        let input = self.right.parse_discard(input)?;
        Some((vl, input))
    }

    fn find_parsable(&self, input: Input<'i>) -> Option<(TL, usize, Input<'i>)> {
        let (vl, offset, input) = self.left.find_parsable(input)?;
        let input = self.right.parse_discard(input)?;
        Some((vl, offset, input))
    }
}
