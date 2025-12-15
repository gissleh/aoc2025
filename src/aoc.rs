use crate::graph::Graph;
use crate::search::{Dijkstra, KCS, Search};
use chrono::Datelike;
use hashbrown::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::fs::{File, OpenOptions, create_dir_all};
use std::io::{Read, Write};
use std::iter::Sum;
use std::ops::Add;
use std::time::Instant;

pub fn run(year: u16, day: u16, cb: fn(&mut Runner, input: &[u8])) {
    let args: Vec<String> = std::env::args().collect();
    let selected_day = args
        .get(2)
        .map(|v| v.parse::<u32>().unwrap())
        .or(Some(chrono::Local::now().day()))
        .expect("Day number not provided") as u16;

    if selected_day == day || selected_day == 0 {
        let op = args.get(1).cloned().or(Some(String::from("once"))).unwrap();

        let mut runner = Runner::new(
            day,
            RunnerOptions {
                once: op == "run" || op == "bench_once",
                bench: op == "bench" || op == "bench_once",
            },
        );

        let input_data = load_input(year, day);

        cb(&mut runner, input_data.as_slice());
        runner.print();
    }
}

pub struct Runner {
    graph: Graph<&'static str, (), (&'static str, String, i64)>,
    roots: Vec<usize>,
    infos: Vec<(&'static str, String)>,
    day: u16,
    tail: usize,
    options: RunnerOptions,
}

impl Runner {
    fn new(day: u16, options: RunnerOptions) -> Self {
        Self {
            graph: Graph::with_capacity(16, 4),
            tail: usize::MAX,
            infos: Vec::new(),
            roots: Vec::new(),
            day,
            options,
        }
    }

