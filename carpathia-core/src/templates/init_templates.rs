#![allow(unfulfilled_lint_expectations)]
use flate2::read::GzDecoder;
//use std::fs::File;
use std::io::{Cursor};
use tar::Archive;


const EMBEDDED_TAR_GZ: &[u8] = include_bytes!("../../../tera/rust_lib.tar.gz");

#[expect(dead_code)]
fn extract_to_disk(path: &str) -> std::io::Result<()> {
    let tar = GzDecoder::new(Cursor::new(EMBEDDED_TAR_GZ));
    let mut archive = Archive::new(tar);
    archive.unpack(path)?;
    Ok(())
}
