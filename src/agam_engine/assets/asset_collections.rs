
use bevy_asset_loader::asset_collection::AssetCollection;

use crate::{agam_engine::assets::assets::ProfileAsset, prelude::*};


#[derive(AssetCollection, Resource)]
pub struct AgamDataAssets {
    // ! because of limitations in bevy_asset_loader, this path cannot update from the consts file. must be updated manually!
    #[asset(path = "../appdata/profiles", collection(typed))]
    pub profiles: Vec<Handle<ProfileAsset>>,
}
