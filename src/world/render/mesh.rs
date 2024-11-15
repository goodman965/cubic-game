use crate::world::render::{ChunkModel, WorldPos};
use crate::world::CHUNK_SIZE_16;
use macroquad::prelude::{Mesh, Texture2D};
pub fn build_model_meshes(
    model: &ChunkModel,
    atlas: Texture2D,
    chunk_pos: WorldPos
) -> Vec<Mesh> {
    let mut ans= vec![];
    if let Some(layers) = model.0{
        println!("Some");
        for (y, layer) in layers.iter().enumerate() {
            for x in 0..CHUNK_SIZE_16 {
                for z in 0..CHUNK_SIZE_16 {
                    let block = layer.get(x,z);
                    if !block.render_byte.is_nothing() {
                        let mut pos = WorldPos {
                            x: x as f32 + chunk_pos.x,
                            y: y as f32 + chunk_pos.y,
                            z: z as f32 + chunk_pos.z,
                        };
                    let block_meshes = block.get_meshes(&atlas, pos.clone());
                    ans.extend(block_meshes);
                    }
                }
            }
        }
    }
    ans
}


fn has_same_texture(m1: Mesh, m2: Mesh) -> bool {
    m1.texture.eq(&m2.texture)
}
pub const PLANE_IND: [u16; 6] = [
    0, 1, 2,
    0, 3, 2,
];
