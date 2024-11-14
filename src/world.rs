use std::array::from_fn as arr_fn;
use std::ops::Range;
use crate::world::render::render_cube_byte::RenderCubeByte;
use crate::world::render::{UvTexture, WorldPos};
use macroquad::math::Vec3;
use macroquad::prelude::vec3;
// use rand::Rng;""

pub mod render;

pub const CHUNK_SIZE_16: usize = 16;
type TextureSet = [Option<UvTexture>; 6];

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct BlockState {
    pub block_type: BlockType,
}

impl BlockState {
    pub fn get_texture_set(&self) -> TextureSet {
        let mut ts = [None; 6];
        match self.block_type {
            BlockType::Air => {}
            BlockType::Dirt => ts.fill(Some(UvTexture::DIRT)),
            BlockType::Grass => {
                ts[0] = Some(UvTexture::GRASS_TOP);
                ts[1] = Some(UvTexture::DIRT);
                for i in 2..6 {
                    ts[i] = Some(UvTexture::GRASS_SIDE)
                }
            }
            BlockType::Stone => ts.fill(Some(UvTexture::STONE)),
            BlockType::Sand => ts.fill(Some(UvTexture::SAND)),
        }
        ts.reverse(); //to byte order
        ts
    }
}
impl BlockState {
    pub const fn new(block_type: BlockType) -> Self {
        Self { block_type }
    }

    pub const fn is_empty(&self) -> bool {
        self.block_type.is_empty()
    }
}

#[allow(dead_code)]
impl BlockState {
    pub const AIR: BlockState = BlockState::new(BlockType::Air);
    pub const EMPTY: BlockState = BlockState::new(BlockType::Air);

    pub const STONE: BlockState = BlockState::new(BlockType::Stone);
    pub const DIRT: BlockState = BlockState::new(BlockType::Dirt);
    pub const GRASS: BlockState = BlockState::new(BlockType::Grass);
    pub const SAND: BlockState = BlockState::new(BlockType::Sand);
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
#[allow(dead_code)]
pub enum BlockType {
    #[default] Air,
    Dirt,
    Grass,
    Stone,
    Sand,
}

impl BlockType {
    pub const fn is_empty(&self) -> bool {
        matches!(self, BlockType::Air)
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
#[allow(dead_code)]
pub enum Biome {
    #[default] Plains,
    Desert,
    Forest,
    Jungle,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Chunk {
    pub biome: Biome,
    pub blocks: [ChunkLayer; CHUNK_SIZE_16],
    pub pos: WorldPos,
}

#[allow(dead_code)]
impl Chunk {
    const RANGE: Range<usize> = 0..CHUNK_SIZE_16;
    /// (usize, usize, usize) - pos in chunk 0..16
    #[allow(dead_code)]
    pub fn from_fn(mut func: impl FnMut(usize, usize, usize) -> BlockState) -> Chunk {
        Chunk {
            biome: Biome::Plains,
            blocks: arr_fn(|y| ChunkLayer::from_fn(|x, z| func(x, y, z))),
            pos: Default::default(),
        }
    }

    pub fn fill(&mut self, state: BlockState) {
        for y in 0..CHUNK_SIZE_16 {
            self.blocks[y].fill(state.clone());
        }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> &BlockState {
        self.blocks[y].get(x, z)
    }

    fn calc_indexes(x: usize, y: usize, z: usize) -> [[usize; 3]; 6] {
        [
            [x, y, z - 1], //nz
            [x, y, z + 1], //pz
            [x - 1, y, z], //nx
            [x + 1, y, z], //px
            [x, y - 1, z], //ny
            [x, y + 1, z], //py
        ]
    }
    pub fn is_isolated(&self, x: usize, y: usize, z: usize) -> bool {
        let indexes = Self::calc_indexes(x, y, z);
        for i in 0..6 {
            if Self::RANGE.contains(&indexes[i][0]) &&
                Self::RANGE.contains(&indexes[i][1]) &&
                Self::RANGE.contains(&indexes[i][2])
            {
                if !self.get(x, y, z).is_empty() {
                    return false;
                }
            }
        }
        true
    }
    pub fn get_neighbours(&self, x: usize, y: usize, z: usize) -> RenderCubeByte {
        let mut ans = RenderCubeByte::NOTHING;
        let indexes = Self::calc_indexes(x, y, z);
        for i in 0..6 {
            ans.set_bit(i, self.has_visible_block_at(indexes[i][0], indexes[i][1], indexes[i][2]));
        }
        ans
    }
    pub fn has_visible_block_at(&self, x: usize, y: usize, z: usize) -> bool {
        if Self::RANGE.contains(&x) && Self::RANGE.contains(&y) && Self::RANGE.contains(&z) {
            !self.get(x, y, z).is_empty()
        } else {
            false
        }
    }

    pub fn get_block_visible_sides(block_pos: WorldPos, player_pos: Vec3) -> RenderCubeByte {
        let block_pos = vec3(block_pos.x, block_pos.y, block_pos.z);

        let (player_py, player_px, player_pz) = (
            player_pos.y > block_pos.y, player_pos.x > block_pos.x, player_pos.z > block_pos.z
        );
        let mut byte = RenderCubeByte::NOTHING;
        let y_pos = if player_py { 5 } else { 4 };
        let x_pos = if player_px { 3 } else { 2 };
        let z_pos = if player_pz { 1 } else { 0 };
        byte.set_bit(y_pos, true);
        byte.set_bit(x_pos, true);
        byte.set_bit(z_pos, true);
        byte
    }

    pub fn get_mut(&mut self, x: usize, y: usize, z: usize) -> &mut BlockState {
        self.blocks[y].get_mut(x, z)
    }

    pub const EMPTY: Chunk = Chunk {
        biome: Biome::Plains,
        blocks: [ChunkLayer::EMPTY; CHUNK_SIZE_16],
        pos: WorldPos {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    };
    pub fn get_pos(&self) -> WorldPos {
        self.pos
    }
    pub fn set_pos(&mut self, x: f32, y: f32, z: f32) {
        self.pos.x = x;
        self.pos.y = y;
        self.pos.z = z;
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct ChunkLayer(pub [[BlockState; CHUNK_SIZE_16]; CHUNK_SIZE_16]);

#[allow(dead_code)]
impl ChunkLayer {
    pub fn new(inner: [[BlockState; CHUNK_SIZE_16]; CHUNK_SIZE_16]) -> Self {
        Self(inner)
    }

    /// (usize, usize) - (x, z) in chunk layer 0..16
    pub fn from_fn(mut func: impl FnMut(usize, usize) -> BlockState) -> Self {
        Self(arr_fn(|x| arr_fn(|z| func(x, z))))
    }

    pub fn fill(&mut self, state: BlockState) {
        for x in 0..CHUNK_SIZE_16 {
            for z in 0..CHUNK_SIZE_16 {
                *self.get_mut(x, z) = state.clone();
            }
        }
    }

    pub fn get(&self, x: usize, z: usize) -> &BlockState {
        &self.0[x][z]
    }

    pub fn get_mut(&mut self, x: usize, z: usize) -> &mut BlockState {
        &mut self.0[x][z]
    }

    pub fn is_empty(&self) -> bool {
        *self == ChunkLayer::EMPTY
    }

    pub const EMPTY: ChunkLayer =
        ChunkLayer([const { [BlockState::AIR; CHUNK_SIZE_16] }; CHUNK_SIZE_16]);
}
