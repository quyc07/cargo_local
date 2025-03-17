use std::sync::LazyLock;

use clap::{Parser, Subcommand, ValueEnum};
use command::Crate;
mod command;

static CARGO_REGISTRY: LazyLock<String> =
    LazyLock::new(|| "/usr/local/cargo/registry/cache".to_string());

fn main() {
    let cli = Cli::parse();
    let res: Vec<Crate> = match cli.command {
        Command::List => command::list(),
        Command::Search { name, mode } => command::search(name, mode),
    };
    res.iter().for_each(|c| println!("{c}"));
}

#[derive(Parser)]
#[command(version,about,long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    List,

    Search {
        #[arg(short)]
        name: String,
        #[arg(short,default_value=Mode::All)]
        mode: Mode,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    All,
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
