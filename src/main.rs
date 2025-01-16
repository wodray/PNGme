mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use crate::args::{Args, Commands};
use crate::commands::{decode_msg, encode_msg, print_msg, remove_msg};
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = Args::parse();

    match cli.command {
        Commands::Encode {
            file_path,
            chunk_type,
            message,
            output_file,
        } => encode_msg(file_path, chunk_type, message, output_file)?,
        Commands::Decode {
            file_path,
            chunk_type,
        } => decode_msg(file_path, chunk_type)?,
        Commands::Remove {
            file_path,
            chunk_type,
        } => remove_msg(file_path, chunk_type)?,
        Commands::Print { file_path } => print_msg(file_path)?,
    }
    Ok(())
}
