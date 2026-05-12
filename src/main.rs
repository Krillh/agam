

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
        )
    .add_plugins(agam_engine::AgamEnginePlugins)
    ;

    app.run();

    info!("agam has stopped.");

}
