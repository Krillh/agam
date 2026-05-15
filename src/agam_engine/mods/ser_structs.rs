use serde::{Deserialize, Serialize, self};



/// A block definition designed to be erganomic for modders using RON.
#[derive(Debug, Serialize, Deserialize)]
pub struct BlockDefinition {
    pub string_id: String,
    pub name: String,

    pub mesh: BlockMesh,

    #[serde(default)]
    pub collider: Collider,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum BlockMesh {
    Empty,
    SymmetricStandardCube(String),
    RadiallySymmetricStandardCube {
        up: String,
        down: String,
        sides: String,
    },
}


#[derive(Debug, Default, Serialize, Deserialize)]
pub enum Collider {
    #[default]
    StandardCube,
}


