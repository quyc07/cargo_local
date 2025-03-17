use std::sync::LazyLock;

use clap::{Parser, Subcommand, ValueEnum};
use command::Crate;
mod command;

static CARGO_REGISTRY: LazyLock<String> =
    LazyLock::new(|| "/usr/local/cargo/registry/cache".to_string());

fn main() {
    let cli = Cli::parse();
    let crates: Vec<Crate> = match cli.command {
        Command::List => command::list(),
        Command::Search { name, mode } => command::search(name, mode),
    };
    let max_name_len = crates.iter().map(|c| c.name.len()).max().unwrap_or(10);

    crates
        .iter()
        .for_each(|c| println!("{:<width$} {}", c.name, c.version, width = max_name_len));
}

#[derive(Parser)]
#[command(version,about,long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// show all crates
    List,
    /// search crates
    Search {
        /// crate name
        #[arg(short)]
        name: String,
        /// search mode
        #[arg(short,default_value=Mode::All)]
        mode: Mode,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    /// search and show all crates
    All,
    /// search and only show newest crate
    New,
}

impl Mode {
    fn name(&self) -> String {
        match self {
            Mode::All => "all".to_string(),
            Mode::New => "new".to_string(),
        }
    }
}

impl From<Mode> for clap::builder::OsStr {
    fn from(value: Mode) -> Self {
        let name = value.name();
        name.into()
    }
}
