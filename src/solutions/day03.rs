use crate::parsing::BufReadExt;
use crate::solver::Solver;
use std::io::BufRead;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Bank>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: BufRead>(&self, r: R) -> anyhow::Result<Self::Input> {
        Ok(r.split_lines())
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input.iter().map(|b| b.largest_n(2)).sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input.iter().map(|b| b.largest_n(12)).sum()
    }
}

pub struct Bank {
    batteries: Vec<u8>,
}

impl Bank {
    fn largest_pair(&self) -> u8 {
        // find biggest in all but the last
        let (i, dozen) = self.batteries[0..self.batteries.len() - 1]
            .iter()
            .cloned()
            .enumerate()
            .rev()
            .max_by(|&(_, a), &(_, b)| a.cmp(&b))
            .unwrap_or((0, 0));

        // find largest in second part of the array
        let digit = self.batteries[i + 1..].iter().cloned().max().unwrap_or(0);

        dozen * 10 + digit
    }

    fn largest_n(&self, n: usize) -> u64 {
        let mut total = 0;
        let mut found = 0;
        let mut pos = 0;

        // find biggest in all but the last N-1
        while found < n {
            let (next_pos, joltage) = self.batteries[pos..self.batteries.len() - (n - found - 1)]
                .iter()
                .cloned()
                .enumerate()
                .rev()
                .max_by(|&(_, a), &(_, b)| a.cmp(&b))
                .unwrap_or((0, 0));
            total += joltage as u64 * 10u64.pow(n as u32 - found as u32 - 1);
            pos += next_pos + 1;
            found += 1;
        }

        total
    }
}

impl FromStr for Bank {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            batteries: s.bytes().map(|b| b - b'0').collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_pair() {
        assert_eq!(
            Bank::from_str("987654321111111").unwrap().largest_pair(),
            98
        );
        assert_eq!(
            Bank::from_str("811111111111119").unwrap().largest_pair(),
            89
        );
        assert_eq!(
            Bank::from_str("234234234234278").unwrap().largest_pair(),
            78
        );
        assert_eq!(
            Bank::from_str("818181911112111").unwrap().largest_pair(),
            92
        );
        assert_eq!(Bank::from_str("90997").unwrap().largest_pair(), 99);
        assert_eq!(Bank::from_str("5772447636633536424366261954729934453835645363432553634652753366355883885321733333657475668366474567").unwrap().largest_pair(), 99);
    }

    #[test]
    fn test_largest_n_2() {
        assert_eq!(Bank::from_str("987654321111111").unwrap().largest_n(2), 98);
        assert_eq!(Bank::from_str("811111111111119").unwrap().largest_n(2), 89);
        assert_eq!(Bank::from_str("234234234234278").unwrap().largest_n(2), 78);
        assert_eq!(Bank::from_str("818181911112111").unwrap().largest_n(2), 92);
        assert_eq!(Bank::from_str("90997").unwrap().largest_n(2), 99);
        assert_eq!(Bank::from_str("5772447636633536424366261954729934453835645363432553634652753366355883885321733333657475668366474567").unwrap().largest_n(2), 99);
    }

    #[test]
    fn test_largest_n() {
        assert_eq!(
            Bank::from_str("987654321111111").unwrap().largest_n(12),
            987654321111
        );
        assert_eq!(
            Bank::from_str("811111111111119").unwrap().largest_n(12),
            811111111119
        );
        assert_eq!(
            Bank::from_str("234234234234278").unwrap().largest_n(12),
            434234234278
        );
        assert_eq!(
            Bank::from_str("818181911112111").unwrap().largest_n(12),
            888911112111
        );
    }
}
