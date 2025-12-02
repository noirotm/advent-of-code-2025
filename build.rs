use quote::{format_ident, quote};
use std::error::Error;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::{fs, io};

fn days(input_dir: &str) -> io::Result<Vec<u32>> {
    let mut days = read_dir(input_dir)?
        .flatten()
        .filter(|e| e.path().is_file())
        .flat_map(|e| e.file_name().into_string())
        .flat_map(|s| s[0..2].parse::<u32>())
        .collect::<Vec<_>>();
    days.sort_unstable();
    Ok(days)
}

fn gen_solutions_mod<P: AsRef<Path>>(p: P, days: &[u32]) -> io::Result<()> {
    let day_strings = days
        .iter()
        .map(|d| format!("{0:02}", d))
        .collect::<Vec<_>>();
    let mods = day_strings
        .iter()
        .map(|s| format_ident!("day{}", s))
        .collect::<Vec<_>>();

    let tokens = quote! {
        use crate::solver::{ProblemOutput, Solver};

        #(mod #mods;)*

        pub fn exec_day(day: u32) -> Option<ProblemOutput> {
            match day {
                #(#days => Some(#mods::Problem.solve(day)),)*
                _ => None,
            }
        }

        pub fn exec_all_days() -> Vec<ProblemOutput> {
            vec![#(#mods::Problem.solve(#days)),*]
        }
    };
    let syntax_tree = syn::parse2(tokens).expect("valid token stream");
    let pretty = prettyplease::unparse(&syntax_tree);

    fs::write(p, pretty)
}

fn gen_solutions(dir: &str, days: &[u32]) -> io::Result<()> {
    let token_stream = quote! {
        use crate::solver::Solver;
        use std::io::BufRead;

        pub struct Problem;

        impl Solver for Problem {
            type Input = ();
            type Output1 = u64;
            type Output2 = u64;

            fn parse_input<R: BufRead>(&self, r: R) -> anyhow::Result<Self::Input> {
                todo!()
            }

            fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
                todo!()
            }

            fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
                todo!()
            }
        }
    };
    let syntax_tree = syn::parse2(token_stream).expect("valid token stream");
    let pretty = prettyplease::unparse(&syntax_tree);

    for day in days {
        let file = PathBuf::from(format!("{}/day{:02}.rs", dir, day));
        if file.exists() {
            continue;
        }

        fs::write(file, &pretty)?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_dir = "./input";
    let output_dir = "./src/solutions";
    let solutions_mod_output_path = Path::new(&output_dir).join("mod.rs");

    let days = days(input_dir)?;

    // write solutions mod file
    gen_solutions_mod(solutions_mod_output_path, &days)?;

    // write solutions
    gen_solutions(output_dir, &days)?;

    Ok(())
}
