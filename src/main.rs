use clap::Parser;
use rcli::{process_csv, Opts, Subcommand};
// rcli csv -i input.csv -o output.json --header -d ','

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.output_format)
            };
            process_csv(&opts.input, output, opts.output_format)?
        }
    }
    Ok(())
}
