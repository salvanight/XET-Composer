use axum::{routing::post, Router, Json};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::env; // Added for env::var

// Existing module declarations
mod sol_template_engine;
mod deploy_engine;

// Use statements for our modules
use sol_template_engine::{SolTemplateEngine, TemplateError};
use deploy_engine::{DeployEngine, CompilationOutput, DeployError};

#[derive(Deserialize, Debug)]
struct DeployRequest {
    contract: String, // e.g., "TokenVesting.sol.tera"
    params: serde_json::Value,
}

#[derive(Serialize, Debug)]
struct DeployResult {
    success: bool,
    message: String,
    contract_name: Option<String>,
    contract_address: Option<String>, // Still simulated for now
    abi: Option<serde_json::Value>,
    bytecode: Option<String>,
}

async fn deploy_handler(Json(payload): Json<DeployRequest>) -> Json<DeployResult> {
    println!("Received deploy request for contract template: {}", payload.contract);
    println!("Params: {:?}", payload.params);

    // --- Configuration ---
    let contracts_base_dir = PathBuf::from("../../contracts") // Relative to executable in target/debug or if run from workspace root
        .canonicalize()
        .unwrap_or_else(|e| {
            eprintln!("Failed to canonicalize contracts_base_dir: {:?}. Using relative path.", e);
            // Fallback: if canonicalize fails (e.g. dir doesn't exist yet in test env),
            // provide a relative path. This might be okay if solc handles it.
            PathBuf::from("../../contracts")
        });
    
    println!("Using contracts base directory: {:?}", contracts_base_dir);

    let solc_executable = env::var("SOLC_PATH").unwrap_or_else(|_| "solc".to_string());
    println!("Using SOLC executable: {}", solc_executable);
    
    let solc_remappings = vec![
        "@openzeppelin/contracts/=lib/openzeppelin-repo/contracts/".to_string(),
        // Example: "ds-test/=lib/forge-std/lib/ds-test/src/"
    ];
    println!("Using SOLC remappings: {:?}", solc_remappings);


    // Initialize engines
    // SolTemplateEngine expects the path to the directory containing .sol.tera files.
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
            });
        }
    };

    let deploy_engine = DeployEngine::new(solc_executable.clone());

    // 1. Render the Solidity template
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
            });
        }
    };

    let contract_name_to_compile = payload.contract.replace(".sol.tera", "");

    // 2. Compile the rendered Solidity
    // Pass the contracts_base_dir as the base_path for solc.
    match deploy_engine.compile_solidity(&rendered_solidity, &contract_name_to_compile, &contracts_base_dir, &solc_remappings) {
        Ok(comp_output) => {
            println!("Compilation successful for {}", contract_name_to_compile);
            // Deployment is still simulated.
            Json(DeployResult {
                success: true,
                message: format!("Contract '{}' compiled successfully (deployment simulated).", contract_name_to_compile),
                contract_name: Some(contract_name_to_compile),
                contract_address: Some("0xSIMULATED_ADDRESS_AFTER_COMPILE_AND_REMAP".to_string()), // Updated placeholder
                abi: Some(comp_output.abi),
                bytecode: Some(comp_output.bytecode),
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
