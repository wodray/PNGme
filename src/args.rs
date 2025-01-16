use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about = "A png tool.", long_about = None)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    Encode {
        file_path: PathBuf,
        chunk_type: String,
        message: String,
        output_file: Option<PathBuf>,
    },

    Decode {
        file_path: PathBuf,
        chunk_type: String,
    },

    Remove {
        file_path: PathBuf,
        chunk_type: String,
    },

    Print {
        file_path: PathBuf,
    },
}
