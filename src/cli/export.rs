//! CLI export command for Structurizr DSL serialization.

use std::path::PathBuf;
use structopt::StructOpt;

/// Export C4 model to Structurizr DSL format
#[derive(Debug, StructOpt)]
pub struct ExportCommand {
    /// Input file path (JSON format)
    #[structopt(short, long, parse(from_os_str))]
    pub input: PathBuf,

    /// Output file path (default: stdout)
    #[structopt(short, long, parse(from_os_str))]
    pub output: Option<PathBuf>,
}

impl ExportCommand {
    pub fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