    pub fn info<T>(&mut self, name: &'static str, value: T)
    where
        T: Display,
    {
        self.infos.push((name, format!("{value}")));
    }

    pub fn info_debug<T>(&mut self, name: &'static str, value: T)
    where
        T: Debug,
    {
        self.infos.push((name, format!("{value:?}")));
    }

    pub fn set_tail(&mut self, key: &'static str) {
        self.tail = self.graph.node(&key).unwrap();
    }

    pub fn clear_tail(&mut self) {
        self.tail = usize::MAX;
    }

    pub fn prep<T, F>(&mut self, name: &'static str, f: F) -> T
    where
        F: Fn() -> T,
    {
        let (v, _) = self.run(name, f);
        v
    }

    pub fn part<T, F>(&mut self, name: &'static str, f: F) -> T
    where
        F: Fn() -> T,
        T: Display,
    {
        let (v, index) = self.run(name, f);
        self.graph.node_value_mut(index).1 = format!("{}", v).to_string();
        v
    }

    pub fn mark_dead_end(&mut self, src: &'static str) {
        self.link(src, src);
    }

    pub fn link(&mut self, src: &'static str, dst: &'static str) {
        self.graph.connect(
            self.graph.node(&src).unwrap(),
            self.graph.node(&dst).unwrap(),
            (),
        )
    }

    fn run<T, F>(&mut self, name: &'static str, f: F) -> (T, usize)
    where
        F: Fn() -> T,
    {
        let before = Instant::now();
        let mut res = f();
        let after = Instant::now();
        let mut dur = after - before;

        let mut runs = 1;
        if self.options.bench && !self.options.once {
            runs = match dur.as_millis() {
                0 => 2500,
                1 => 1000,
                2..=4 => 500,
                5..=9 => 100,
                10..=19 => 50,
                20..=49 => 20,
                50..=299 => 10,
                _ => 1,
            };

            if runs > 1 {
                let before = Instant::now();
                for _ in 1..runs {
                    res = f();
                }
                let after = Instant::now();
                dur += after - before;
            }
        }

        let dur = dur.as_nanos() as i64 / runs as i64;
        let index = self.graph.insert(name, (name, String::new(), dur));
        if self.tail != usize::MAX {
            self.graph.connect(self.tail, index, ());
        } else {
            self.roots.push(index);
        }
        self.tail = index;

        (res, index)
    }

    pub fn print(&self) {
        println!("--- Day {} ---", self.day);
        println!("Results:");
        for (name, res, _) in self.graph.values() {
            if res.is_empty() {
                continue;
            }

            println!("  {}: {}", name, res);
        }
        if !self.infos.is_empty() {
            println!();
            println!("Info:");
            for (key, value) in self.infos.iter() {
                if value.contains("\n") {
                    let value2 = value
                        .trim_end_matches(|v| v == '\n')
                        .replace("\n", "\n    ");
                    println!("  {}: \n    {}", key, value2);
                } else {
                    println!("  {}: {}", key, value);
                }
            }
        }
        println!();
        println!("Times:");
        for (name, _, duration) in self.graph.values() {
            println!("  {}: {}", name, format_duration(*duration));
        }

        if let Some((shortest, path)) = self.shortest_time() {
            println!();

            if path.len() != self.graph.len() {
                println!("Total ({:?}): {}", path, format_duration(shortest));
            } else {
                println!("Total: {}", format_duration(shortest));
            }
        }
    }

    pub fn shortest_time(&self) -> Option<(i64, Vec<&'static str>)> {
        let mut s = Dijkstra::<_, _, KCS<usize, i64, Vec<&'static str>>, _>::new(HashMap::new());

        for root in self.roots.iter() {
            let (name, _, ns) = self.graph.node_value(*root);
            s.push(KCS(*root, *ns, vec![name]));
        }

        while let Some(KCS(index, ns, path)) = s.pop() {
            let mut has_edges = false;
            for (next, _) in self.graph.edges(index) {
                let (next_name, _, next_ns) = self.graph.node_value(next);

                let mut path = path.clone();
                path.push(next_name);

                s.push(KCS(next, ns + *next_ns, path));
                has_edges = true;
            }

            if !has_edges {
                return Some((ns, path));
            }
        }

        None
    }
}

struct RunnerOptions {
    once: bool,
    bench: bool,
}

pub fn format_duration(ns: i64) -> String {
    if ns == i64::MAX {
        return "-".to_string();
    }

    if ns > 10_000_000_000 {
        format!("{:.1}s", (ns as f64) / (1_000_000_000f64))
    } else if ns > 1_000_000_000 {
        format!("{:.2}s", (ns as f64) / (1_000_000_000f64))
    } else if ns > 1_000_000 {
        format!("{:.2}ms", (ns as f64) / (1_000_000f64))
    } else if ns > 1_000 {
        format!("{:.2}µs", (ns as f64) / (1_000f64))
    } else {
        format!("{}ns", ns)
    }
}

pub struct WithExtra<TD, TX>(pub TD, pub TX);

impl<TD, TX> Display for WithExtra<TD, TX>
where
    TD: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (+data for next part)", self.0)
    }
}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct BothParts<T1, T2>(pub T1, pub T2)
where
    T1: Display,
    T2: Display;

impl<T1, T2> Sum<BothParts<T1, T2>> for BothParts<T1, T2>
where
    T1: Display,
    T2: Display,
    T1: Add<Output = T1> + Default + Copy,
    T2: Add<Output = T2> + Default + Copy,
{
    fn sum<I: Iterator<Item = BothParts<T1, T2>>>(iter: I) -> Self {
        iter.fold(Self(Default::default(), Default::default()), |a, b| {
            BothParts(a.0 + b.0, a.1 + b.1)
        })
    }
}

impl<T1, T2> Display for BothParts<T1, T2>
where
    T1: Display,
    T2: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.0, self.1)
    }
}

pub fn load_input(year: u16, day_number: u16) -> Vec<u8> {
    let mut buf = Vec::with_capacity(2048);
    let file_name = format!(
        "/tmp/advent-of-code-{}-day-{:02}-input.txt",
        year, day_number
    );
    match File::open(file_name.clone()) {
        Ok(mut file) => {
            file.read_to_end(&mut buf).expect("Could not read file");
            buf.to_vec()
        }
        Err(_) => {
            let token = option_env!("AOC_SESSION").expect("token not found");
            if token == "" {
                panic!("Env is not set")
            }

            eprintln!("Downloading input for day {}...", day_number);

            create_dir_all(format!("./input/{}", year)).expect("Could not create dir");
            let data = reqwest::blocking::Client::builder()
                .build()
                .unwrap()
                .get(format!(
                    "https://adventofcode.com/{}/day/{}/input",
                    year, day_number
                ))
                .header(
                    "User-Agent",
                    "AOC Runner (github.com/gissleh/aoc2025, by dev@gisle.me)",
                )
                .header("Authority", "adventofcode.com")
                .header("Cookie", format!("session={}", token))
                .send()
                .expect("failed to send request")
                .bytes()
                .expect("could not read file");

            buf.extend(data.iter());

            eprintln!("Saving input as {file_name}");

            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(file_name)
                .expect("Could not open file");
            file.write_all(&buf).expect("Could not write file");

            data.to_vec()
        }
    }
}
