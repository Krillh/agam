
use std::path::PathBuf;

use bevy::asset::AssetLoader;
use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::prelude::*;


/// A game profile is a set of mods, worlds, and settings.
#[derive(Asset, TypePath, Serialize, Deserialize)]
pub struct ProfileAsset {
    pub name: String,
    pub mods: Vec<String>, // convert to ModId on processing
    pub data_folder: PathBuf, // verify exists on processing
}
#[derive(Default, TypePath)]
pub struct ProfileAssetLoader;
impl AssetLoader for ProfileAssetLoader {
    type Asset = ProfileAsset;
    type Settings = ();
    type Error = AgamAssetLoaderError;
    fn extensions(&self) -> &[&str] { &["ron"] }
    async fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &Self::Settings,
        _load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let custom_asset = ron::de::from_bytes::<ProfileAsset>(&bytes)?;
        Ok(custom_asset)
    }
}


#[non_exhaustive]
#[derive(Debug, Error)]
pub enum AgamAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON](ron) Error
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
}