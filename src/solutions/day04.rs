use crate::grid::Grid;
use crate::solver::Solver;
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
        removable_coords(input).len()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut g = input.clone();
        let mut removed = 0;
        loop {
            let to_remove = removable_coords(&g);
            if to_remove.is_empty() {
                break;
            }

            removed += to_remove.len();
            for c in to_remove {
                if let Some(e) = g.get_mut(c) {
                    *e = Entry::Empty;
                }
            }
        }

        removed
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Entry {
    Empty,
    Roll,
}

impl TryFrom<u8> for Entry {
    type Error = anyhow::Error;

    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            b'.' => Ok(Entry::Empty),
            b'@' => Ok(Entry::Roll),
            _ => Err(anyhow::anyhow!("invalid entry")),
        }
    }
}

fn removable_coords(g: &Grid<Entry>) -> Vec<(usize, usize)> {
    g.iter_with_coords()
        .filter(|&(_, e)| e.eq(&Entry::Roll))
        .filter(|&(c, _)| {
            g.neighbours_coords8(c)
                .iter()
                .filter(|&c| g.get(c).eq(&Some(&Entry::Roll)))
                .count()
                < 4
        })
        .map(|(c, _)| c)
        .collect()
}
