use super::*;
use crate::world::render::render_cube_byte::RenderCubeByte;

pub fn build_chunk_model(chunk: &mut Chunk) {
    let mut this_chunk_model = ChunkModel::new_empty();
    for y in 0..CHUNK_SIZE_16 {
        for x in 0..CHUNK_SIZE_16 {
            for z in 0..CHUNK_SIZE_16 {
                let block_type: BlockType = chunk.get(x, y, z).block_type;
                let mut render_byte = RenderCubeByte::from_block_type(block_type);
                if render_byte.is_nothing() || chunk.is_isolated(x as isize, y as isize, z as isize) {
                    continue;
                }
                let neighbours = chunk.get_neighbours(x as isize, y as isize, z as isize);
                render_byte.apply_other_negative(neighbours);
                if render_byte.is_nothing() {
                    continue;
                }
                let block_model = BlockModel { render_byte, block_type };
                this_chunk_model.set(x, y, z, block_model);
            }
        }
    }
    chunk.model = this_chunk_model;
}


pub fn update_chunk_model(chunk: &mut Chunk, player_pos: Vec3) {
    let chunk_pos: WorldPos = chunk.get_pos();
    for y in 0..CHUNK_SIZE_16 {
        for x in 0..CHUNK_SIZE_16 {
            for z in 0..CHUNK_SIZE_16 {
                let neighbours = &chunk.get_neighbours(x as isize, y as isize, z as isize);
                if let Some(mut render_byte) = chunk.model.get_render_byte(x, y, z) {
                    render_byte.apply_other_negative(*neighbours);
                    if render_byte.is_nothing() {
                        continue;
                    }
                    let block_pos = WorldPos {
                        x: x as f32 + chunk_pos.x,
                        y: y as f32 + chunk_pos.y,
                        z: z as f32 + chunk_pos.z,
                    };
                    let visible = Chunk::get_block_visible_sides(block_pos, player_pos);
                    render_byte.apply_other(visible);
                    chunk.model.set_render_byte(x, y, z, render_byte)
                }
            }
        }
    }
}
