use std::{env, path::PathBuf, sync::LazyLock};

use clap::{Parser, Subcommand, ValueEnum};
use command::Crate;
mod command;

static CARGO_HOME: LazyLock<String> = LazyLock::new(|| {
    let cargo_home = env::var("CARGO_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            dirs_next::home_dir()
                .map(|home| home.join(".cargo"))
                .expect("can not find cargo home")
        });
    cargo_home.to_string_lossy().to_string()
});

static CARGO_REGISTRY: LazyLock<String> =
    LazyLock::new(|| format!("{}/registry/cache", CARGO_HOME.to_string()));

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::List => command::list(),
        Command::Search { name, mode } => command::search(name, mode),
        Command::Home => println!("Cargo home: {}",CARGO_HOME.to_string()),
    };
}

#[derive(Parser)]
#[command(version,about,long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// cargo home
    Home,
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
