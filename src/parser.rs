use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::logic;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Display your notes without styles
    List,
    ///Display your notes with styles
    Display,
    /// Display note with content by index
    Cat {
        index: usize,

        /// Special group?
        #[arg(short, long)]
        special: bool,
    },
    /// Create new note
    New {
        name: String,
        content: Option<String>,

        /// Special group?
        #[arg(short, long)]
        special: bool,
    },
    /// Delete note by index
    Delete {
        index: usize,

        /// Special group?
        #[arg(short, long)]
        special: bool,

        /// Will not ask "Are you sure?"
        #[arg(short, long)]
        force: bool,
    },
    /// You can use this to create your own interface for CL.NET Notes
    PrintJson,
}

pub async fn parse() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => {
            logic::execute_list().await?;
        }
        Commands::Display => {
            logic::execute_display().await?;
        }
        Commands::Cat { index, special } => {
            logic::execute_cat(index, special).await?;
        }
        Commands::New {
            name,
            content,
            special,
        } => {
            logic::execute_new(name, content, special).await?;
        }
        Commands::Delete {
            index,
            special,
            force,
        } => {
            logic::execute_delete(index, special, force).await?;
        }
        Commands::PrintJson => {
            logic::execute_printjson().await?;
        }
    }

    Ok(())
}
