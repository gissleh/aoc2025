use common::aoc::Runner;
use common::graph::SimpleGraph;
use common::parser;
use common::parser::Parser;

pub fn main(r: &mut Runner, input: &[u8]) {
    let graph = r.prep("Parse", || parse(input));

    r.part("Part 1", || part_1(&graph));
    r.part("Part 2", || part_2(&graph));
    r.set_tail("Part 1");
    r.part("Part 2 (Alt)", || part_2_alt(&graph));
}

fn part_1(graph: &SimpleGraph<[u8; 3]>) -> u32 {
    let you = graph.node(b"you").unwrap();
    let out = graph.node(b"out").unwrap();
    P1DP::new(graph.len()).run_path(you, out, graph)
}

fn part_2_alt(graph: &SimpleGraph<[u8; 3]>) -> u64 {
    let svr = graph.node(b"svr").unwrap();
    let out = graph.node(b"out").unwrap();
    let dac = graph.node(b"dac").unwrap();
    let fft = graph.node(b"fft").unwrap();

    let mut dp = P1DP::new(graph.len());
    let dac_out = dp.run_path(dac, out, &graph) as u64;
    let fft_dac = dp.run_path(fft, dac, &graph) as u64;
    let svr_fft = dp.run_path(svr, fft, &graph) as u64;
    let fft_out = dp.run_path(fft, out, &graph) as u64;
    let dac_fft = dp.run_path(dac, fft, &graph) as u64;
    let svr_dac = dp.run_path(svr, dac, &graph) as u64;

    (svr_fft * fft_dac * dac_out) + (svr_dac * dac_fft * fft_out)
}

fn part_2(graph: &SimpleGraph<[u8; 3]>) -> u64 {
    let svr = graph.node(b"svr").unwrap();
    let out = graph.node(b"out").unwrap();
    let dac = graph.node(b"dac").unwrap();
    let fft = graph.node(b"fft").unwrap();
    P2DP::new(graph.len(), out, dac, fft).run(svr, 0, graph)
}

struct P1DP {
    out: u16,
    map: Vec<Option<u32>>,
}

impl P1DP {
    #[inline]
    fn new(len: usize) -> Self {
        Self {
            out: 0,
            map: vec![None; len],
        }
    }

    #[inline]
    fn run_path(&mut self, src: u16, out: u16, graph: &SimpleGraph<[u8; 3]>) -> u32 {
        self.out = out;
        self.map.fill(None);
        self.run(src, graph)
    }

    fn run(&mut self, curr: u16, graph: &SimpleGraph<[u8; 3]>) -> u32 {
        if let Some(cached) = self.map[curr as usize] {
            cached
        } else {
            if curr == self.out {
                1
            } else {
                let mut sum = 0;
                for e in graph.edges(curr) {
                    sum += self.run(e, graph);
                }

                self.map[curr as usize] = Some(sum);
                sum
            }
        }
    }
}

struct P2DP {
    out: u16,
    dac: u16,
    fft: u16,
    len: usize,
    map: Vec<Option<u64>>,
}

impl P2DP {
    #[inline]
    pub fn new(len: usize, out: u16, dac: u16, fft: u16) -> Self {
        Self {
            out,
            dac,
            fft,
            len,
            map: vec![None; len * 4],
        }
    }

    fn run(&mut self, curr: u16, mut seen_flags: u8, graph: &SimpleGraph<[u8; 3]>) -> u64 {
        let map_key = seen_flags as usize * self.len + curr as usize;

        if let Some(res) = self.map[map_key] {
            res
        } else {
            if curr == self.out {
                if seen_flags == 0b11 { 1 } else { 0 }
            } else {
                if curr == self.dac {
                    seen_flags |= 1;
                }
                if curr == self.fft {
                    seen_flags |= 2;
                }

                let mut sum = 0;
                for e in graph.edges(curr) {
                    sum += self.run(e, seen_flags, graph);
                }

                self.map[map_key] = Some(sum);
                sum
            }
        }
    }
}

fn parse(input: &[u8]) -> SimpleGraph<[u8; 3]> {
    parser::n_bytes::<3>()
        .and_skip(b": ")
        .and(
            parser::n_bytes::<3>()
                .delimited_by(b' ')
                .repeat::<[[u8; 3]; 32]>(),
        )
        .delimited_by(b'\n')
        .repeat_fold(
            || SimpleGraph::<[u8; 3]>::with_capacity(128),
            |graph, (curr, nexts)| {
                let ci = graph.insert(curr);
                for next in nexts {
                    if next == [0u8; 3] {
                        break;
                    }

                    let ni = graph.insert(next);
                    graph.connect(ci, ni);
                }

                true
            },
        )
        .run(input)
        .unwrap()
}
