use crate::dict::to_bits;

pub const WORD_BONUSES: [u8; 13] = [0, 0, 0, 0, 1, 3, 5, 8, 10, 12, 14, 15, 16];

pub const IDEAL_POINTS: [u8; 37] = [
    0, 0, 0, 0, 1, 3, 5, 8, 10, 12, 14, 15, 16, 16, 16, 18, 20, 22, 24, 26, 28, 29, 30, 31, 32, 32,
    34, 36, 38, 40, 42, 43, 44, 45, 46, 47, 48,
];

pub const COLUMN_MASKS: [u64; 8] = [
    0x0101010101010101,
    0x0202020202020202,
    0x0404040404040404,
    0x0808080808080808,
    0x1010101010101010,
    0x2020202020202020,
    0x4040404040404040,
    0x8080808080808080,
];

pub const LOWEST_BITS: [u64; 8] = [
    0x0100000000000000,
    0x0200000000000000,
    0x0400000000000000,
    0x0800000000000000,
    0x1000000000000000,
    0x2000000000000000,
    0x4000000000000000,
    0x8000000000000080,
];

pub struct Grid {
    pub letters: [usize; 64],
    pub adjacency: [u64; 64],
    pub adjacency_ords: [usize; 64],
    pub remaining: u64,
}

impl Grid {
    pub fn new(s: &str) -> Self {
        let mut idx = 0;
        let mut letters = [0; 64];
        let mut remaining = 0;

        for l in s.chars() {
            match l {
                '\n' => {
                    if idx % 8 != 0 {
                        idx += 8 - idx % 8;
                    }
                }
                ' ' => {
                    idx += 1;
                }
                _ => {
                    letters[idx] = to_bits(l) as usize;
                    remaining |= 1 << idx;
                    idx += 1;
                }
            }
        }

        let adjacency = [0; 64];
        let adjacency_ords = [0; 64];

        let mut grid = Self {
            letters,
            adjacency,
            adjacency_ords,
            remaining,
        };

        grid.recompute();
        grid
    }

    pub fn recompute(&mut self) {
        for i in 0..64 {
            let bit = 1 << i;
            if bit & self.remaining == 0 {
                continue;
            }

            let col_mask = COLUMN_MASKS[i % 8];
            let mut adjacency = 0;

            let mut bit = 1 << i;
            while bit != 0 {
                bit >>= 8;
                if self.remaining & bit != 0 {
                    adjacency |= bit;
                    break;
                }
            }

            let mut bit = 1 << i;
            let mut col_idx = 0;
            while bit != 0 {
                bit <<= 8;
                if self.remaining & bit != 0 {
                    if col_idx == 0 {
                        adjacency |= bit;
                    }
                    col_idx += 1;
                }
            }

            let mut col_bit = LOWEST_BITS[i % 8];
            let mut column = col_mask;
            loop {
                col_bit >>= 1;
                column >>= 1;

                if column & 0xFF00000000000000 == 0 {
                    break;
                }

                if column & self.remaining != 0 {
                    let mut adj_col_idx = 0;
                    while col_bit != 0 {
                        adjacency |= (col_bit & self.remaining)
                            * (i32::abs(adj_col_idx - col_idx) <= 1) as u64;
                        adj_col_idx += ((col_bit & self.remaining) != 0) as i32;
                        col_bit >>= 8;
                    }
                    break;
                }
            }

            let mut col_bit = LOWEST_BITS[i % 8];
            let mut column = col_mask;
            loop {
                col_bit <<= 1;
                column <<= 1;

                if column & 0xFF == 0 {
                    break;
                }

                if column & self.remaining != 0 {
                    let mut adj_col_idx = 0;
                    while col_bit != 0 {
                        adjacency |= (col_bit & self.remaining)
                            * (i32::abs(adj_col_idx - col_idx) <= 1) as u64;
                        adj_col_idx += ((col_bit & self.remaining) != 0) as i32;
                        col_bit >>= 8;
                    }
                    break;
                }
            }

            self.adjacency[i] = adjacency;
            let mut adjacency_ords = 0;
            while adjacency != 0 {
                let zeros = adjacency.trailing_zeros() as usize;
                let adj_bit = 1 << zeros;
                adjacency ^= adj_bit;
                adjacency_ords |= 1 << self.letters[zeros];
            }

            self.adjacency_ords[i] = adjacency_ords;
        }
    }

    pub fn flip(&mut self, seq: u64) {
        self.remaining ^= seq;
    }
}
