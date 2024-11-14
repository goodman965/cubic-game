use super::*;
use crate::world::render::render_cube_byte::RenderCubeByte;

pub fn build_chunk_model(
    player_pos: Vec3, player_front: Vec3,
    chunk: &Chunk,
) -> Option<ChunkModel> {
    let chunk_pos: WorldPos = chunk.get_pos();
    let chunk_center_pos: Vec3 = vec3(
        chunk_pos.x + CHUNK_SIZE_16 as f32 / 2.0,
        chunk_pos.y + CHUNK_SIZE_16 as f32 / 2.0,
        chunk_pos.z + CHUNK_SIZE_16 as f32 / 2.0,
    );

    let chunk_view_vec: Vec3 = chunk_center_pos - player_pos;
    let angle: f32 = player_front.angle_between(chunk_view_vec);
    let distance: f32 = player_pos.distance(chunk_center_pos);
    if (angle > 65f32.to_radians()) || distance > 20.0 * CHUNK_SIZE_16 as f32 {
        return None;
    }

    let mut this_chunk_model = ChunkModel::default();
    let chunk_x = chunk_pos.x;
    let chunk_y = chunk_pos.y;
    let chunk_z = chunk_pos.z;
    let mut block_pos = WorldPos { x: chunk_x, y: chunk_y, z: 0.0 };
    for y in 0..CHUNK_SIZE_16 {
        block_pos.y += 1.0;
        for x in 0..CHUNK_SIZE_16 {
            block_pos.x += 1.0;
            for z in 0..CHUNK_SIZE_16 {
                let block_state: &BlockState = chunk.get(x, y, z);
                let mut render_byte = RenderCubeByte::from_block_type(&block_state.block_type);
                if render_byte.is_nothing() || chunk.is_isolated(x, y, z) {
                    continue;
                }
                let neighbours = chunk.get_neighbours(x, y, z);
                render_byte.apply_other_negative(neighbours);
                if render_byte.is_nothing() {
                    continue;
                }
                block_pos.z = z as f32 + chunk_z;
                let visible = Chunk::get_block_visible_sides(block_pos, player_pos);
                render_byte.apply_other(visible);
                if render_byte.is_nothing() {
                    continue;
                }
                let textures = block_state.get_texture_set();
                let block_model = BlockModel { render_byte, textures };
                this_chunk_model.set(x, y, z, block_model);
            }
        }
    }
    Some(this_chunk_model)
}
