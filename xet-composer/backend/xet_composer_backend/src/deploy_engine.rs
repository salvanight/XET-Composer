// In deploy_engine.rs
// ...
use serde_json::Value; // Ensure this is imported if not already at top level
use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::{NamedTempFile, Builder}; // Builder for tempdir

// Error type for this module
#[derive(Debug)]
pub enum DeployError {
    IoError(std::io::Error),
    SolcError(String),
    EthersError(String), // Placeholder for actual ethers errors
    JsonError(serde_json::Error),
    CompilationFailed(String), // Kept this, but SolcError is more specific for compilation
    NoAbiFound(String),
    NoBytecodeFound(String),
    TempDirError(std::io::Error), // For tempdir creation errors
}

impl From<std::io::Error> for DeployError {
    fn from(err: std::io::Error) -> DeployError {
        // Differentiate tempdir errors if possible, or generalize
        DeployError::IoError(err)
    }
}

impl From<serde_json::Error> for DeployError {
    fn from(err: serde_json::Error) -> DeployError {
        DeployError::JsonError(err)
    }
}


pub struct DeployEngine {
    solc_executable: String,
}

#[derive(Debug, Clone)] // Ensure Clone is kept if it was there
pub struct CompiledArtifact { // Renamed from CompilationOutput
    pub contract_name: String, // New field
    pub abi: Value,
    pub bytecode: String,
}

impl DeployEngine {
    pub fn new(solc_executable: String) -> Self {
        Self { solc_executable }
    }

    pub fn compile_solidity(
        &self,
        solidity_source: &str,
        contract_name_to_compile: &str, // Parameter providing the contract name
        base_path: &Path,
        remappings: &[String],
    ) -> Result<CompiledArtifact, DeployError> { // Return type updated
        let mut temp_sol_file = NamedTempFile::new()?;
        temp_sol_file.write_all(solidity_source.as_bytes())?;
        let temp_sol_path = temp_sol_file.path();

        let temp_out_dir = Builder::new().prefix("solc_out_").tempdir()
            .map_err(DeployError::TempDirError)?; // Specific error for tempdir
        let out_dir_path_str = temp_out_dir.path().to_str().unwrap_or_default();

        let mut cmd = Command::new(&self.solc_executable);
        cmd.arg("--abi")
           .arg("--bin")
           .arg("--optimize")
           .arg("--overwrite")
           .arg("-o")
           .arg(out_dir_path_str)
           .arg("--base-path")
           .arg(base_path)
           .arg(temp_sol_path);

        for remap in remappings {
            cmd.arg(remap);
        }
        
        let output = cmd.output()?;

        if !output.status.success() {
            return Err(DeployError::SolcError(format!(
                "solc failed with status: {}\nstdout: {}\nstderr: {}",
                output.status,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        let abi_file_path = temp_out_dir.path().join(format!("{}.abi", contract_name_to_compile));
        let bin_file_path = temp_out_dir.path().join(format!("{}.bin", contract_name_to_compile));

        let abi_str = fs::read_to_string(&abi_file_path)
            .map_err(|e| DeployError::NoAbiFound(format!("Could not read ABI file {:?}: {}", abi_file_path, e)))?;
        let bytecode_hex = fs::read_to_string(&bin_file_path)
            .map_err(|e| DeployError::NoBytecodeFound(format!("Could not read BIN file {:?}: {}", bin_file_path, e)))?;
         
        let abi_json: Value = serde_json::from_str(&abi_str)?;

        Ok(CompiledArtifact { // Update struct instantiation
            contract_name: contract_name_to_compile.to_string(), // Populate new field
            abi: abi_json,
            bytecode: bytecode_hex.trim().to_string(),
        })
    }
    
    /// Placeholder for deploying a compiled contract.
    pub async fn deploy_contract(
        &self,
        _abi: Value,
        _bytecode: String,
        _constructor_args: Option<Value>,
    ) -> Result<String, DeployError> {
        println!("Simulating contract deployment...");
        Ok("0xSIMULATED_DEPLOYED_ADDRESS".to_string())
    }
}
