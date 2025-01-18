use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Embedding message in png file
    Encode {
        /// The png file path
        file_path: PathBuf,
        /// Type of message chunk
        chunk_type: String,
        /// Embedded message content
        message: String,
        /// Path to the new png file containing the embedded message
        output_file: Option<PathBuf>,
    },

    /// Fetch the embedded message
    Decode {
        /// The png file path
        file_path: PathBuf,
        /// Type of message chunk
        chunk_type: String,
    },

    /// Delete the given embedded message
    Remove {
        /// The png file path
        file_path: PathBuf,
        /// Type of message chunk
        chunk_type: String,
    },

    /// Display all embedded messages
    Print {
        /// The png file path
        file_path: PathBuf,
    },
}
