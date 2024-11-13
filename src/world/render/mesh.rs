use macroquad::prelude::{Mesh, Texture2D};
use crate::world::CHUNK_SIZE_16;
use crate::world::render::{ChunkModel, WorldPos};

pub fn build_model_meshes(
    model: ChunkModel,
    atlas: Option<Texture2D>,
    chunk_pos: WorldPos
) -> Vec<Mesh> {
    let mut ans= vec![];
    let atlas = atlas.unwrap();
    let mut pos = WorldPos {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    if let Some(layers) = model.0{
        let dx = chunk_pos.x;
        let dy = chunk_pos.y;
        let dz = chunk_pos.z;
        for (y, layer) in layers.iter().enumerate() {
            pos.y = y as f32 + dy;
            for x in 0..CHUNK_SIZE_16 {
                pos.x = x as f32 + dx;
                for z in 0..CHUNK_SIZE_16 {
                    let block = layer.get(x,z);
                    pos.z = z as f32 + dz;
                    let block_meshes = block.get_meshes(&atlas, pos.clone());
                    ans.extend(block_meshes);
                }
            }
        }
    }
    ans
}

pub const PLANE_IND: [u16; 6] = [
    0, 1, 2,
    0, 3, 2,
];