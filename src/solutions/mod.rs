use crate::solver::{ProblemOutput, Solver};
mod day01;
mod day02;
pub fn exec_day(day: u32) -> Option<ProblemOutput> {
    match day {
        1u32 => Some(day01::Problem.solve(day)),
        2u32 => Some(day02::Problem.solve(day)),
        _ => None,
    }
}
pub fn exec_all_days() -> Vec<ProblemOutput> {
    vec![day01::Problem.solve(1u32), day02::Problem.solve(2u32)]
}
