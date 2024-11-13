use std::fmt::Debug;
use std::ops::Add;

use derive_more::{Deref, DerefMut};
use macroquad::prelude::*;

use super::*;

pub mod mesh;
pub mod model;
pub mod render_cube_byte;

#[derive(Default, Clone, PartialEq)]
pub struct ChunkModel(Option<[ModelLayer; CHUNK_SIZE_16]>);

#[allow(dead_code)]
impl ChunkModel {
    pub const EMPTY: ChunkModel = ChunkModel(None);

    pub fn set(&mut self, x: usize, y: usize, z: usize, model: BlockModel) {
        match &mut self.0 {
            Some(arr) => *arr[y].get_mut(x, z) = model,
            None => {
                let mut arr = [ModelLayer::EMPTY; CHUNK_SIZE_16];
                *arr[y].get_mut(x, z) = model;

                self.0 = Some(arr);
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        match &self.0 {
            Some(arr) => arr.iter().all(|layer| layer.is_empty()),
            None => true,
        }
    }
}

#[derive(Default, Clone, PartialEq, Deref, DerefMut)]
pub struct ModelLayer(pub [[BlockModel; CHUNK_SIZE_16]; CHUNK_SIZE_16]);

#[allow(dead_code)]
impl ModelLayer {
    pub const EMPTY: ModelLayer = ModelLayer([const{[const{BlockModel::EMPTY};CHUNK_SIZE_16]};CHUNK_SIZE_16]);

    pub fn get(&self, x: usize, z: usize) -> &BlockModel {
        &self.0[x][z]
    }

    pub fn get_mut(&mut self, x: usize, z: usize) -> &mut BlockModel {
        &mut self.0[x][z]
    }

    pub fn is_empty(&self) -> bool {
        *self == Self::EMPTY
    }
}

struct ChunkPlusConnected<'ch, 'to, 'bo, 'px, 'nx, 'pz, 'nz> {
    chunk: &'ch Chunk,
    top: &'to ChunkLayer,
    bottom: &'bo ChunkLayer,
    px: &'px ChunkLayer,
    nx: &'nx ChunkLayer,
    pz: &'pz ChunkLayer,
    nz: &'nz ChunkLayer,
}

#[rustfmt::skip]
impl<'ch, 'to, 'bo, 'px, 'nx, 'pz, 'nz> ChunkPlusConnected<'ch, 'to, 'bo, 'px, 'nx, 'pz, 'nz> {
    /// (usize, usize, usize) - (x, y, z) pos in chunk 0..16
    #[rustfmt::skip]
    fn connected_blocks(&self, x: usize, y: usize, z: usize) -> ConnectedBlocks {

        let top = if y == 15 { &self.top.get(x, z) } else { &self.chunk.get(x, y + 1, z) };
        let bottom = if y == 0 { &self.bottom.get(x, z) } else { &self.chunk.get(x, y - 1, z) };

        let px = if x == 15 { &self.px.get(x, z) } else { &self.chunk.get(x + 1, y, z) };
        let nx = if x == 0 { &self.nx.get(x, z) } else { &self.chunk.get(x - 1, y, z) };

        let pz = if z == 15 { &self.pz.get(x, z) } else { &self.chunk.get(x, y, z + 1) };
        let nz = if z == 0 { &self.nz.get(x, z) } else { &self.chunk.get(x, y, z - 1) };

        ConnectedBlocks::new(&top, &bottom, &px, &nx, &pz, &nz)
    }
}

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct WorldPos {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}


#[derive(Deref, Clone, Copy, PartialEq)]
pub struct UvTexture(Vec2);

impl UvTexture {
    pub const fn from_n(n: usize) -> UvTexture {
        UvTexture::new(vec2(0., 0.01 * n as f32))
    }
    pub const fn new(inner: Vec2) -> UvTexture {
        Self(inner)
    }

    pub const fn up_left(&self) -> Vec2 {
        self.0
    }

    pub const fn up_right(&self) -> Vec2 {
        vec2(self.0.x + 1., self.0.y + 0.)
    }

    pub const fn low_left(&self) -> Vec2 {
        vec2(self.0.x + 0., self.0.y + 0.01)
    }

    pub const fn low_right(&self) -> Vec2 {
        vec2(self.0.x + 1., self.0.y + 0.01)
    }

