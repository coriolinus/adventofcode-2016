use day11::{part1, part2};

use color_eyre::eyre::Result;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct RunArgs {
    // normally, we allow users to select an input file at the CLI, but not here: turns out that
    // parsing this weird, human-readable format is annoying enough that I'm just going to
    // accept manually entering the input
    //
    // See https://github.com/coriolinus/adventofcode-2016/pull/2

    /// skip part 1
    #[structopt(long)]
    no_part1: bool,

    /// run part 2
    #[structopt(long)]
    part2: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = RunArgs::from_args();

    if !args.no_part1 {
        part1()?;
    }
    if args.part2 {
        part2()?;
    }
    Ok(())
}
