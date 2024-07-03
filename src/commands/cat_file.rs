use std::{fs, io::{Bytes, Read, Result}};

use clap::Args;
use flate2::bufread::ZlibDecoder;

#[derive(Args, Debug)]
pub struct CatFileArgs {
    #[arg(short, long)]
    path: String
}

pub fn decompress_blob_to_string(buf: &[u8]) -> String {
    let mut d = ZlibDecoder::new(buf);
    let mut out_vec = Vec::new();
    d.read_to_end(&mut out_vec).unwrap();
    String::from_utf8_lossy(&out_vec).into_owned()
}

pub fn cat_file(args: &CatFileArgs) -> Result<()> {
    let objects_dir = "./.git/objects";
    let (blob_dir, blob_file) = args.path.split_at(2);
    let blob_path = format!("{}/{}/{}", objects_dir, blob_dir, blob_file);
    fs::read(blob_path).and_then(|buf| {
        let out = decompress_blob_to_string(buf.as_slice());
        println!("{}", out);
        Ok(())
    })
}