
use std::path::PathBuf;

use crate::prelude::*;

use super::events::ReloadModList;
use super::mods_helper::ModMetadata;
use super::resources::{ModLoadingTracker, ModList};




/// Reads in all the mod folders and stores their metadata in a resource called `ModList`.
pub fn reload_mod_list(
    _event: On<ReloadModList>,
    mut commands: Commands
) {
    // Read all the mod folders and make a list of all the installed mods.
    let mut had_errors = false;
    let mut mod_list = ModList::default();
    for mod_folder in std::fs::read_dir(consts::file_paths::MOD_FOLDER).unwrap() {
        match mod_folder {
            Ok(folder) => {
                if let Some(mod_metadata) =  try_read_mod_folder(folder) {
                    mod_list.0.insert(
                        mod_metadata.get_mod_id(),
                        (mod_metadata.path.clone(), mod_metadata)
                    );
                } else {
                    had_errors = true;
                }
            },
            Err(err) => {
                warn!("Failed to read mod folder: {err}");
            },
        }
    }
    
    if had_errors {
        error!("Error(s) occured while reading mods.");
    }

    info!("Mods reloaded. {} mods recognized.", mod_list.0.len());
    commands.insert_resource(mod_list);
}

/// helper function
fn try_read_mod_folder(folder: std::fs::DirEntry) -> Option<ModMetadata> {
    debug!("Reading mod folder `{}`", &folder.path().display());
    let mod_toml_path = folder.path().join(PathBuf::from("mod.toml"));
    let mod_toml_file = std::fs::read_to_string(&mod_toml_path)
        .or_else(|e| {warn!("Failed to read mod metadata at {}: {e}", &mod_toml_path.display()); Err(e)}).ok()?;
    let mod_metadata = toml::de::from_str::<ModMetadata>(&mod_toml_file)
        .or_else(|e| {warn!("Failed to parse mod metadataat {}: {e}", &mod_toml_path.display()); Err(e)}).ok()?;
    if !mod_metadata.is_valid() {
        warn!("Mod metadata is invalid at {}.", &mod_toml_path.display());
    }
    return Some(mod_metadata.with_path(folder.path()));
}