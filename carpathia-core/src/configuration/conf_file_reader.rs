use crate::configuration::conf_structs::Types;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub(crate) fn load_type_mappings(path: &PathBuf) -> Result<Types, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let map: Types = serde_json::from_reader(reader)?;
    Ok(map)
}
