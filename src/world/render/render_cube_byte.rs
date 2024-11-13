use crate::world::BlockType;

//bits order
//0 0 0 0 0 0 0 0 - _ _ py ny px nx pz nz
#[derive(Default, Clone, PartialEq, Debug)]
pub struct RenderCubeByte(u8);

impl RenderCubeByte {
    pub const NOTHING: Self = RenderCubeByte(0b00000000);
    pub const ALL: Self = RenderCubeByte(0b00111111);
    pub fn is_nothing(&self) -> bool {
        self.0 == 0
    }
    #[allow(dead_code)]
    pub fn get_value(&self) -> u8 {
        self.0
    }
    pub fn bool_in_pos(&self, pos: usize) -> bool {
        self.0 >> pos & 1 == 1
    }

    pub fn apply_other_negative(&mut self, other: Self) {
        let neg_other = !other.0 & 0b00111111;
        self.0 = self.0 & neg_other;
    }
    pub fn apply_other(&mut self, other: Self) {
        self.0 = self.0 & other.0;
    }

    pub fn from_block_type(bt: &BlockType) -> Self {
        let value = match bt {
            BlockType::Air => 0,
            _ => 0b00111111,
        };
        Self(value)
    }
    pub fn set_bit(&mut self, pos: usize, bit: bool) {
        let mask = !(1 << pos);
        let flag = (bit as u8) << pos;
        self.0 = self.0 & mask | flag;
    }

    #[allow(dead_code)]
    pub fn get_bit(&self, pos: usize) -> bool {
        if pos < 8 {
            self.0 & (1 << pos) != 0
        } else {
            false
        }
    }
}

