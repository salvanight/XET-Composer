use serde::Serialize; // For serializing the artifact content
use serde_json::Value; // For ABI
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use chrono::{Utc, NaiveDateTime, Datelike}; // For YYYY-MM-DD subdirectory

// Re-define a structure for serialization, or use CompiledArtifact if it's serializable
// For clarity, let's define what we want to save.
#[derive(Serialize)]
struct StoredArtifact<'a> {
    #[serde(rename = "contractName")]
    contract_name: &'a str,
    abi: &'a Value,
    bytecode: &'a str,
    #[serde(rename = "compilationTimestamp")]
    compilation_timestamp: u64,
}

// Assuming CompiledArtifact is defined in deploy_engine.rs and available here
// If not, you might need to pass its fields directly or redefine a local version.
// For this subtask, assume deploy_engine::CompiledArtifact is accessible.
use crate::deploy_engine::CompiledArtifact;


/// Saves the compilation artifact to a JSON file.
/// The file will be stored in `base_deployments_path / YYYY-MM-DD / CONTRACT_NAME-TIMESTAMP.json`.
/// Returns the full path to the saved artifact file.
pub fn save_artifact(
    artifact: &CompiledArtifact,
    base_deployments_path: &Path,
    timestamp_secs: u64,
) -> io::Result<PathBuf> {
    // Create YYYY-MM-DD subdirectory
    let naive_datetime = NaiveDateTime::from_timestamp_opt(timestamp_secs as i64, 0)
        .unwrap_or_else(|| NaiveDateTime::from_timestamp_opt(0, 0).unwrap()); // Fallback to epoch
    let date_subdir_name = format!("{}", naive_datetime.format("%Y-%m-%d"));
    let date_specific_path = base_deployments_path.join(date_subdir_name);

    fs::create_dir_all(&date_specific_path)?;

    // Define filename: CONTRACT_NAME-TIMESTAMP.json
    let filename = format!("{}-{}.json", artifact.contract_name, timestamp_secs);
    let full_artifact_path = date_specific_path.join(filename);

    // Prepare content to save
    let stored_content = StoredArtifact {
        contract_name: &artifact.contract_name,
        abi: &artifact.abi,
        bytecode: &artifact.bytecode,
        compilation_timestamp: timestamp_secs,
    };

    let mut file = File::create(&full_artifact_path)?;
    let json_string = serde_json::to_string_pretty(&stored_content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?; // Convert serde_json::Error to io::Error
    
    file.write_all(json_string.as_bytes())?;

    Ok(full_artifact_path)
}
