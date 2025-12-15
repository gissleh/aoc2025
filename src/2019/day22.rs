use common::aoc::Runner;
use common::parser;
use common::parser::Parser;

pub fn main(r: &mut Runner, input: &[u8]) {
    let techniques = r.prep("Parse", || parse(input));

    r.part("Part 1", || part_1(&techniques));
    r.mark_dead_end("Part 1");
    r.set_tail("Parse");

    r.part("Part 1 (Fast)", || part_1_math(&techniques));

    r.info("Techniques", techniques.len());
}

fn part_1(techniques: &[Technique]) -> usize {
    let mut deck = Deck::<10007>::new();
    for technique in techniques {
        deck = deck.apply_technique(technique);
    }

    deck.position_of(2019)
}

fn part_1_math(techniques: &[Technique]) -> u64 {
    let mut v = 2019;
    for technique in techniques {
        v = technique.move_position::<10007>(v);
    }

    v
}

fn parse(input: &[u8]) -> Vec<Technique> {
    Technique::parser()
        .delimited_by(b'\n')
        .repeat()
        .run(input)
        .unwrap()
}

#[derive(Eq, PartialEq, Debug)]
struct Deck<const N: usize>([u16; N]);

impl<const N: usize> Deck<N> {
    #[inline]
    fn new() -> Self {
        let mut res = [0u16; N];
        for v in 0..N {
            res[v] = v as u16
        }
        Self(res)
    }

    #[inline]
    fn position_of(&self, p0: u16) -> usize {
        self.0.iter().position(|v| *v == p0).unwrap()
    }

    #[inline]
    fn apply_technique(mut self, technique: &Technique) -> Self {
        match technique {
            Technique::DealIntoNewStack => self.0.reverse(),
            Technique::CutCards(n) => {
                if *n < 0 {
                    self.0.rotate_right(-n as usize);
                } else {
                    self.0.rotate_left(*n as usize);
                }
            }
            Technique::DealWithIncrement(n) => {
                let mut new_arr = [0; N];
                let n = *n as usize;
                for i in 0..N {
                    new_arr[(i * n) % N] = self.0[i];
                }
                self.0 = new_arr;
            }
        }

        self
    }
}

enum Technique {
    DealIntoNewStack,
    CutCards(i64),
    DealWithIncrement(u64),
}

impl Technique {
    #[inline]
    fn move_position<const N: u64>(&self, n: u64) -> u64 {
        match self {
            Self::DealIntoNewStack => (N - n) - 1,
            Self::CutCards(c) => {
                if *c > 0 {
                    (n + N - (*c as u64)) % N
                } else {
                    (n + -c as u64) % N
                }
            }
            Self::DealWithIncrement(c) => (n * *c) % N,
        }
    }

    #[inline]
    fn parser<'i>() -> impl Parser<'i, Self> {
        b"cut "
            .and_instead(parser::int())
            .map(|n| Technique::CutCards(n))
            .or(b"deal with increment "
                .and_instead(parser::uint().map(|v| Technique::DealWithIncrement(v))))
            .or(b"deal into new stack".map(|_| Technique::DealIntoNewStack))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_technique_works_on_deck() {
        assert_eq!(
            Deck([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]).apply_technique(&Technique::CutCards(3)),
            Deck([3, 4, 5, 6, 7, 8, 9, 0, 1, 2])
        );
        assert_eq!(
            Deck([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]).apply_technique(&Technique::CutCards(-4)),
            Deck([6, 7, 8, 9, 0, 1, 2, 3, 4, 5])
        );
        assert_eq!(
            Deck([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]).apply_technique(&Technique::DealIntoNewStack),
            Deck([9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
        );
        assert_eq!(
            Deck([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]).apply_technique(&Technique::DealWithIncrement(3)),
            Deck([0, 7, 4, 1, 8, 5, 2, 9, 6, 3])
        );
    }
}
