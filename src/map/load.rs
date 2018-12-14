use std::fs;
use std::io;

use map::map::{Map};

pub fn load_map(file_path: &str)-> Result<Map, io::Error>{
    let file = fs::read_to_string(file_path)?;
    let matrix: Vec<Vec<char>> = file.split("\n") .map(|row| row.chars().collect()).collect();

    let height:u8 = matrix.len() as u8;
    let mut width:u8 = 0;

    for row in &matrix {
        let size = row.len() as u8;
        if size > width {
            width = size;
        }
    }
    Ok(Map{ width: width, height: height, data: matrix})
}
