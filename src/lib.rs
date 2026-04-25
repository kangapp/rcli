mod opts;
mod process;

pub use opts::{Cli, Subcommand};
pub use process::process_csv;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_process_csv_with_test_data() {
        let input = "assets/test.csv";
        let output = "assets/output_test.json";

        process_csv(input, output).unwrap();

        let content = fs::read_to_string(output).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();

        assert!(json.is_array());
        assert_eq!(json.as_array().unwrap().len(), 2);

        let first = &json[0];
        assert_eq!(first["Name"].as_str().unwrap(), "刘付康");
        assert_eq!(first["Age"].as_u64().unwrap(), 30);
        assert_eq!(first["Address"].as_str().unwrap(), "七所大院");

        fs::remove_file(output).ok();
    }

    #[test]
    fn test_members_deserialization() {
        use serde::Deserialize;
        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct Members {
            name: String,
            age: u32,
            #[allow(dead_code)]
            address: String,
        }

        let json = r#"[
            {"Name": "刘付康", "Age": 30, "Address": "七所大院"},
            {"Name": "唐玮", "Age": 28, "Address": "七所大院"}
        ]"#;

        let members: Vec<Members> = serde_json::from_str(json).unwrap();
        assert_eq!(members.len(), 2);
        assert_eq!(members[0].name, "刘付康");
        assert_eq!(members[0].age, 30);
    }
}
