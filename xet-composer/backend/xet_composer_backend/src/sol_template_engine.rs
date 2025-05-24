use tera::{Context, Tera};
use std::path::{Path, PathBuf};
use serde::Serialize; // Required for context

// Error type for this module
#[derive(Debug)]
pub enum TemplateError {
    IoError(std::io::Error),
    TeraError(tera::Error),
    TemplateNotFound(String),
}

impl From<std::io::Error> for TemplateError {
    fn from(err: std::io::Error) -> TemplateError {
        TemplateError::IoError(err)
    }
}

impl From<tera::Error> for TemplateError {
    fn from(err: tera::Error) -> TemplateError {
        TemplateError::TeraError(err)
    }
}

pub struct SolTemplateEngine {
    tera: Tera,
    contracts_dir: PathBuf,
}

impl SolTemplateEngine {
    /// Creates a new SolTemplateEngine.
    /// The `contracts_base_dir` should be the path to the `contracts` directory
    /// relative to where the backend executable will be run, or an absolute path.
    /// For this project, it will likely be `../../contracts` if running from `xet-composer/backend/target/debug/`.
    /// Or, more robustly, pass an absolute path or one relative to the crate root during initialization.
    pub fn new(contracts_base_dir: PathBuf) -> Result<Self, TemplateError> {
        // Adjust the path to point to where .sol.tera files will be.
        // This assumes `contracts_base_dir` IS the directory containing .sol.tera files.
        let templates_glob = contracts_base_dir.join("*.sol.tera");
        
        let tera_instance = match Tera::new(templates_glob.to_str().unwrap()) {
            Ok(t) => t,
            Err(e) => return Err(TemplateError::TeraError(e)),
        };
        Ok(Self { tera: tera_instance, contracts_dir: contracts_base_dir })
    }

    /// Renders a Solidity contract template.
    /// `template_name` should be the filename, e.g., "TokenVesting.sol.tera".
    /// `params` should be a serializable struct or a `serde_json::Value` that can be converted to `tera::Context`.
    pub fn render_template<S: Serialize>(
        &self,
        template_name: &str,
        params: &S,
    ) -> Result<String, TemplateError> {
        let context = Context::from_serialize(params)
            .map_err(TemplateError::TeraError)?;

        if self.tera.get_template_names().find(|&name| name == template_name).is_none() {
            return Err(TemplateError::TemplateNotFound(template_name.to_string()));
        }

        self.tera.render(template_name, &context)
            .map_err(TemplateError::TeraError)
    }
}

// Example usage (will be integrated into main.rs later)
/*
fn example() {
    // This path needs to be correctly set based on where the binary runs
    // or by making it configurable.
    // For development, if running `cargo run` from `xet-composer/backend/xet_composer_backend`,
    // this would be `../../contracts`.
    let contracts_path = PathBuf::from("../../contracts"); 
    match SolTemplateEngine::new(contracts_path) {
        Ok(engine) => {
            let mut context_params = serde_json::Map::new();
            context_params.insert("beneficiary".to_string(), serde_json::json!("0xADDRESS"));
            context_params.insert("start_time".to_string(), serde_json::json!(1700000000));
            // ... add other params

            match engine.render_template("TokenVesting.sol.tera", &context_params) {
                Ok(rendered_solidity) => {
                    println!("Rendered Solidity:
{}", rendered_solidity);
                }
                Err(e) => {
                    eprintln!("Error rendering template: {:?}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error creating template engine: {:?}", e);
        }
    }
}
*/
