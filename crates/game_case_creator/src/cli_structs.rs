use std::path::{PathBuf};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct AppConfig{
    pub region: String,
    pub default_browse_directory: PathBuf
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            region: "United States".to_string(),
            default_browse_directory: PathBuf::from(
                shellexpand::tilde("~")
                .to_string()
            ),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum BuildStep {
    #[default]
    RomSelection,
    ManualSelection,
    //RomhackSelection,
    //ImageSelection,
    //VideoSelection,
    //MetaDataInput
}

#[derive(Debug, Default, Clone)]
pub struct BuildState {
    pub current_step: BuildStep,
    pub staged_roms: Vec<PathBuf>,
    pub staged_manuals: Vec<PathBuf>,
    pub staged_rom_hacks: Vec<PathBuf>,
    pub staged_images: Vec<PathBuf>,
    pub staged_videos: Vec<PathBuf>,
}

#[derive(Default, Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub build_state: BuildState,
}