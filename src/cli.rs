use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// See what would be deleted without actually deleting anything
    #[arg(short, long)]
    dry_run: bool,
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }

    pub fn dry_run(&self) -> bool {
        self.dry_run
    }
}