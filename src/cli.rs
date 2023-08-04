use crate::notes::notes_value_parser;
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
    AddNotes {
        #[arg(long, value_parser = notes_value_parser)]
        note: Vec<(u64, String)>,
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
