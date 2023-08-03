use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    AddNote {
        #[arg(long)]
        id: u64,
        #[arg(long)]
        content: String,
    },
    GetNote {
        #[arg(long)]
        id: u64,
    },
    DeleteNote {
        #[arg(long)]
        id: u64,
    },
    ListNotes {},
}
