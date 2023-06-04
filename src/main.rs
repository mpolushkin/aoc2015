#![allow(dead_code)]

mod challenges;
mod cli;

use challenges::Challenges;
use clap::Parser;

fn main() {
    let args = cli::Args::parse();
    let challenges = Challenges::new();
    challenges.print_solutions(args.day);
}
