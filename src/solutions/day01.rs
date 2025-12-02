use crate::parsing::BufReadExt;
use crate::solver::Solver;
use anyhow::anyhow;
use sscanf::sscanf;
use std::error::Error;
use std::io::BufRead;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Instruction>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: BufRead>(&self, r: R) -> anyhow::Result<Self::Input> {
        Ok(r.split_lines())
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut current = 50;
        let mut password = 0;

        for i in input.iter() {
            current = i.apply(current);
            if current == 0 {
                password += 1;
            }
        }

        password
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut current = 50;
        let mut password = 0;

        for i in input.iter() {
            let (pos, clicks) = i.apply_with_clicks(current);
            password += clicks;
            current = pos;
        }

        password
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Instruction {
    dir: Direction,
    distance: u16,
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (d, n) = sscanf!(s, "{}{}", char, u16)?;
        let dir = match d {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => {
                return Err(anyhow!("Invalid direction: {}", d).into());
            }
        };

        Ok(Instruction { dir, distance: n })
    }
}

impl Instruction {
    fn apply(&self, pos: u8) -> u8 {
        // rotate right adds to the value, left substracts, wraps at 0 and 99
        let v = pos as i32;
        let d = self.distance as i32;
        let d = match self.dir {
            Direction::Left => -d,
            Direction::Right => d,
        };
        let mut v = v + d;
        while v < 0 {
            v += 100;
        }
        while v > 99 {
            v -= 100;
        }

        v as u8
    }

    fn apply_with_clicks(&self, pos: u8) -> (u8, usize) {
        let mut pos = pos;
        let mut clicks = 0;

        for _ in 0..self.distance {
            pos = self.apply_one(pos);
            if pos == 0 {
                clicks += 1;
            }
        }

        (pos, clicks)
    }

    fn apply_one(&self, pos: u8) -> u8 {
        let v = pos as i32;
        let d = match self.dir {
            Direction::Left => -1,
            Direction::Right => 1,
        };
        match v + d {
            100 => 0,
            -1 => 99,
            res => res as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply() {
        let i = Instruction::from_str("L2").unwrap();
        assert_eq!(i.apply(0), 98);

        let i = Instruction::from_str("R8").unwrap();
        assert_eq!(i.apply(11), 19);

        let i = Instruction::from_str("L10").unwrap();
        assert_eq!(i.apply(5), 95);
    }
}
