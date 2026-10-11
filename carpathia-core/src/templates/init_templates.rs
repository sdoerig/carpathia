use carpathia_templates::{MARKDOWN, MERMAID, RUST_LIB, SVG};
use flate2::read::GzDecoder;
use std::io::Cursor;
use tar::Archive;

use crate::{
    configuration::carpathia_conf::CarpathiaConfig,
    return_values::carpathia_errors::CarpathiaError, templates::enum_templates::InitTemplate,
};

pub fn extract_to_disk(conf: &CarpathiaConfig) -> Result<(), CarpathiaError> {
    let tar = match conf.init_template {
        InitTemplate::RustLib => GzDecoder::new(Cursor::new(RUST_LIB)),
        InitTemplate::Svg => GzDecoder::new(Cursor::new(SVG)),
        InitTemplate::Markdown => GzDecoder::new(Cursor::new(MARKDOWN)),
        InitTemplate::Mermaid => GzDecoder::new(Cursor::new(MERMAID)),
        InitTemplate::None => {
            // Having none - do nothing just returning..
            return Ok(());
        }
    };
    let mut archive = Archive::new(tar);
    archive
        .unpack(&conf.template_directory)
        .map_err(|e| CarpathiaError {
            message: format!(
                "Failed to extract init template to disk at {:?}: {}",
                conf.template_directory, e
            ),
            error_type:
                crate::return_values::carpathia_errors::ErrorNumber::ErrorWritingInitTemplate,
        })?;
    Ok(())
}
