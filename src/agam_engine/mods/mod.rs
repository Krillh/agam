
use crate::{agam_engine::mods::{mods_helper::ModMetadata, resources::ModList}, prelude::*};

mod block_def;
mod events;
mod mods_helper;
mod resources;
mod ser_structs;
mod systems;

pub use mods_helper::ModId;

#[derive(Default)]
pub struct AgamModPlugin;
impl Plugin for AgamModPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup)
        .add_observer(systems::reload_mod_list)
        ;
    }
}

fn setup(
    mut commands: Commands,
) {

    // create required folders to ensure that they're present.
    std::fs::create_dir(consts::file_paths::MOD_FOLDER);
    // TODO: handle errors properly.
    // I need to figure out what the exact error conditions for create_dir_all, because I
    // don't want it to panic if the path already exists, but I also want to make sure the
    // paths exist, so there should be a problem if they don't and can't be created.
    // blah, blah, blah...

    commands.trigger(events::ReloadModList);
}



#[derive(Debug, States, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum ModsLoadingState {
    #[default]
    NotLoading,
    ReadingMods,
    Baking,
    Finished,
}
