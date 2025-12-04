use crate::parser::{Input, Parser};
use std::marker::PhantomData;

pub struct Map<PI, TI, TM, F> {
    f: F,
    inner: PI,
    spooky_ghost: PhantomData<(TI, TM)>,
}

impl<PI, TI, TM, F> Map<PI, TI, TM, F> {
    #[inline]
    pub fn new(inner: PI, f: F) -> Self {
        Self {
            f,
            inner,
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, PI, TI, TM, F> Parser<'i, TM> for Map<PI, TI, TM, F>
where
    PI: Parser<'i, TI>,
    F: Fn(TI) -> TM,
{
    #[inline]
    fn parse(&self, input: Input<'i>) -> Option<(TM, Input<'i>)> {
        self.inner
            .parse(input)
            .map(|(v, input)| ((self.f)(v), input))
    }

    #[inline]
    fn parse_discard(&self, input: Input<'i>) -> Option<Input<'i>> {
        self.inner.parse_discard(input)
    }

    #[inline]
    fn find_parsable(&self, input: Input<'i>) -> Option<(TM, usize, Input<'i>)> {
        self.inner
            .find_parsable(input)
            .map(|(v, index, input)| ((self.f)(v), index, input))
    }
}
