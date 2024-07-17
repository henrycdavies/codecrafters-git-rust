use std::{fs, io::{Read, Result}};

use clap::Args;
use flate2::bufread::ZlibDecoder;

#[derive(Args, Debug)]
pub struct CatFileArgs {
    #[arg(short, long)]
    path: String
}

pub fn read_blob_to_bytes(blob_path: String) -> Result<Vec<u8>> {
    fs::read(blob_path).and_then(|buf| {
        let mut d = ZlibDecoder::new(&buf[..]);
        let mut bytes = Vec::new();
        d.read_to_end(&mut bytes).unwrap();
        Ok(bytes)
    })
}

pub fn cat_file(args: &CatFileArgs) -> Result<()> {
    let objects_dir = "./.git/objects";
    let (blob_dir, blob_file) = args.path.split_at(2);
    let blob_path = format!("{}/{}/{}", objects_dir, blob_dir, blob_file);
    if let Ok(blob_bytes) = read_blob_to_bytes(blob_path) {
        println!("{}", String::from_utf8_lossy(&blob_bytes));
        return Ok(());
    };
    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Blob not found"))
} 