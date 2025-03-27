pub mod csv;
pub mod genpass;
pub mod base64;

use csv::{CsvOpts, OutputFormat};
use genpass::GenPassOpts;
use base64::Base64SubCommand;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),

    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),

    #[command(name = "base54", about = "Encode or decode base64")]
    Base64(Base64SubCommand),
}

