mod cli;
mod challenges;
mod challenges_old;

use clap::Parser;
use challenges::Challenges;

fn main() {
    let args = cli::Args::parse();
    let challenges = Challenges::new();
    challenges.print_solutions(args.day);
}
