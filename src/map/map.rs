use std::io;

#[derive(Debug)]
pub struct Map<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<T>>,
}

impl<T> Map<T> {
    pub fn tile_at(&self, x: usize, y: usize) -> Result<&T, io::Error> {
        if y >= self.height || x >= self.width {
            panic!("Points must be within matrix limits");
        }
        Ok(&self.data[y][x])
    }
}
