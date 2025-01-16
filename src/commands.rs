use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use anyhow::Result;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub(crate) fn encode_msg(
    file_path: PathBuf,
    chunk_type: String,
    message: String,
    output_file: Option<PathBuf>,
) -> Result<()> {
    let file_bytes = read_to_bytes(&file_path)?;
    let mut png = Png::try_from(file_bytes.as_ref())?;
    let chunk = Chunk::new(ChunkType::from_str(&chunk_type)?, message.into_bytes());
    png.append_chunk(chunk);

    match output_file {
        Some(output_file) => {
            let mut wf = File::create(output_file)?;
            wf.write_all(png.as_bytes().as_ref())?;
        }
        None => {
            let temp_p = file_path.with_extension("png.temp");
            let mut temp_f = File::create(&temp_p)?;
            temp_f.write_all(png.as_bytes().as_ref())?;
            fs::rename(temp_p, file_path)?;
        }
    }
    println!("Encode {} successfully", chunk_type);
    Ok(())
}

pub(crate) fn decode_msg(file_path: PathBuf, chunk_type: String) -> Result<()> {
    let file_bytes = read_to_bytes(file_path)?;
    let png = Png::try_from(file_bytes.as_ref())?;
    let msg_chunk = png.chunk_by_type(&chunk_type);
    match msg_chunk {
        Some(chunk) => println!("{}: {}", chunk_type, chunk),
        None => println!("No such chunk"),
    }
    Ok(())
}

pub(crate) fn remove_msg(file_path: PathBuf, chunk_type: String) -> Result<()> {
    let file_bytes = read_to_bytes(&file_path)?;
    let mut png = Png::try_from(file_bytes.as_ref())?;
    match png.remove_first_chunk(&chunk_type) {
        Ok(_) => {
            let temp_p = file_path.with_extension("png.temp");
            let mut temp_f = File::create(&temp_p)?;
            temp_f.write_all(png.as_bytes().as_ref())?;
            fs::rename(temp_p, file_path)?;
            println!("Removed {}", chunk_type);
        }
        Err(e) => println!("{e}"),
    }
    Ok(())
}

pub(crate) fn print_msg(file_path: PathBuf) -> Result<()> {
    let file_bytes = read_to_bytes(file_path)?;
    let png = Png::try_from(file_bytes.as_ref())?;
    println!("{png}");
    Ok(())
}

fn read_to_bytes<P: AsRef<Path>>(file_path: P) -> Result<Vec<u8>> {
    let mut rf = File::open(&file_path)?;
    let mut file_bytes = Vec::new();
    rf.read_to_end(&mut file_bytes)?;
    Ok(file_bytes)
}

// 安全修改文件内容（如果是就地修改文件，程序运行时断电或突然终止，可能会损坏文件内容）
fn safe_overwrite() -> Result<()> {
    // 读取原文件的内容
    let mut rf = File::open("foo.txt")?;
    let mut file_bytes = vec![];
    rf.read_to_end(&mut file_bytes)?;
    drop(rf); // 显式删除文件对象，避免无法覆盖

    file_bytes.remove(0);
    // 创建临时文件
    let mut temp_file = File::create("foo_new.txt")?;
    temp_file.write_all(&file_bytes)?;
    // 使用临时文件替换原文件
    fs::rename("foo_new.txt", "foo.txt")?;
    Ok(())
}
