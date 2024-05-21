use clap::Parser;
use rcli::{process_csv, Opts, Subcommand};
// rcli csv -i input.csv -o output.json --header -d ','

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => process_csv(&opts.input, &opts.output)?,
    }
    Ok(())
}
