
use std::path::PathBuf;
use std::str::FromStr;

use bevy::ecs::system::SystemState;

use crate::agam_engine::assets::asset_collections::AgamDataAssets;
use crate::agam_engine::assets::assets::ProfileAsset;
use crate::agam_engine::mods::ModId;
use crate::prelude::*;

#[derive(Resource, Debug)]
pub struct ProfileList {
    profiles: Vec<Profile>,
}
impl FromWorld for ProfileList {
    fn from_world(world: &mut World) -> Self {
        let mut system_state = SystemState::<(
            Res<AgamDataAssets>,
            Res<Assets<ProfileAsset>>,
        )>::new(world);
        let (
            data_assets,
            profile_assets,
        ) = system_state.get(world);

        let data = data_assets.into_inner();

        // v---------- Data Processing -----------v
        
        let mut profiles = Vec::new();
        for profile_handle in &data.profiles {
            let profile_option = profile_assets
                .get(profile_handle.id())
                .and_then(|raw_profile| {
                    let name = raw_profile.name.clone();
                    debug!("Validating profile: `{name}`");
                    let mods = raw_profile.mods.iter().map(|m| ModId::from_str(m).ok()).collect::<Option<Vec<ModId>>>()?;
                    let world_folder = Some(raw_profile.data_folder.clone())
                        .and_then(|fp| {
                                let profile_data_path = consts::file_paths::PROFILE_DATA_FOLDER;
                                let path = PathBuf::from(profile_data_path).join(fp);
                                if std::fs::exists(&path).ok()? { Some(path) } else { None }
                            }
                        )?;
                    
                    Some(Profile {
                        name,
                        mods,
                        data_folder: world_folder,
                    })
                }
            );

            match profile_option {
                Some(profile) => profiles.push(profile),
                None => warn!("An error occured while validating profile. Skipping..."),
            }
        }

        info!("Sucessfully loaded {} of {} profiles.", profiles.len(), data.profiles.len());

        return ProfileList {
            profiles,
        }
    }
}

#[derive(Debug)]
pub struct Profile {
    pub name: String,
    pub mods: Vec<ModId>,
    pub data_folder: PathBuf,
}
