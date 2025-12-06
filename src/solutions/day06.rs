use crate::grid::Grid;
use crate::solver::Solver;
use anyhow::anyhow;
use std::fmt::Display;
use std::io::BufRead;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<u8>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: BufRead>(&self, mut r: R) -> anyhow::Result<Self::Input> {
        let mut v = vec![];
        let _ = r.read_to_end(&mut v)?;
        Ok(v)
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let g = Grid::from_split_whitespace_reader(input.as_slice()).expect("valid grid");
        (0..g.w).map(|c| eval_col(&g, c)).sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let g: Grid<u8> = Grid::from_reader(input.as_slice()).expect("valid grid");

        // divide into groups of columns separated by spaces, sum all groups
        let rows = (0..g.w).collect::<Vec<_>>();
        rows.split(|&c| g.iter_col(c).all(|&b| b == b' '))
            .map(|cols| eval_col2(&g, cols))
            .sum()
    }
}

pub enum Entry {
    Add,
    Multiply,
    Value(u64),
}

impl FromStr for Entry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Entry::Add),
            "*" => Ok(Entry::Multiply),
            s => s
                .parse::<u64>()
                .map_err(|_| anyhow!("invalid entry"))
                .map(Entry::Value),
        }
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ",
            match self {
                Entry::Add => "+".to_string(),
                Entry::Multiply => "*".to_string(),
                Entry::Value(v) => v.to_string(),
            }
        )
    }
}

fn eval_col(g: &Grid<Entry>, col: usize) -> u64 {
    let mut it = g.iter_col(col).rev();
    let op = it.next();
    let it = it.filter_map(|e| match e {
        Entry::Value(v) => Some(*v),
        _ => None,
    });

    match op {
        Some(Entry::Add) => it.sum(),
        Some(Entry::Multiply) => it.product(),
        _ => 0,
    }
}

fn eval_col2(g: &Grid<u8>, cols: &[usize]) -> u64 {
    let mut op = None;
    let mut numbers = vec![];

    for c in cols {
        let mut accum = 0u64;
        for &b in g.iter_col(*c) {
            match b {
                b'0'..=b'9' => {
                    let n = b - b'0';
                    accum = accum * 10 + n as u64;
                }
                b'+' | b'*' => op = Some(b),
                _ => {}
            }
        }
        numbers.push(accum);
    }

    match op {
        Some(b'+') => numbers.iter().sum(),
        Some(b'*') => numbers.iter().product(),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::Grid;

    #[test]
    fn test_parse() {
        let s = r"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +"
            .as_bytes();
        let g: Grid<Entry> = Grid::from_split_whitespace_reader(s).unwrap();

        println!("{g}");
    }
}
