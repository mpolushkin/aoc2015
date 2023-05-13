use clap::Parser;

/// Michael's solutions for Advent of Code 2015
#[derive(Parser, Debug)]
pub struct Args {
    /// Which day to solve (1-25)
    pub day: u8,
}
