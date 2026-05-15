
use bevy::app::plugin_group;

mod assets;
mod chunks;
mod mobs;
mod mods;
mod players;

plugin_group! {

    pub struct AgamEnginePlugins {
        assets:::AgamAssetPlugin,
        mods:::AgamModPlugin,
    }
}
