use clap::{
    Parser,
    Subcommand,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = "ksau", version = VERSION, about = "ksau cli tool")]
pub struct KsauCli {
    /// Upload a file
    #[command(subcommand)]
    pub command: Option<Command>,

    // /// List info about remotes
    // #[subcommand]
    // list: List
}

#[derive(Subcommand)]
pub enum Command {
    Upload(Upload),
    List(List),
}

#[derive(Parser)]
pub struct Upload {
    /// The file to upload
    pub file: String,

    /// The folder to upload to
    #[arg(short, long)]
    pub folder: Option<String>,
}

#[derive(Parser)]
pub struct List {}

pub fn parse_args() -> KsauCli {
    KsauCli::parse()
}