    pub const DIRT: UvTexture = UvTexture::from_n(0);
    pub const GRASS_SIDE: UvTexture = UvTexture::from_n(1);
    pub const GRASS_TOP: UvTexture = UvTexture::from_n(2);
    pub const STONE: UvTexture = UvTexture::from_n(3);
    pub const SAND: UvTexture = UvTexture::from_n(4);
    pub fn get_vertices(&self, pos: WorldPos, side: BlockSide) -> Vec<Vertex> {
        let coef = side.get_coef();
        let corners = match side {
            BlockSide::Py => [self.low_left(), self.low_right(), self.up_right(), self.low_right()],
            BlockSide::Ny => [self.low_left(), self.low_right(), self.up_right(), self.low_right()],
            BlockSide::Px => [self.low_left(), self.low_right(), self.up_right(), self.up_left()],
            BlockSide::Nx => [self.low_right(), self.low_left(), self.up_left(), self.up_right()],
            BlockSide::Pz => [self.low_right(), self.low_left(), self.up_left(), self.up_right()],
            BlockSide::Nz => [self.low_right(), self.low_left(), self.up_left(), self.up_right()],
        };
        let (x, y, z) = (pos.x as f32, pos.y as f32, pos.z as f32);
        let mut ans = vec![];
        ans.push(vertex(vec3(coef[0][0] + x, coef[0][1] + y, coef[0][2] + z), corners[0]));
        ans.push(vertex(vec3(coef[1][0] + x, coef[1][1] + y, coef[1][2] + z), corners[1]));
        ans.push(vertex(vec3(coef[2][0] + x, coef[2][1] + y, coef[2][2] + z), corners[2]));
        ans.push(vertex(vec3(coef[3][0] + x, coef[3][1] + y, coef[3][2] + z), corners[3]));
        ans
    }
}

impl Debug for UvTexture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("UvTexture")
            .field(&((self.0.y * 100.) as usize))
            .finish()
    }
}

impl Default for UvTexture {
    fn default() -> Self {
        Self::from_n(3)
    }
}

use macroquad::models::Vertex;
use crate::world::render::mesh::PLANE_IND;
use crate::world::render::render_cube_byte::RenderCubeByte;

const WHITE: [u8; 4] = [u8::MAX, u8::MAX, u8::MAX, u8::MAX];

const fn vertex(pos: Vec3, uv: Vec2) -> Vertex {
    Vertex {
        normal: Vec4::ONE,
        position: pos,
        uv,
        color: WHITE,
    }
}

// #[derive(Debug, Clone, Copy)]
// pub struct BlockPos {
//     pub x: isize,
//     pub y: isize,
//     pub z: isize,
// }

// impl Add for BlockPos {
//     type Output = BlockPos;
//
//     fn add(self, rhs: Self) -> Self::Output {
//         let BlockPos { x, y, z } = self;
//         BlockPos {
//             x: x + rhs.x,
//             y: y + rhs.y,
//             z: z + rhs.z,
//         }
//     }
// }

#[allow(dead_code)]
pub struct ConnectedBlocks<'to, 'bo, 'px, 'nx, 'pz, 'nz> {
    pub top: &'to BlockState,
    pub bottom: &'bo BlockState,
    pub px: &'px BlockState,
    pub nx: &'nx BlockState,
    pub pz: &'pz BlockState,
    pub nz: &'nz BlockState,
}

#[allow(dead_code)]
impl<'to, 'bo, 'px, 'nx, 'pz, 'nz> ConnectedBlocks<'to, 'bo, 'px, 'nx, 'pz, 'nz> {

    const EMPTY: ConnectedBlocks<'static, 'static, 'static, 'static, 'static, 'static> 
                = ConnectedBlocks::new(
                    &BlockState::EMPTY, &BlockState::EMPTY, &BlockState::EMPTY,
                    &BlockState::EMPTY, &BlockState::EMPTY, &BlockState::EMPTY,
                );

    pub const fn new(
        top: &'to BlockState, bottom: &'bo BlockState, 
        px: &'px BlockState, nx: &'nx BlockState, 
        pz: &'pz BlockState, nz: &'nz BlockState,
    ) -> Self {
        Self { top, bottom, px, nx, pz, nz }
    }
}

#[allow(dead_code)]
pub struct ConnectedChunks<'to, 'bo, 'px, 'nx, 'pz, 'nz> {
    pub top: &'to ChunkLayer,
    pub bottom: &'bo ChunkLayer,
    pub px: &'px ChunkLayer,
    pub nx: &'nx ChunkLayer,
    pub pz: &'pz ChunkLayer,
    pub nz: &'nz ChunkLayer,
}

#[allow(dead_code)]
impl<'to, 'bo, 'px, 'nx, 'pz, 'nz> ConnectedChunks<'to, 'bo, 'px, 'nx, 'pz, 'nz> {

    pub const EMPTY: ConnectedChunks<'static, 'static, 'static, 'static, 'static, 'static> 
            = ConnectedChunks::new(
                &ChunkLayer::EMPTY, &ChunkLayer::EMPTY, &ChunkLayer::EMPTY,
                &ChunkLayer::EMPTY, &ChunkLayer::EMPTY, &ChunkLayer::EMPTY,
            );

