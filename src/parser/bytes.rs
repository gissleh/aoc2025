use super::{Input, Parser};

#[inline]
pub fn n_bytes<'i, const N: usize>() -> impl Parser<'i, [u8; N]> {
    NBytes
}

struct NBytes<const N: usize>;

impl<'i, const N: usize> Parser<'i, [u8; N]> for NBytes<N> {
    #[inline]
    fn parse(&self, input: Input<'i>) -> Option<([u8; N], Input<'i>)> {
        if input.data.len() >= N {
            let mut res = [0u8; N];
            res.copy_from_slice(&input.data[..N]);
            Some((res, input.advance(N)))
        } else {
            None
        }
    }

    #[inline]
    fn find_parsable(&self, input: Input<'i>) -> Option<([u8; N], usize, Input<'i>)> {
        match self.parse(input) {
            Some((res, next)) => Some((res, 0, next)),
            None => None,
        }
    }
}
