use crate::parsing::BufReadExt;
use crate::solver::Solver;
use anyhow::anyhow;
use sscanf::sscanf;
use std::io::BufRead;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Range>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: BufRead>(&self, r: R) -> anyhow::Result<Self::Input> {
        Ok(r.split_commas::<Vec<Range>>())
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut total = 0;

        for r in input {
            for n in r.start..=r.end {
                if is_id_invalid(n) {
                    total += n;
                }
            }
        }

        total
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut total = 0;

        for r in input {
            for n in r.start..=r.end {
                if is_id_really_invalid2(n) {
                    total += n;
                }
            }
        }

        total
    }
}

pub struct Range {
    start: u64,
    end: u64,
}

impl FromStr for Range {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = sscanf!(s, "{}-{}", u64, u64).map_err(|e| anyhow!("{e}"))?;
        Ok(Self { start, end })
    }
}

fn is_id_invalid(id: u64) -> bool {
    // convert to string
    // if len is odd, id is valid
    // else, double iteration from left and middle and return true at first difference
    let s = id.to_string();
    let b = s.as_bytes();

    if !b.len().is_multiple_of(2) {
        return false;
    }

    let mid = b.len() / 2;
    (0..mid).all(|i| b[i] == b[i + mid])
}

fn is_id_really_invalid(id: u64) -> bool {
    let s = id.to_string();
    let b = s.as_bytes();

    // for each possible cut point (1 to n/2), if b is a multiple of n, create n chunks, and
    // see if they're all the same. Once we have a positive return true, else return false
    (1..=b.len() / 2)
        .filter(|i| b.len().is_multiple_of(*i))
        .any(|i| {
            b.chunks(i)
                .collect::<Vec<_>>()
                .windows(2)
                .all(|w| w[0] == w[1])
        })
}

fn is_id_really_invalid2(id: u64) -> bool {
    /*
    111111 / 10^5 + 10^4 + 10^3 + 10^2 + 10^1 + 10^0 = 1
    111111 % 10^1 = 1

    121212 / 10^4 + 10^2 + 10^0 = 12
    121212 % 10^2 = 12

    123123 / 10^3 + 10^0 = 123 rem = 0
    123123 % 10^3 = 123
    */

    let digits = id.ilog10();
    for n in 1..=digits.div_ceil(2) {
        // compute divisor
        let divisor = (0..=digits)
            .step_by(n as usize)
            .take_while(|&v| v <= digits)
            .map(|v| 10u64.pow(v))
            .sum::<u64>();

        // divide and get remainder
        let (q, r) = (id / divisor, id % divisor);

        // if remainder not zero, not divisible, so not repeated
        if r != 0 {
            continue;
        }

        // isolate N last digits which should be equal to quotient if invalid
        let last = id % 10u64.pow(n);

        // make sure we have the right number of digits
        /*
        expecting 2 digits
        10101 / 10^4 + 10^2 + 10^0 = 1
        10101 % 10^2 = 1
        only got 1
         */
        if (last.ilog10() + 1) != n {
            continue;
        }

        if last == q {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_id_invalid() {
        assert!(is_id_invalid(111111));
        assert!(is_id_invalid(1111));
        assert!(!is_id_invalid(1211));
        assert!(is_id_invalid(1212));
        assert!(!is_id_invalid(1));
        assert!(!is_id_invalid(12));
    }

    #[test]
    fn test_is_id_really_invalid() {
        assert!(is_id_really_invalid(111111));
        assert!(is_id_really_invalid(1111111));
        assert!(is_id_really_invalid(121212));
        assert!(is_id_really_invalid(123123123));
        assert!(!is_id_really_invalid(1231231234));
        assert!(is_id_really_invalid(1111));
        assert!(!is_id_really_invalid(1211));
        assert!(!is_id_really_invalid(1));
        assert!(!is_id_really_invalid(12));
        assert!(is_id_really_invalid(122122));
    }

    #[test]
    fn test_is_id_really_invalid2() {
        assert!(is_id_really_invalid2(111111));
        assert!(is_id_really_invalid2(1111111));
        assert!(is_id_really_invalid2(121212));
        assert!(is_id_really_invalid2(123123123));
        assert!(!is_id_really_invalid2(1231231234));
        assert!(is_id_really_invalid2(1111));
        assert!(!is_id_really_invalid2(1211));
        assert!(!is_id_really_invalid2(1));
        assert!(!is_id_really_invalid2(12));
        assert!(is_id_really_invalid2(122122));
        assert!(!is_id_really_invalid2(10101));
    }
}
