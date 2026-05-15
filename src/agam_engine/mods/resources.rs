
use std::path::PathBuf;

use bevy::platform::collections::{HashMap, HashSet};
use serde::Deserialize;

use crate::prelude::*;
use super::mods_helper::{ModId, ModMetadata};

#[derive(Resource, Default)]
pub struct ModList (pub HashMap<ModId, (PathBuf, ModMetadata)>);

#[derive(Resource, Default)]
pub struct ModLoadingTracker {
    pub loading_mods: HashSet<ModId>,
    pub loaded_mods: HashSet<ModId>,
    pub loading_assets: HashSet<UntypedHandle>,
    pub loaded_assets: HashSet<UntypedHandle>,
}


