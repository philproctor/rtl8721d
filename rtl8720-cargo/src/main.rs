mod args;

pub use anyhow::Result;
use args::Command;

fn main() -> Result<()> {
    let args = args::parse()?;
    match args.cmd {
        Command::Flash { .. } => {
            println!("Flash not implemented yet");
        }
        Command::Image { .. } => {
            println!("Image not implemented yet");
        }
        Command::Pick { .. } => {
            println!("Pick not implemented yet");
        }
        Command::Pad { .. } => {
            println!("Pad not implemented yet");
        }
    };
    Ok(())
}
