use crate::parser::{Input, Parser};
use std::marker::PhantomData;

pub struct Repeat<PI, TI, C> {
    inner: PI,
    size_hint: usize,
    spooky_ghost: PhantomData<(TI, C)>,
}

impl<PI, TI, C> Repeat<PI, TI, C> {
    #[inline]
    pub(crate) fn new(inner: PI, size_hint: usize) -> Self {
        Self {
            inner,
            size_hint,
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, PI, TI, C> Parser<'i, C> for Repeat<PI, TI, C>
where
    C: GatherTarget<TI>,
    PI: Parser<'i, TI>,
{
    fn parse(&self, input: Input<'i>) -> Option<(C, Input<'i>)> {
        let (first, mut input) = self.inner.parse(input.with_index(0))?;

        let mut res = C::init(self.size_hint);
        if res.gather(0, first) {
            let mut index = 1;
            while let Some((v, next)) = self.inner.parse(input.with_index(index)) {
                input = next;
                if !res.gather(index, v) {
                    break;
                }
                index += 1;
            }
        }

        Some((res, input))
    }

    fn find_parsable(&self, input: Input<'i>) -> Option<(C, usize, Input<'i>)> {
        let (first, offset, mut input) = self.inner.find_parsable(input.with_index(0))?;

        let mut res = C::init(1);
        if res.gather(0, first) {
            let mut index = 1;
            while let Some((v, next)) = self.inner.parse(input.with_index(index)) {
                input = next;
                if !res.gather(index, v) {
                    break;
                }
                index += 1;
            }
        }

        Some((res, offset, input))
    }
}

pub struct RepeatFold<PI, TI, FI, FS, T> {
    inner: PI,
    f_init: FI,
    f_step: FS,
    spooky_ghost: PhantomData<(TI, T)>,
}

impl<PI, TI, FI, FS, T> RepeatFold<PI, TI, FI, FS, T> {
    #[inline]
    pub(crate) fn new(inner: PI, f_init: FI, f_step: FS) -> Self {
        Self {
            inner,
            f_init,
            f_step,
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, PI, TI, FI, FS, T> Parser<'i, T> for RepeatFold<PI, TI, FI, FS, T>
where
    PI: Parser<'i, TI>,
    FI: Fn() -> T,
    FS: Fn(&mut T, TI) -> bool,
{
    fn parse(&self, input: Input<'i>) -> Option<(T, Input<'i>)> {
        let (first, mut input) = self.inner.parse(input.with_index(0))?;
        let mut result = (self.f_init)();
        let mut index = 1;

        if (self.f_step)(&mut result, first) {
            while let Some((inner_result, next)) = self.inner.parse(input.with_index(index)) {
                index += 1;
                input = next;

                if !(self.f_step)(&mut result, inner_result) {
                    break;
                }
            }
        }

        Some((result, input))
    }

    fn find_parsable(&self, input: Input<'i>) -> Option<(T, usize, Input<'i>)> {
        let (first, offset, mut input) = self.inner.find_parsable(input.with_index(0))?;
        let mut result = (self.f_init)();
        let mut index = 1;

        if (self.f_step)(&mut result, first) {
            while let Some((inner_result, next)) = self.inner.parse(input.with_index(index)) {
                index += 1;
                input = next;

                if !(self.f_step)(&mut result, inner_result) {
                    break;
                }
            }
        }

        Some((result, offset, input))
    }
}

pub struct RepeatFind<PI, TI, C> {
    inner: PI,
    spooky_ghost: PhantomData<(TI, C)>,
}

impl<PI, TI, C> RepeatFind<PI, TI, C> {
    #[inline]
    pub(crate) fn new(inner: PI) -> Self {
        Self {
            inner,
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, PI, TI, C> Parser<'i, C> for RepeatFind<PI, TI, C>
where
    C: GatherTarget<TI>,
    PI: Parser<'i, TI>,
{
    fn parse(&self, input: Input<'i>) -> Option<(C, Input<'i>)> {
        let (first, _, mut input) = self.inner.find_parsable(input.with_index(0))?;

        let mut res = C::init(1);
        if res.gather(0, first) {
            let mut index = 1;
            while let Some((v, _, next)) = self.inner.find_parsable(input.with_index(index)) {
                input = next;
                if !res.gather(index, v) {
                    break;
                }
                index += 1;
            }
        }

        Some((res, input))
    }

    #[inline]
    fn find_parsable(&self, input: Input<'i>) -> Option<(C, usize, Input<'i>)> {
        let (res, input) = self.parse(input)?;
        Some((res, 0, input))
    }
}

pub trait GatherTarget<T> {
    fn init(size_hint: usize) -> Self;
    fn gather(&mut self, index: usize, t: T) -> bool;
}

impl<T> GatherTarget<T> for Vec<T> {
    #[inline]
    fn init(size_hint: usize) -> Self {
        Self::with_capacity(size_hint)
    }

    #[inline]
    fn gather(&mut self, index: usize, t: T) -> bool {
        if self.len() == index {
            self.push(t);
        } else {
            self[index] = t;
        }

        true
    }
}

impl<const N: usize, T> GatherTarget<T> for [T; N]
where
    T: Copy + Default,
{
    #[inline]
    fn init(_: usize) -> Self {
        [T::default(); N]
    }

    #[inline]
    fn gather(&mut self, index: usize, t: T) -> bool {
        self[index] = t;
        index != N - 1
    }
}
