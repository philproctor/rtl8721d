use super::*;
use structopt::clap::AppSettings::*;
use structopt::StructOpt;

pub fn parse() -> Result<Args> {
    Ok(Args::from_args())
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "cargo rtl8720",
    rename_all = "kebab_case",
    about,
    settings = &[ArgRequiredElseHelp, DeriveDisplayOrder, UnifiedHelpMessage]
)]
pub struct Args {
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    /// Flash RTL8720
    Flash {},
    /// Build image from compiled object
    Image {},
    /// Pick section from image
    Pick {},
    /// Pad image
    Pad {},
}
