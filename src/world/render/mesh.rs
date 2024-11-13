use macroquad::prelude::{Mesh, Texture2D};
use crate::world::CHUNK_SIZE_16;
use crate::world::render::{BlockPos, ChunkModel};

pub fn build_model_meshes(
    model: ChunkModel,
    atlas: Option<Texture2D>,
) -> Vec<Mesh> {
    let mut ans= vec![];
    let atlas = atlas.unwrap();
    if let Some(layers) = model.0{
        for (y, layer) in layers.iter().enumerate() {
            for x in 0..CHUNK_SIZE_16 {
                for z in 0..CHUNK_SIZE_16 {
                    let block = layer.get(x,z);
                    let block_meshes = block.get_meshes(&atlas, BlockPos {
                        x: x as isize,
                        y: y as isize,
                        z: z as isize,
                    });
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