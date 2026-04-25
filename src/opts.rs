use anyhow::Result;
use clap::Parser;
use std::path::Path;

#[derive(Debug, Parser)]
#[command(author, version, about="A CLI tool for converting CSV to other formats", long_about = None, name = "rcli")]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "Convert CSV to other formats")]
    Csv(CsvArgs),
}

#[derive(Debug, Parser)]
pub struct CsvArgs {
    #[arg(short, long, value_parser = verlify_input_file)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    #[arg(long, default_value_t = true)]
    pub header: bool,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
}

fn verlify_input_file(filename: &str) -> Result<String> {
    if !Path::new(filename).exists() {
        return Err(anyhow::anyhow!("Input file {} does not exist", filename));
    }
    Ok(filename.to_string())
}
