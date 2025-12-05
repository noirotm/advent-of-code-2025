use crate::solver::{ProblemOutput, Solver};
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
pub fn exec_day(day: u32) -> Option<ProblemOutput> {
    match day {
        1u32 => Some(day01::Problem.solve(day)),
        2u32 => Some(day02::Problem.solve(day)),
        3u32 => Some(day03::Problem.solve(day)),
        4u32 => Some(day04::Problem.solve(day)),
        5u32 => Some(day05::Problem.solve(day)),
        _ => None,
    }
}
pub fn exec_all_days() -> Vec<ProblemOutput> {
    vec![
        day01::Problem.solve(1u32), day02::Problem.solve(2u32), day03::Problem
        .solve(3u32), day04::Problem.solve(4u32), day05::Problem.solve(5u32)
    ]
}
