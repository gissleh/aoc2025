use crate::parser::{Input, Parser};
use std::marker::PhantomData;

pub struct Or<PL, PR, T> {
    left: PL,
    right: PR,
    spooky_ghost: PhantomData<T>,
}

impl<PL, PR, T> Or<PL, PR, T> {
    #[inline]
    pub(crate) fn new(left: PL, right: PR) -> Self {
        Self {
            left,
            right,
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, PL, PR, T> Parser<'i, T> for Or<PL, PR, T>
where
    PL: Parser<'i, T>,
    PR: Parser<'i, T>,
{
    fn parse(&self, input: Input<'i>) -> Option<(T, Input<'i>)> {
        if let Some(res) = self.left.parse(input) {
            Some(res)
        } else if let Some(res) = self.right.parse(input) {
            Some(res)
        } else {
            None
        }
    }

    fn find_parsable(&self, input: Input<'i>) -> Option<(T, usize, Input<'i>)> {
        let left = self.left.find_parsable(input);
        let right = self.right.find_parsable(input);
        match (left, right) {
            (Some((lv, lo, ln)), Some((rv, ro, rn))) => {
                if lo <= ro {
                    Some((lv, lo, ln))
                } else {
                    Some((rv, ro, rn))
                }
            }
            (Some(l_res), None) => Some(l_res),
            (None, Some(r_res)) => Some(r_res),
            (None, None) => None,
        }
    }
}
