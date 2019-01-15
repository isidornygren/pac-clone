use std::io;

#[derive(Debug)]
pub struct Map<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<T>>,
    pub default: T,
}

static SQUARE_VALUES: [[u16; 3]; 3] = [[1, 2, 4], [8, 0, 16], [32, 64, 128]];

impl<T> Map<T>
where
    T: PartialEq,
{
    pub fn tile_at(&self, x: usize, y: usize) -> &T {
        if y >= self.height || x >= self.width {
            // Just return a wall outside of the map level
            return &self.default;
        }
        return &self.data[y][x];
    }
    pub fn marching_square_at(&self, x: usize, y: usize, block: T) -> u16 {
        let mut val: u16 = 0;
        for local_y in 0..2 {
            for local_x in 0..2 {
                if ((x + local_x) as i8 - 1) < 0 || ((y + local_y) as i8 - 1) < 0 {
                    // unsigned overflow
                    val += SQUARE_VALUES[local_y][local_x];
                } else {
                    let neighbour = self.tile_at(x + local_x - 1, y + local_y - 1);
                    if *neighbour == block {
                        val += SQUARE_VALUES[local_y][local_x];
                    }
                }
            }
        }
        return val;
    }
}
