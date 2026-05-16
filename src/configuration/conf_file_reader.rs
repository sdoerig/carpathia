use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use crate::configuration::conf_structs::TypeMapping;
use crate::return_values::carpathia_errors::{CarpathiaError, ErrorNumber};

fn load_type_mappings(path: &str) -> Result<BTreeMap<String, TypeMapping>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let map: BTreeMap<String, TypeMapping> = serde_json::from_reader(reader)?;
    Ok(map)
}
