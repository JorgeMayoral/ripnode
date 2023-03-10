use clap::Parser;
use clap_verbosity_flag::Verbosity;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// See what would be deleted without actually deleting anything
    #[arg(short, long)]
    dry_run: bool,
    /// The name of the folder to delete
    #[arg(short, long, default_value_t = String::from("node_modules"))]
    name: String,
    /// Run as CLI, without TUI
    #[arg(long)]
    non_interactive: bool,
    #[command(flatten)]
    verbose: Verbosity,
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

    pub fn non_interactive(&self) -> bool {
        self.non_interactive
    }

    pub fn verbose(&self) -> &Verbosity {
        &self.verbose
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args() {
        let args = Cli::parse_args();
        assert!(!args.dry_run());
        assert_eq!(args.name(), "node_modules");
        assert!(!args.non_interactive());
        assert_eq!(args.verbose().log_level(), Some(log::Level::Error));
    }
}
