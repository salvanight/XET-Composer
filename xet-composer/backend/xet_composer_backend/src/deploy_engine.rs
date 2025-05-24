use ethers::prelude::*; // Basic ethers types, actual deployment later
use serde_json::Value;
use std::process::{Command, Output};
use std::path::Path; // Keep Path
use std::fs;
use tempfile::{NamedTempFile, Builder}; // Added Builder for tempdir
use std::io::Write;

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
    solc_executable: String, // Modified field name
}

#[derive(Debug, Clone)]
pub struct CompilationOutput {
    pub abi: Value,
    pub bytecode: String, // Hex string of bytecode
}

impl DeployEngine {
    pub fn new(solc_executable: String) -> Self { // Modified
        Self { solc_executable }
    }

    /// Compiles a Solidity source string using solc CLI.
    pub fn compile_solidity(
        &self,
        solidity_source: &str,
        contract_name: &str,
        base_path: &Path, // New parameter
        remappings: &[String], // New parameter: e.g., "@openzeppelin/=lib/openzeppelin/"
    ) -> Result<CompilationOutput, DeployError> {
        let mut temp_sol_file = NamedTempFile::new()?; // Handled by From<std::io::Error>
        temp_sol_file.write_all(solidity_source.as_bytes())?;
        let temp_sol_path = temp_sol_file.path();

        // Output directory for ABI and BIN files - use a temporary directory
        let temp_out_dir = Builder::new().prefix("solc_out_").tempdir()
            .map_err(DeployError::TempDirError)?; // Specific error for tempdir
        let out_dir_path_str = temp_out_dir.path().to_str().unwrap_or_default(); // Handle potential None from to_str

        let mut cmd = Command::new(&self.solc_executable);
        cmd.arg("--abi")
           .arg("--bin")
           .arg("--optimize")
           .arg("--overwrite") // Important for subsequent calls
           .arg("-o")
           .arg(out_dir_path_str) // Output to temp directory
           .arg("--base-path")    // Add base-path
           .arg(base_path)        // The actual base path
           .arg(temp_sol_path);   // Input .sol file

        // Add remappings
        for remap in remappings {
            cmd.arg(remap);
        }
        
        let output = cmd.output()?; // Handled by From<std::io::Error>

        if !output.status.success() {
            return Err(DeployError::SolcError(format!(
                "solc failed with status: {}\nstdout: {}\nstderr: {}",
                output.status,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        // Construct paths to ABI and BIN files within the temp output directory
        let abi_file_path = temp_out_dir.path().join(format!("{}.abi", contract_name));
        let bin_file_path = temp_out_dir.path().join(format!("{}.bin", contract_name));

        let abi_str = fs::read_to_string(&abi_file_path)
            .map_err(|e| DeployError::NoAbiFound(format!("Could not read ABI file {:?}: {}", abi_file_path, e)))?;
        let bytecode_hex = fs::read_to_string(&bin_file_path)
            .map_err(|e| DeployError::NoBytecodeFound(format!("Could not read BIN file {:?}: {}", bin_file_path, e)))?;
         
        let abi_json: Value = serde_json::from_str(&abi_str)?; // Handled by From<serde_json::Error>

        Ok(CompilationOutput {
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

// Example usage (commented out, for reference)
/*
async fn example_deploy() {
    let solc_exe = env::var("SOLC_PATH").unwrap_or_else(|_| "solc".to_string());
    let engine = DeployEngine::new(solc_exe); 
    let source_code = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
import "@openzeppelin/contracts/utils/Context.sol"; // Example import
contract MyContract is Context {
    uint public myNumber;
    constructor(uint _initialNumber) {
        myNumber = _initialNumber;
    }
    function setNumber(uint _newNumber) public {
        myNumber = _newNumber;
    }
}"#;
    
    // Define a base path (e.g., where your 'lib' or 'node_modules' might be if not in default include paths)
    // For this example, assume 'contracts' is our base, and openzeppelin is in 'contracts/lib/openzeppelin-repo/contracts'
    let base_contracts_dir = PathBuf::from("./"); // Or wherever your project root relative to execution is
    let remappings = vec!["@openzeppelin/contracts/=lib/openzeppelin-repo/contracts/".to_string()];

    match engine.compile_solidity(source_code, "MyContract", &base_contracts_dir, &remappings) {
        Ok(comp_output) => {
            println!("ABI: {}", comp_output.abi.to_string());
            println!("Bytecode: {}", comp_output.bytecode);
            
            match engine.deploy_contract(comp_output.abi, comp_output.bytecode, None).await {
                Ok(address) => println!("Deployed to: {}", address),
                Err(e) => eprintln!("Deployment error: {:?}", e),
            }
        }
        Err(e) => eprintln!("Compilation error: {:?}", e),
    }
}
*/
