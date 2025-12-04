mod number;

pub use number::{base10_digit, base16_digit, uint, uint_hex};

#[derive(Clone, Copy)]
pub struct Input<'i> {
    pub data: &'i [u8],
    pub index: usize,
}

impl<'i> Input<'i> {
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn from(data: &'i [u8]) -> Self {
        Self { data, index: 0 }
    }

    pub fn advance(&self, offset: usize) -> Self {
        #[cfg(debug_assertions)]
        assert!(offset < self.data.len());

        Self {
            data: &self.data[offset..],
            index: self.index,
        }
    }

    pub fn advance_index(&self, data_offset: usize, index_offset: usize) -> Self {
        #[cfg(debug_assertions)]
        assert!(data_offset < self.data.len());

        Self {
            data: &self.data[data_offset..],
            index: self.index + index_offset,
        }
    }
}

pub trait Parser<'i, T> {
    fn parse(&self, input: Input<'i>) -> Option<(T, Input<'i>)>;

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
}

impl<'i> Parser<'i, u8> for u8 {
    fn parse(&self, input: Input<'i>) -> Option<(u8, Input<'i>)> {
        if input.data.get(0) == Some(self) {
            Some((*self, input.advance(1)))
        } else {
            None
        }
    }

    fn find_parsable(&self, input: Input<'i>) -> Option<(u8, usize, Input<'i>)> {
        input
            .data
            .iter()
            .position(|v| v.eq(self))
            .map(|i| (*self, i, input.advance(i + 1)))
    }
}

impl<'i, 'p> Parser<'i, &'p [u8]> for &'p [u8] {
    fn parse(&self, input: Input<'i>) -> Option<(&'p [u8], Input<'i>)> {
        if input.data.starts_with(self) {
            Some((self, input.advance(self.len())))
        } else {
            None
        }
    }

    fn find_parsable(&self, input: Input<'i>) -> Option<(&'p [u8], usize, Input<'i>)> {
        if input.data.len() < self.len() {
            return None;
        }

        (0..input.data.len() - self.len())
            .find(|i| input.data[*i..].starts_with(self))
            .map(|i| (*self, i, input.advance(i + self.len())))
    }
}
