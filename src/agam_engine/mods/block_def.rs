
use std::sync::Arc;

use bevy::asset::LoadContext;

use crate::prelude::*;
use super::ser_structs;

#[derive(Asset, TypePath)]
pub struct BlockDefinitionAsset {
    pub string_id: Arc<str>,
    pub name: Arc<str>,
    // pub uuid: Uuid,
    // pub nid: BlockId,

    pub mesh: PreBakedMesh,
}

/// A block mesh that needs to get baked when constructing the block registry.
pub enum PreBakedMesh {
    Empty,
    Cube {
        front: Handle<Image>,
        back: Handle<Image>,
        up: Handle<Image>,
        down: Handle<Image>,
        right: Handle<Image>,
        left: Handle<Image>,
    },
}

impl BlockDefinitionAsset {
    
    /// Converts the mod-friendly block definition into a more computer-friendly one that can then get baked.
    fn from_der_def(load_context: &mut LoadContext, ser_def: ser_structs::BlockDefinition) -> Self {
        BlockDefinitionAsset {
            string_id: ser_def.string_id.into(),
            name: ser_def.name.into(),
            mesh: PreBakedMesh::from_ser_mesh(load_context, ser_def.mesh),
        }
    }
}

impl PreBakedMesh {
    fn from_ser_mesh(load_context: &mut LoadContext, ser_mesh: ser_structs::BlockMesh) -> Self {
        match ser_mesh {
            ser_structs::BlockMesh::Empty => Self::Empty,
            ser_structs::BlockMesh::SymmetricStandardCube(faces) =>
                Self::cube(load_context, &faces, &faces, &faces, &faces, &faces, &faces),
            ser_structs::BlockMesh::RadiallySymmetricStandardCube { up, down, sides } =>
                Self::cube(load_context, &sides, &sides, &up, &down, &sides, &sides),
        }
    }

    /// A helper function for cube meshes, since they're the most common for a block game.
    fn cube(
        load_context: &mut LoadContext,
        front: &str, back: &str,
        up: &str, down: &str,
        right: &str, left: &str
    ) -> Self {
        todo!();
    }
}

// TODO make BlockDefAssetLoader and asset loader implementation






