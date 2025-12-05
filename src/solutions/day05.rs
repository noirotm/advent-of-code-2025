use crate::parsing::ReadAll;
use crate::solver::Solver;
use anyhow::anyhow;
use sscanf::sscanf;
use std::io::BufRead;
use std::ops::RangeInclusive;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Ingredients;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: BufRead>(&self, r: R) -> anyhow::Result<Self::Input> {
        Ingredients::from_str(&r.read_all())
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input
            .ids
            .iter()
            .filter(|n| input.fresh_ranges.iter().any(|r| r.contains(n)))
            .count() as u64
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut ranges = input
            .fresh_ranges
            .iter()
            .cloned()
            .map(Some)
            .collect::<Vec<_>>();

        loop {
            let mut updated = false;

            'outer: for i in 0..ranges.len() {
                for j in i + 1..ranges.len() {
                    if let (Some(r1), Some(r2)) = (&ranges[i], &ranges[j]) {
                        // insert new one
                        if let Some(r) = overlapping_union(r1, r2) {
                            ranges.push(Some(r));
                            updated = true;

                            // delete old ones
                            ranges[i] = None;
                            ranges[j] = None;

                            // need to break from outer loop as total length has changed
                            break 'outer;
                        }
                    }
                }
            }

            if !updated {
                break;
            }

            ranges.retain(|r| r.is_some());
        }

        // compute all lengths
        ranges
            .iter()
            .flatten()
            .map(|r| r.end() - r.start() + 1)
            .sum()
    }
}

pub struct Ingredients {
    fresh_ranges: Vec<RangeInclusive<u64>>,
    ids: Vec<u64>,
}

impl FromStr for Ingredients {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut groups = s.split("\n\n");
        let fresh_ranges = groups
            .next()
            .ok_or(anyhow!("empty input"))?
            .split("\n")
            .flat_map(|s| sscanf!(s, "{}-{}", u64, u64))
            .map(|(x, y)| x..=y)
            .collect();
        let ids = groups
            .next()
            .ok_or(anyhow!("missing ids"))?
            .split("\n")
            .flat_map(u64::from_str)
            .collect();

        Ok(Self { fresh_ranges, ids })
    }
}

fn overlapping_union(
    r1: &RangeInclusive<u64>,
    r2: &RangeInclusive<u64>,
) -> Option<RangeInclusive<u64>> {
    // ranges can overlap, meaning that for each new range we see, it can be transformed
    // into a smaller range if it overlaps with a previous one.
    //
    // 1. equal ranges -> range 2 is discarded
    // a    b
    // c    d
    // 2. partial overlap -> we get a ..= d
    // a    b
    //    c     d
    // 3. no overlap -> we keep a..=b & c..=d
    // a    b
    //         c     d
    // 4. full inclusion -> we get a..=b
    // a          b
    //    c   d

    // same, reduce
    if r1 == r2 {
        return Some(r1.clone());
    }

    // returns a new range that excludes values from r2 found in r1
    // 1. make sure to use the 1st range (start1 < start2)
    let (range1, range2) = if r1.start() <= r2.start() {
        (r1, r2)
    } else {
        (r2, r1)
    };

    // total overlap, return enclosing
    if range1.start() <= range2.start() && range1.end() >= range2.end() {
        return Some(range1.clone());
    }

    // partial overlap, return a merge
    if range2.start() <= range1.end() {
        // merge
        return Some(*range1.start()..=*range2.end());
    }

    // disjoint, no overlap
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = r"3-5
10-14

1
5
8";
        let ingredients = Ingredients::from_str(input).unwrap();
        assert_eq!(ingredients.fresh_ranges, vec![3..=5, 10..=14]);
        assert_eq!(ingredients.ids, vec![1, 5, 8]);
    }

    #[test]
    fn test_overlap() {
        assert_eq!(overlapping_union(&(3..=5), &(4..=6)), Some(3..=6));
        assert_eq!(overlapping_union(&(3..=5), &(10..=12)), None);
        assert_eq!(overlapping_union(&(3..=5), &(4..=12)), Some(3..=12));
    }
}
