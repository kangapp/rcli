use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Members {
    name: String,
    age: u32,
    address: String,
}

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = csv::Reader::from_path(input)?;
    let mut members = Vec::new();
    for result in reader.deserialize() {
        let record: Members = result?;
        members.push(record);
    }
    let json = serde_json::to_string_pretty(&members)?;
    std::fs::write(output, json)?;
    println!("CSV converted to JSON and saved to {}", output);
    Ok(())
}
