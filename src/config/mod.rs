use glam::Vec3;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::android_wrapper::{get_config_file, save_config_file};
use crate::saber_game_loop::SaberOffsets;

#[derive(Serialize, Deserialize)]
pub struct ConfigFile {
    pub saber_offsets: SaberOffsets,
}

impl ConfigFile {
    pub fn get() -> Result<Self> {
        let mut config_file = get_config_file()?;
        let possible = serde_json::from_str(config_file.as_str());
        match possible {
            Ok(file) => {
                Ok(file)
            }
            Err(error) => {
                Ok(Self{ saber_offsets: Default::default() })
            }
        }
    }
    pub fn set(&self) -> Result<()>{
        let as_str = serde_json::to_string(self)?;
        save_config_file(as_str)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct MatrixGroup {
    pos: Vec3,
    rot: Vec3,
    scale: Vec3,
}

