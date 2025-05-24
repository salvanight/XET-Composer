use axum::{routing::post, Router, Json};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::env;
use chrono::Utc; // Added for timestamp

// Module declarations
mod sol_template_engine;
mod deploy_engine;
mod artifact_manager; // Added module

// Use statements for our modules
use sol_template_engine::{SolTemplateEngine, TemplateError};
use deploy_engine::{DeployEngine, CompiledArtifact, DeployError};
// artifact_manager is used via its functions

#[derive(Deserialize, Debug)]
struct DeployRequest {
    contract: String, // e.g., "TokenVesting.sol.tera"
    params: serde_json::Value,
}

// Updated DeployResult struct
#[derive(Serialize, Debug)]
struct DeployResult {
    success: bool,
    message: String,
    contract_name: Option<String>,
    contract_address: Option<String>, // Remains simulated for now
    abi: Option<serde_json::Value>,
    bytecode: Option<String>,
    compilation_timestamp: Option<u64>, // New field
    artifact_path: Option<String>,     // New field
}

async fn deploy_handler(Json(payload): Json<DeployRequest>) -> Json<DeployResult> {
    println!("Received deploy request for contract template: {}", payload.contract);
    println!("Params: {:?}", payload.params);

    // --- Conceptual Placeholder for Pre-flight/KYC Checks ---
    // Here, you might add checks before proceeding with resource-intensive operations.
    // For example, check for a required user signature or other parameters.
    /*
    let perform_kyc_check = false; // Make this true to simulate the check
    if perform_kyc_check {
        if payload.params.get("user_signature").is_none() || 
           payload.params.get("legal_name").is_none() {
            println!("Pre-flight check failed: Missing required KYC parameters (e.g., user_signature, legal_name).");
            return Json(DeployResult {
                success: false,
                message: "Pre-flight check failed: Missing required KYC parameters.".to_string(),
                contract_name: None,
                contract_address: None,
                abi: None,
                bytecode: None,
                compilation_timestamp: None,
                artifact_path: None,
            });
        }
        println!("Pre-flight/KYC check passed (simulated).");
    }
    */
    // --- End of Placeholder ---

    // --- Configuration ---
    let contracts_base_dir = PathBuf::from("../../contracts")
        .canonicalize()
        .unwrap_or_else(|e| {
            eprintln!("Warning: could not canonicalize contracts path '../../contracts': {}. Using relative path.", e);
            PathBuf::from("../../contracts")
        });
    
    println!("Using contracts base directory: {:?}", contracts_base_dir);

    let solc_executable = env::var("SOLC_PATH").unwrap_or_else(|_| "solc".to_string());
    println!("Using SOLC executable: {}", solc_executable);
    
    let solc_remappings = vec![
        "@openzeppelin/contracts/=lib/openzeppelin-repo/contracts/".to_string(),
    ];
    println!("Using SOLC remappings: {:?}", solc_remappings);

    let template_engine = match SolTemplateEngine::new(contracts_base_dir.clone()) {
        Ok(engine) => engine,
        Err(e) => {
            eprintln!("Failed to initialize SolTemplateEngine: {:?}", e);
            return Json(DeployResult {
                success: false,
                message: format!("Failed to initialize template engine: {:?}", e),
                contract_name: None,
                contract_address: None,
                abi: None,
                bytecode: None,
                compilation_timestamp: None, // New field
                artifact_path: None,         // New field
            });
        }
    };

    let deploy_engine = DeployEngine::new(solc_executable.clone());

    let rendered_solidity = match template_engine.render_template(&payload.contract, &payload.params) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Failed to render template {}: {:?}", payload.contract, e);
            return Json(DeployResult {
                success: false,
                message: format!("Failed to render template '{}': {:?}", payload.contract, e),
                contract_name: None,
                contract_address: None,
                abi: None,
                bytecode: None,
                compilation_timestamp: None, // New field
                artifact_path: None,         // New field
            });
        }
    };

    let contract_name_to_compile = payload.contract.replace(".sol.tera", "");

    match deploy_engine.compile_solidity(&rendered_solidity, &contract_name_to_compile, &contracts_base_dir, &solc_remappings) {
        Ok(compiled_artifact) => {
            println!("Compilation successful for {}", compiled_artifact.contract_name);
            
            let current_timestamp = Utc::now().timestamp() as u64;
            let base_deployments_dir = PathBuf::from("../../deployments")
                .canonicalize()
                .unwrap_or_else(|e| {
                    eprintln!("Warning: could not canonicalize deployments path '../../deployments': {}. Using relative path './deployments'.", e);
                    PathBuf::from("./deployments") // Fallback to current dir's deployments
                });

            let mut success_message = format!("Contract '{}' compiled successfully.", compiled_artifact.contract_name);
            let artifact_file_path_str: Option<String> = 
                match artifact_manager::save_artifact(&compiled_artifact, &base_deployments_dir, current_timestamp) {
                Ok(path) => {
                    let path_str = path.to_string_lossy().into_owned();
                    println!("Artifact saved to: {}", path_str);
                    success_message = format!("Contract '{}' compiled successfully. Artifact saved.", compiled_artifact.contract_name);
                    Some(path_str)
                }
                Err(e) => {
                    eprintln!("Failed to save artifact for {}: {:?}", compiled_artifact.contract_name, e);
                    success_message = format!("Contract '{}' compiled successfully, but failed to save artifact: {:?}", compiled_artifact.contract_name, e);
                    None
                }
            };
            
            // Deployment is still simulated.
            Json(DeployResult {
                success: true,
                message: success_message, // Updated message
                contract_name: Some(compiled_artifact.contract_name.clone()), 
                contract_address: Some("0xSIMULATED_ADDRESS_AFTER_COMPILE".to_string()), // Still simulated
                abi: Some(compiled_artifact.abi.clone()),
                bytecode: Some(compiled_artifact.bytecode.clone()),
                compilation_timestamp: Some(current_timestamp), // Populate new field
                artifact_path: artifact_file_path_str,       // Populate new field
            })
        }
        Err(e) => {
            eprintln!("Failed to compile Solidity for {}: {:?}", contract_name_to_compile, e);
            let error_message = match e {
                DeployError::SolcError(solc_err) => format!("Solidity compilation failed: {}", solc_err),
                DeployError::NoAbiFound(path_err) => format!("ABI file not found after compilation: {}", path_err),
                DeployError::NoBytecodeFound(path_err) => format!("Bytecode file not found after compilation: {}", path_err),
                _ => format!("Solidity compilation failed: {:?}", e),
            };
            Json(DeployResult {
                success: false,
                message: error_message,
                contract_name: Some(contract_name_to_compile),
                contract_address: None,
                abi: None,
                bytecode: None,
                compilation_timestamp: None, // New field
                artifact_path: None,         // New field
            })
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/deploy", post(deploy_handler));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Backend server listening on {}", addr);

    if let Err(e) = axum::Server::bind(&addr).serve(app.into_make_service()).await {
        eprintln!("Server error: {}", e);
    }
}
