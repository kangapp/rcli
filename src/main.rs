// rcli csv -i input.csv -o output.json --header -d ','
use anyhow::Result;
use clap::Parser;
use rcli::{process_csv, Cli, Subcommand};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Subcommand::Csv(args) => {
            process_csv(&args.input, &args.output)?;
        }
    }
    Ok(())
}
