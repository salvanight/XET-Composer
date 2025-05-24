use axum::{routing::post, Router, Json};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

mod sol_template_engine; // Added module
mod deploy_engine; // Add this line

// Define the request structure for the /api/deploy endpoint
#[derive(Deserialize, Debug)]
struct DeployRequest {
    contract: String,
    params: serde_json::Value, // Using serde_json::Value for flexible parameters
}

// Define the response structure for the /api/deploy endpoint
#[derive(Serialize, Debug)]
struct DeployResult {
    success: bool,
    message: String,
    contract_address: Option<String>,
    abi: Option<serde_json::Value>, // ABI can be complex, so Value is suitable
}

// The handler for the /api/deploy endpoint
async fn deploy_handler(Json(payload): Json<DeployRequest>) -> Json<DeployResult> {
    // For now, this is a placeholder.
    // Actual logic for template rendering, compilation, and deployment will be added later.
    println!("Received deploy request for contract: {}", payload.contract);
    println!("Params: {:?}", payload.params);

    // Simulate a successful deployment for now
    Json(DeployResult {
        success: true,
        message: format!("Contract '{}' processed (simulated).", payload.contract),
        contract_address: Some("0x1234567890abcdef1234567890abcdef12345678".to_string()),
        abi: Some(serde_json::json!({"temp_abi": "coming_soon"})),
    })
}

#[tokio::main]
async fn main() {
    // Define the application routes
    let app = Router::new()
        .route("/api/deploy", post(deploy_handler));

    // Define the server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Backend server listening on {}", addr);

    // Run the server
    if let Err(e) = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
    {
        eprintln!("Server error: {}", e);
    }
}
