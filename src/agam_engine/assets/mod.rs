
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt, config::ConfigureLoadingState};

use crate::prelude::*;

mod asset_collections;
mod assets;
mod resources;

#[derive(Default)]
pub struct AgamAssetPlugin;
impl Plugin for AgamAssetPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_state::<AssetLoadingState>()
        .init_asset::<assets::ProfileAsset>()
        .init_asset_loader::<assets::ProfileAssetLoader>()
        .add_loading_state(
            LoadingState::new(AssetLoadingState::Loading)
            .continue_to_state(AssetLoadingState::Done)
            .load_collection::<asset_collections::AgamDataAssets>()
            .finally_init_resource::<resources::ProfileList>()
        )
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AssetLoadingState::Done), test)
        ;
    }
}

fn setup(
    mut commands: Commands,
) {
    commands.set_state(AssetLoadingState::Loading);
    info!("Starting asset loading.");
    
}

fn test(
    profile_list: Res<resources::ProfileList>,
) {
    //info!("Loaded profiles: {:#?}", profile_list.into_inner());
}

#[derive(Debug, Default, States, Clone, Copy, PartialEq, Eq, Hash)]
enum AssetLoadingState {
    #[default]
    Waiting,
    Loading,
    Done,
}
