mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use anyhow::Result;

fn main() -> Result<()> {
    let a: [u8; 4] = [82, 117, 83, 116];
    println!("{}", String::from_utf8(a.to_vec())?);
    Ok(())
}
