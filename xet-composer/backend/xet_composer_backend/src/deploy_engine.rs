use ethers::prelude::*; // Basic ethers types, actual deployment later
use serde_json::Value;
use std::process::{Command, Output};
use std::path::Path;
use std::fs;
use tempfile::NamedTempFile;
use std::io::Write;

// Error type for this module
#[derive(Debug)]
pub enum DeployError {
    IoError(std::io::Error),
    SolcError(String),
    EthersError(String), // Placeholder for actual ethers errors
    JsonError(serde_json::Error),
    CompilationFailed(String),
    NoAbiFound(String),
    NoBytecodeFound(String),
}

impl From<std::io::Error> for DeployError {
    fn from(err: std::io::Error) -> DeployError {
        DeployError::IoError(err)
    }
}

impl From<serde_json::Error> for DeployError {
    fn from(err: serde_json::Error) -> DeployError {
        DeployError::JsonError(err)
    }
}

pub struct DeployEngine {
    // In the future, this might hold an ethers client or signer.
    // For now, it's stateless or holds configuration for solc.
    solc_path: String, // Path to the solc executable
}

#[derive(Debug, Clone)]
pub struct CompilationOutput {
    pub abi: Value,
    pub bytecode: String, // Hex string of bytecode
}

impl DeployEngine {
    pub fn new(solc_path: String) -> Self {
        Self { solc_path }
    }

    /// Compiles a Solidity source string using solc CLI.
    /// Expects `solc` to be available in the provided path.
    /// `contract_name` should be the name of the contract to extract from solc output (e.g., "TokenVesting").
    pub fn compile_solidity(
        &self,
        solidity_source: &str,
        contract_name: &str,
    ) -> Result<CompilationOutput, DeployError> {
        // Create a temporary file for the Solidity source
        let mut temp_sol_file = NamedTempFile::new()?;
        temp_sol_file.write_all(solidity_source.as_bytes())?;
        let temp_sol_path = temp_sol_file.path().to_str().unwrap();

        // Run solc
        let output = Command::new(&self.solc_path)
            .arg("--abi")
            .arg("--bin")
            .arg("--optimize") // Optional: add optimization
            .arg("-o")
            .arg(".") // Output directory (current dir, files will be named like <contract_name>.abi/bin)
            .arg(temp_sol_path)
            .arg("--overwrite") // Allow overwriting output files
            .output()?; // Get stdout/stderr

        if !output.status.success() {
            return Err(DeployError::SolcError(format!(
                "solc failed with status: {}
stdout: {}
stderr: {}",
                output.status,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        // Read ABI and Bin files - solc creates them in the current directory
        // or a specified output directory. Here, using "." means current dir.
        // The files are typically named like ContractName.abi and ContractName.bin
        // Adjust if solc version outputs differently or if using combined JSON.
        // For simplicity, this assumes separate .abi and .bin files are generated in the CWD.
        // A more robust approach is to use `solc --combined-json abi,bin` and parse that.

        let abi_file_path = format!("{}.abi", contract_name);
        let bin_file_path = format!("{}.bin", contract_name);

        let abi_str = fs::read_to_string(&abi_file_path)?;
        let bytecode_hex = fs::read_to_string(&bin_file_path)?; // This is often just the hex string

        // Clean up temporary files created by solc
        fs::remove_file(&abi_file_path).ok(); // .ok() to ignore error if file not found
        fs::remove_file(&bin_file_path).ok();
        // Temp source file is removed when `temp_sol_file` goes out of scope.

        let abi_json: Value = serde_json::from_str(&abi_str)?;

        Ok(CompilationOutput {
            abi: abi_json,
            bytecode: bytecode_hex.trim().to_string(), // Trim whitespace/newline
        })
    }
    
    /// Placeholder for deploying a compiled contract.
    /// This will be significantly expanded when ethers-rs is fully integrated.
    pub async fn deploy_contract(
        &self,
        _abi: Value, // Will be ethers::abi::Abi
        _bytecode: String, // Will be ethers::types::Bytes
        _constructor_args: Option<Value>, // Parameters for the constructor
    ) -> Result<String, DeployError> {
        // Simulate deployment
        println!("Simulating contract deployment...");
        // In a real scenario:
        // 1. Setup ethers provider & signer.
        // 2. Create ContractFactory.
        // 3. Deploy contract with constructor_args.
        // 4. Wait for transaction confirmation.
        // 5. Return contract address.
        Ok("0xSIMULATED_DEPLOYED_ADDRESS".to_string())
    }
}

// Example usage (will be integrated into main.rs later)
/*
async fn example_deploy() {
    // Assume solc is in PATH or provide direct path
    let engine = DeployEngine::new("solc".to_string()); 
    let source_code = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract MyContract {
    uint public myNumber;
    constructor(uint _initialNumber) {
        myNumber = _initialNumber;
    }
    function setNumber(uint _newNumber) public {
        myNumber = _newNumber;
    }
}"#;

    match engine.compile_solidity(source_code, "MyContract") {
        Ok(comp_output) => {
            println!("ABI: {}", comp_output.abi.to_string());
            println!("Bytecode: {}", comp_output.bytecode);
            
            // Simulate constructor args if any
            // let constructor_args = Some(serde_json::json!([123])); 

            match engine.deploy_contract(comp_output.abi, comp_output.bytecode, None).await {
                Ok(address) => println!("Deployed to: {}", address),
                Err(e) => eprintln!("Deployment error: {:?}", e),
            }
        }
        Err(e) => eprintln!("Compilation error: {:?}", e),
    }
}
*/
