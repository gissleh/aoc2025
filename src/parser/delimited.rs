use crate::parser::{Input, Parser};
use std::marker::PhantomData;

pub struct DelimitedBy<PI, TI, PD, TD> {
    inner: PI,
    delimiter: PD,
    spooky_ghost: PhantomData<(TI, TD)>,
}

impl<PI, TI, PD, TD> DelimitedBy<PI, TI, PD, TD> {
    #[inline]
    pub(crate) fn new(inner: PI, delimiter: PD) -> Self {
        Self {
            inner,
            delimiter,
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, PI, TI, PD, TD> Parser<'i, TI> for DelimitedBy<PI, TI, PD, TD>
where
    PI: Parser<'i, TI>,
    PD: Parser<'i, TD>,
{
    #[inline]
    fn parse(&self, input: Input<'i>) -> Option<(TI, Input<'i>)> {
        if input.index > 0 {
            match self.delimiter.parse(input) {
                Some((_, input)) => self.inner.parse(input),
                None => None,
            }
        } else {
            self.inner.parse(input)
        }
    }

    #[inline]
    fn find_parsable(&self, input: Input<'i>) -> Option<(TI, usize, Input<'i>)> {
        if input.index > 0 {
            match self.delimiter.find_parsable(input) {
                Some((_, offset, input)) => match self.inner.parse(input) {
                    Some((res, input)) => Some((res, offset, input)),
                    None => None,
                },
                None => None,
            }
        } else {
            self.inner.find_parsable(input)
        }
    }
}
