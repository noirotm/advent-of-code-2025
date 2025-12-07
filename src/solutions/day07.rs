use crate::grid::{Coord, Grid};
use crate::solver::Solver;
use std::collections::{BTreeMap, BTreeSet};
use std::io::BufRead;

pub struct Problem;

impl Solver for Problem {
    type Input = Grid<Entry>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: BufRead>(&self, r: R) -> anyhow::Result<Self::Input> {
        Grid::from_reader(r)
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let (start, _) = input
            .iter_with_coords()
            .find(|&(_, e)| e.eq(&Entry::Start))
            .expect("No start");
        let mut tachyons = BTreeSet::from([start]);
        let mut splits = 0;

        for _ in start.y()..input.h {
            let mut next_tachyons = BTreeSet::new();
            for &(x, y) in tachyons.iter() {
                if let Some(Entry::Splitter) = input.get((x, y + 1)) {
                    next_tachyons.insert((x - 1, y + 1));
                    next_tachyons.insert((x + 1, y + 1));
                    splits += 1;
                } else {
                    next_tachyons.insert((x, y + 1));
                }
            }
            tachyons = next_tachyons;
        }

        splits
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let (start, _) = input
            .iter_with_coords()
            .find(|&(_, e)| e.eq(&Entry::Start))
            .expect("No start");
        let mut tachyons = BTreeMap::from([(start, 1)]);

        for _ in start.y()..input.h {
            let mut next_tachyons = BTreeMap::new();
            for (&(x, y), &n) in tachyons.iter() {
                if let Some(Entry::Splitter) = input.get((x, y + 1)) {
                    next_tachyons
                        .entry((x - 1, y + 1))
                        .and_modify(|e| *e += n)
                        .or_insert(n);
                    next_tachyons
                        .entry((x + 1, y + 1))
                        .and_modify(|e| *e += n)
                        .or_insert(n);
                } else {
                    next_tachyons
                        .entry((x, y + 1))
                        .and_modify(|e| *e += n)
                        .or_insert(n);
                }
            }
            tachyons = next_tachyons;
        }

        tachyons.values().sum()
    }
}

#[derive(Eq, PartialEq)]
pub enum Entry {
    Start,
    Empty,
    Splitter,
}

impl TryFrom<u8> for Entry {
    type Error = anyhow::Error;

    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            b'S' => Ok(Entry::Start),
            b'.' => Ok(Entry::Empty),
            b'^' => Ok(Entry::Splitter),
            _ => Err(anyhow::anyhow!("Invalid entry {b}")),
        }
    }
}
