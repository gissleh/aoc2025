use crate::parser::{Input, Parser};
use std::marker::PhantomData;

pub struct QuotedBy<PI, TI, PL, TL, PR, TR> {
    inner: PI,
    left: PL,
    right: PR,
    spooky_ghost: PhantomData<(TI, TL, TR)>,
}

impl<PI, TI, PL, TL, PR, TR> QuotedBy<PI, TI, PL, TL, PR, TR> {
    #[inline]
    pub fn new(inner: PI, left: PL, right: PR) -> Self {
        Self {
            inner,
            left,
            right,
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, PI, TI, PL, TL, PR, TR> Parser<'i, TI> for QuotedBy<PI, TI, PL, TL, PR, TR>
where
    PI: Parser<'i, TI>,
    PL: Parser<'i, TL>,
    PR: Parser<'i, TR>,
{
    fn parse(&self, input: Input<'i>) -> Option<(TI, Input<'i>)> {
        let (_, mut input) = self.left.parse(input)?;
        let (_, offset, after) = self.right.find_parsable(input)?;
        input.data = &input.data[..offset];
        let (res, input) = self.inner.parse(input)?;
        if !input.is_empty() {
            return None;
        }

        Some((res, after))
    }
}
