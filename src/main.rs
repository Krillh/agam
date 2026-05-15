#![feature(path_trailing_sep)]

use bevy::log::LogPlugin;

pub mod prelude;
mod agam_engine;

fn main() {
    use crate::prelude::*;

    let mut app = App::new();

    // * Plugins
    app
    .add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(LogPlugin {
                level: bevy::log::Level::DEBUG,
                filter: "info,naga=warn,wgpu_core=warn,wgpu_hal=warn,agam=debug".into(),
                ..default()
            })
        )
    .add_plugins(agam_engine::AgamEnginePlugins)
    ;

    app.run();

    info!("agam has stopped.");

}
