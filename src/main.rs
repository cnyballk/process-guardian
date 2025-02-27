mod config;
mod guardian;

use std::error::Error;
use config::Config;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    
    let config_content = fs::read_to_string("config.json")?;
    let config: Config = serde_json::from_str(&config_content)?;
    
    let guardian = guardian::Guardian::new(config);
    guardian.start()?;

    Ok(())
}
