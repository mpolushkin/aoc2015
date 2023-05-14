mod cli;
mod challenges;

use clap::Parser;
use challenges::Challenges;

fn main() {
    let args = cli::Args::parse();
    let challenges = Challenges::new();
    challenges.print_solutions(args.day);
}
