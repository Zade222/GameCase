use std::path::{PathBuf};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct BoxerConfig{
    pub region: String,
    pub default_browse_directory: PathBuf
}

impl Default for BoxerConfig {
    fn default() -> Self {
        BoxerConfig {
            region: "United States".to_string(),
            default_browse_directory: PathBuf::from(
                shellexpand::tilde("~")
                .to_string())
        }
    }
}
