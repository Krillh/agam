
pub mod file_paths {

    pub const ASSETS_FOLDER: &str = "assets";
    
    pub const APP_DATA_FOLDER: &str = "appdata";
    pub mod app_data_paths {
        pub const PROFILES: &str = "profiles";
    }
    
    pub const PROFILE_DATA_FOLDER: &str = "profile_data";
    pub mod profile_data_paths {
        pub const WORLD_SAVES: &str = "worlds";
        
        pub const SETTINGS: &str = "settings.ron";
    }

    pub const MOD_FOLDER: &str = "mods";
    pub mod mod_file_paths {
        pub const BLOCK_TEXTURES: &str = "textures/blocks";
        pub const ITEM_TEXTURES: &str = "textures/items";

        pub const BLOCK_DEFINITIONS: &str = "defs/blocks";

        pub const SCRIPTS: &str = "scripts";
        pub const SETUP_SCRIPT: &str = "scripts/main.rhai";
    }
}
pub mod world {

    /// Denotes the length of one side of a cubic chunk of blocks,
    /// measured in number of blocks. Use `CHUNK_SIZE_2` and `CHUNK_SIZE_3`
    /// for `CHUNK_SIZE_1` squared and cubed.
    pub const CHUNK_SIZE_1: usize = 16;
    pub const CHUNK_SIZE_2: usize = CHUNK_SIZE_1 * CHUNK_SIZE_1;
    pub const CHUNK_SIZE_3: usize = CHUNK_SIZE_1 * CHUNK_SIZE_1 * CHUNK_SIZE_1;

    /// The length of one block measured in generic game distance units.
    pub const BLOCK_SIZE_1: f32 = 1.0;
}
