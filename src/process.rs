use std::fs;

use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};

use crate::opts::Outputformat;
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    // #[serde(rename="Name")]
    name: String,
    // #[serde(rename="Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    // #[serde(rename="Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: String, output_format: Outputformat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        let json_value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();
        ret.push(json_value);
    }
    let content = match output_format {
        Outputformat::Json => serde_json::to_string_pretty(&ret)?,
        Outputformat::Yaml => serde_yaml::to_string(&ret)?,
        // Outputformat::Toml => toml::to_string_pretty(&ret)?,
    };
    fs::write(output, content)?;
    Ok(())
}
