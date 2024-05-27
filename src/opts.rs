use clap::Parser;
use std::{fmt, path::Path, str::FromStr};
#[derive(Debug, Parser)]
#[command(name="rcli",version,author,about,long_about= None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}
#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "show csv or convert csv to other formats")]
    Csv(CsvOpts),
}
#[derive(Debug, Clone, Copy)]
pub enum Outputformat {
    Json,
    Yaml,
    // Toml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short,long,value_parser = verify_input_file)]
    pub input: String,
    #[arg(long)]
    pub output: Option<String>,
    #[arg(short,long,value_parser=parse_format, default_value = "json")]
    pub output_format: Outputformat,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}
fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File not found")
    }
}

fn parse_format(fmt: &str) -> Result<Outputformat, anyhow::Error> {
    fmt.parse()
}

impl From<Outputformat> for &'static str {
    fn from(format: Outputformat) -> Self {
        match format {
            Outputformat::Json => "json",
            Outputformat::Yaml => "yaml",
            // Outputformat::Toml=>"toml",
        }
    }
}

impl FromStr for Outputformat {
    type Err = anyhow::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "json" => Ok(Outputformat::Json),
            "yaml" => Ok(Outputformat::Yaml),
            // "toml"=> Ok(Outputformat::Toml),
            v => anyhow::bail!("Unsupported format :{}", v),
        }
    }
}

impl fmt::Display for Outputformat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
