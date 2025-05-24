use axum::{routing::post, Router, Json};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::env; // Added for env::var

// Existing module declarations
mod sol_template_engine;
mod deploy_engine;
mod artifact_storage; // Added new module
mod kyc; // Added KYC module

// Use statements for our modules
use sol_template_engine::{SolTemplateEngine, TemplateError};
use deploy_engine::{DeployEngine, CompiledArtifact, DeployError}; // Changed CompilationOutput to CompiledArtifact
use crate::artifact_storage::store_artifact; // Added for storing artifacts
use crate::kyc::simulate_kyc_validation; // Added for KYC
use chrono::Utc; // Added for timestamp
use serde::Serialize; // Ensure Serialize is in scope for FrontendDeployResponse

#[derive(Deserialize, Debug)]
struct DeployRequest {
    contract: String, // e.g., "TokenVesting.sol.tera"
    params: serde_json::Value,
}

// New response structure for the frontend
#[derive(Serialize, Debug)]
struct FrontendDeployResponse {
    contract: String,
    address: String,
    abi: String,
    deployed_at: i64,
}

async fn deploy_handler(Json(payload): Json<DeployRequest>) -> Json<FrontendDeployResponse> {
    println!("Received deploy request for contract template: {}", payload.contract);
    println!("Params: {:?}", payload.params);

    // --- KYC Validation ---
    // Perform KYC check with hardcoded values early in the handler.
    if let Err(kyc_error_msg) = simulate_kyc_validation(
        "Test User",
        "0x1234567890123456789012345678901234567890",
        "testhash",
    ) {
        eprintln!("KYC validation failed: {}", kyc_error_msg);
        return Json(FrontendDeployResponse {
            contract: String::new(), // Or payload.contract if you want to return the requested contract name
            address: String::new(),
            abi: String::new(),
            deployed_at: 0,
        });
    }
    println!("KYC validation successful."); // Optional: log success

    // --- Configuration ---
    let contracts_base_dir = PathBuf::from("../../contracts") // Relative to executable in target/debug or if run from workspace root
        .canonicalize()
        .unwrap_or_else(|e| {
            eprintln!("Failed to canonicalize contracts_base_dir: {:?}. Using relative path.", e);
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
            // Return a default error response
            return Json(FrontendDeployResponse {
                contract: payload.contract,
                address: String::new(),
                abi: String::new(),
                deployed_at: 0,
            });
        }
    };

    let deploy_engine = DeployEngine::new(solc_executable.clone());

    let rendered_solidity = match template_engine.render_template(&payload.contract, &payload.params) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Failed to render template {}: {:?}", payload.contract, e);
            return Json(FrontendDeployResponse {
                contract: payload.contract,
                address: String::new(),
                abi: String::new(),
                deployed_at: 0,
            });
        }
    };

    let contract_name_to_compile = payload.contract.replace(".sol.tera", "");

    match deploy_engine.compile_solidity(&rendered_solidity, &contract_name_to_compile, &contracts_base_dir, &solc_remappings) {
        Ok(comp_output) => {
            println!("Compilation successful for {}", comp_output.contract_name);
            
            // Simulate deployment address (as it was before)
            let simulated_address = "0xSIMULATED_ADDRESS_MAIN_RS".to_string(); // Placeholder
            let deployed_at_ts = Utc::now().timestamp();

            // Store the artifact
            // For now, log error from store_artifact and continue. 
            // A more robust solution would involve returning an error response to the client.
            if let Err(e) = store_artifact(&comp_output, &simulated_address) {
                eprintln!("Failed to store artifact for {}: {:?}", comp_output.contract_name, e);
                // Depending on requirements, you might want to return an error here.
                // For now, we proceed to return success response as compilation & simulation were okay.
            }

            Json(FrontendDeployResponse {
                contract: comp_output.contract_name.clone(),
                address: simulated_address,
                abi: comp_output.abi.clone(), // ABI is now String
                deployed_at: deployed_at_ts,
            })
        }
        Err(e) => {
            eprintln!("Failed to compile Solidity for {}: {:?}", contract_name_to_compile, e);
            // Return a default error response
            Json(FrontendDeployResponse {
                contract: contract_name_to_compile, // Use the name we tried to compile
                address: String::new(),
                abi: String::new(),
                deployed_at: 0,
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
