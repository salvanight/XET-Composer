use serde::Serialize;
use std::fs;
use std::io;
use std::path::PathBuf;
use chrono::Utc;
use crate::deploy_engine::CompiledArtifact; // Assuming CompiledArtifact is here

#[derive(Serialize)]
pub struct StorableArtifactData {
    pub contract_name: String,
    pub abi: String,
    pub bytecode: String,
    pub address: String,
    pub deployed_at: i64,
}

/// Stores the compiled artifact and deployment information to a JSON file.
///
/// The file will be saved in a `deployments` directory, which is expected to be
/// at `../../deployments/` relative to the backend executable's runtime location.
/// This typically means `xet-composer/deployments/` in the project root.
pub fn store_artifact(
    artifact: &CompiledArtifact,
    address: &str,
) -> Result<(), std::io::Error> {
    // Construct the path to the deployments directory.
    // Assuming the executable runs from somewhere like `xet-composer/backend/xet_composer_backend/target/debug/`
    // or similar, `../../deployments/` should correctly point to `xet-composer/deployments/`.
    // For robustness, it might be better to have this path configurable or determined
    // based on the crate's manifest directory at compile time if possible,
    // but for now, we'll use the relative path as specified.
    let mut deployments_dir = PathBuf::from("."); // Start from current dir (where executable is assumed to run)
    deployments_dir.push(".."); // Go up one level
    deployments_dir.push(".."); // Go up another level
    deployments_dir.push("deployments"); // Target the deployments directory

    // Create the deployments directory if it doesn't exist.
    fs::create_dir_all(&deployments_dir)?;

    // Create an instance of StorableArtifactData.
    let data_to_store = StorableArtifactData {
        contract_name: artifact.contract_name.clone(),
        abi: artifact.abi.clone(), // Assuming abi is String, as per previous subtask
        bytecode: artifact.bytecode.clone(),
        address: address.to_string(),
        deployed_at: Utc::now().timestamp(),
    };

    // Construct the filename: <contract_name>-<address>.json.
    // Basic cleaning for address: remove "0x" if present for a cleaner filename,
    // though the requirement said "use the address as is" for now.
    // Let's stick to "as is" for the moment.
    let filename = format!("{}-{}.json", data_to_store.contract_name, address);
    let file_path = deployments_dir.join(filename);

    // Serialize StorableArtifactData to a JSON string.
    let json_string = serde_json::to_string_pretty(&data_to_store)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    // Write the JSON string to the file.
    fs::write(&file_path, json_string)?;

    Ok(())
}

// Example of how it might be used (for illustration, not part of the actual module usually)
/*
fn _example() {
    // This is just a dummy CompiledArtifact for the example.
    let dummy_artifact = CompiledArtifact {
        contract_name: "MyTestContract".to_string(),
        abi: r#"[{"inputs":[],"name":"myFunction","outputs":[],"stateMutability":"nonpayable","type":"function"}]"#.to_string(),
        bytecode: "0x60806040...".to_string(),
    };
    let dummy_address = "0x1234567890abcdef1234567890abcdef12345678";

    match store_artifact(&dummy_artifact, dummy_address) {
        Ok(()) => println!("Artifact stored successfully."),
        Err(e) => eprintln!("Failed to store artifact: {}", e),
    }
}
*/