    pub const fn new(
        top: &'to ChunkLayer, bottom: &'bo ChunkLayer, 
        px: &'px ChunkLayer, nx: &'nx ChunkLayer, 
        pz: &'pz ChunkLayer, nz: &'nz ChunkLayer,
    ) -> Self {
        Self { top, bottom, px, nx, pz, nz }
    }
}

pub enum MyTexture {
    Transparent,
    AllSides(UvTexture),
    Grass {
        top: UvTexture,
        side: UvTexture,
        bottom: UvTexture,
    },
}

#[allow(dead_code)]
impl MyTexture {

    const fn top(&self) -> Option<UvTexture> {
        match self {
            MyTexture::Transparent => None,
            MyTexture::AllSides(texture) => Some(*texture),
            MyTexture::Grass { top, .. } => Some(*top),
        }
    }

    const fn bottom(&self) -> Option<UvTexture> {
        match self {
            MyTexture::Transparent => None,
            MyTexture::AllSides(texture) => Some(*texture),
            MyTexture::Grass { bottom, .. } => Some(*bottom),
        }
    }

    const fn px(&self) -> Option<UvTexture> {
        match self {
            MyTexture::Transparent => None,
            MyTexture::AllSides(texture) => Some(*texture),
            MyTexture::Grass { side, .. } => Some(*side),
        }
    }

    const fn pz(&self) -> Option<UvTexture> {
        self.px()
    }

    const fn nx(&self) -> Option<UvTexture> {
        self.px()
    }

    const fn nz(&self) -> Option<UvTexture> {
        self.px()
    }
}

// TODO: Make texture depend on connected block
// For example dirt will need to be merged with gravel etc.
#[allow(dead_code)]
const fn my_texture(bs: &BlockState, _conn: &ConnectedBlocks) -> MyTexture {
    match bs.block_type {
        BlockType::Air => MyTexture::Transparent,
        BlockType::Dirt => MyTexture::AllSides(UvTexture::DIRT),
        BlockType::Grass => MyTexture::Grass {
            top: UvTexture::GRASS_TOP,
            side: UvTexture::GRASS_SIDE,
            bottom: UvTexture::DIRT,
        },
        BlockType::Stone => MyTexture::AllSides(UvTexture::STONE),
        BlockType::Sand => MyTexture::AllSides(UvTexture::SAND),
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct BlockModel {
    pub render_byte: RenderCubeByte,
    textures: TextureSet,
}
impl BlockModel {
    const EMPTY: Self = BlockModel {
        render_byte: RenderCubeByte::ALL,
        textures: [None; 6],
    };
}

impl Default for BlockModel {
    fn default() -> Self {
        let textures = [None; 6];
        Self {
            render_byte: RenderCubeByte::from_block_type(&BlockType::Air),
            textures,
        }
    }
}

impl BlockModel {
    pub fn get_meshes(&self, atlas: &Texture2D, block_pos: WorldPos) -> Vec<Mesh> {
        let mut ans = vec![];
        for side_idx in 0..6 {
            if self.render_byte.bool_in_pos(side_idx) {
                if let Some(uv_texture) = self.textures[side_idx] {
                    let side = BlockSide::from_position(side_idx);
                    let vertices = uv_texture.get_vertices(block_pos.clone(), side);
                    ans.push(Mesh {
                        vertices,
                        indices: PLANE_IND.to_vec(),
                        texture: Some(atlas.clone()),
                    })
                }
            }
        }
        ans
    }
}

#[derive(Debug)]
pub enum BlockSide {
    Py,
    Ny,
    Px,
    Nx,
    Pz,
    Nz,
}
impl BlockSide {
    pub fn from_position(pos: usize) -> Self {
        match pos {
            5 => Self::Py,
            4 => Self::Ny,
            3 => Self::Px,
            2 => Self::Nx,
            1 => Self::Pz,
            _=> Self::Nz
        }
    }
    fn get_coef(&self) -> [[f32; 3]; 4] {
        match self {
            BlockSide::Py => [[0.0, 1.0, 0.0], [1.0, 1.0, 0.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0]],
            BlockSide::Ny => [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 1.0], [0.0, 0.0, 1.0]],
            BlockSide::Px => [[1.0, 0.0, 0.0], [1.0, 0.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 0.0]],
            BlockSide::Nx => [[0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0, 1.0], [0.0, 1.0, 0.0]],
            BlockSide::Pz => [[0.0, 0.0, 1.0], [1.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0]],
            BlockSide::Nz => [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]],
        }
    }
}

