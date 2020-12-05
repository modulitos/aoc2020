use modulitos_2020::{Args, Result};
use structopt::StructOpt;

use option_ext::OptionExt;

mod option_ext;

fn main() -> Result<()> {
    // Parses our cli args into a shared common struct:
    let args = Args::from_args();
    let res = modulitos_2020::aoc(args.day, args.part, args.input_data_file.try_into()?)?;
    println!("answer is: {:?}", res);

    Ok(())
}