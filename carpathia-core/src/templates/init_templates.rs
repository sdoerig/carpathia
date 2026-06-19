#![allow(unfulfilled_lint_expectations)]
use flate2::read::GzDecoder;
//use std::fs::File;
use std::io::Cursor;
use tar::Archive;

use crate::{
    configuration::carpathia_conf::CarpathiaConfig, return_values::carpathia_errors::CarpathiaError,
};

const RUST_LIB: &[u8] = include_bytes!("../../../tera/rust_lib.tar.gz");

#[expect(dead_code)]
pub fn extract_to_disk(conf: &CarpathiaConfig) -> Result<(), CarpathiaError> {
    let tar = GzDecoder::new(Cursor::new(RUST_LIB));
    let mut archive = Archive::new(tar);
    archive
        .unpack(&conf.template_directory)
        .map_err(|e| CarpathiaError {
            message: format!(
                "Failed to extract init template to disk at {:?}: {}",
                &conf.template_directory, e
            ),
            error_type:
                crate::return_values::carpathia_errors::ErrorNumber::ErrorWritingInitTemplate,
        })?;
    Ok(())
}
