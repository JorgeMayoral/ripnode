use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// See what would be deleted without actually deleting anything
    #[arg(short, long)]
    dry_run: bool,
    /// The name of the folder to delete
    #[arg(short, long, default_value_t = String::from("node_modules"))]
    name: String,
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }

    pub fn dry_run(&self) -> bool {
        self.dry_run
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}