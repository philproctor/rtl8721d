mod args;
mod flash;
mod image;
mod pad;
mod pick;

pub use anyhow::Result;
use args::Command;

fn main() -> Result<()> {
    let args = args::parse()?;
    match args.cmd {
        Command::Flash { .. } => flash::run(),
        Command::Image { .. } => image::run(),
        Command::Pick { .. } => pick::run(),
        Command::Pad { .. } => pad::run(),
    }
}
