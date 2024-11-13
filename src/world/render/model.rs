use super::*;
use crate::world::render::render_cube_byte::RenderCubeByte;

#[rustfmt::skip]
pub fn build_chunk_model(
    player_pos: Vec3, player_front: Vec3, chunk_pos: ChunkPos, 
    chunk: &Chunk
) -> ChunkModel {
    let chunk_pos: BlockPos = chunk_pos.into();
    let ch_pos: Vec3 = vec3(
        chunk_pos.x as f32 + CHUNK_SIZE_16 as f32 / 2.0,
        chunk_pos.y as f32 + CHUNK_SIZE_16 as f32 / 2.0,
        chunk_pos.z as f32 + CHUNK_SIZE_16 as f32 / 2.0,
    );

    let chunk_view_vec: Vec3 = ch_pos - player_pos;
    let angle: f32 = player_front.angle_between(chunk_view_vec);
    let distance: f32 = player_pos.distance(ch_pos);
    if (angle > 65f32.to_radians()) && distance > 2. * CHUNK_SIZE_16 as f32 {
        return ChunkModel::EMPTY;
    }

    let mut this_chunk_model = ChunkModel::default();
    for y in 0..CHUNK_SIZE_16 {
        for x in 0..CHUNK_SIZE_16 {
            for z in 0..CHUNK_SIZE_16 {
                let block_state: &BlockState = chunk.get(x, y, z);
                let mut render_byte = RenderCubeByte::from_block_type(&block_state.block_type);
                if render_byte.is_nothing() {
                    continue;
                }
                let textures = block_state.get_texture_set();
                let neighbours = chunk.get_neighbours(x, y, z);
                let block_pos = chunk_pos + BlockPos { x: x as isize, y: y as isize, z: z as isize };
                let visible = Chunk::get_block_visible_sides(block_pos, player_pos);
                render_byte.apply_other(visible);
                render_byte.apply_other_negative(neighbours);
                let block_model= BlockModel { render_byte, textures };
                this_chunk_model.set(x, y, z, block_model);
            }
        }
    }
    this_chunk_model
}
