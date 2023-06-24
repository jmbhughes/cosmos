use ndarray::{Array2};
use ndarray_conv::*;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};


#[derive(Resource)]
pub struct UlamWarburtonCA {
    pub num_rows: usize,
    pub num_cols: usize,
    grid: Array2<u8>
}

impl UlamWarburtonCA {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {num_rows: rows, num_cols: cols, grid: Array2::zeros((rows, cols))}
    }

    pub fn get_dim(self) -> (usize, usize) {
        (self.num_rows, self.num_cols)
    }

    pub fn get(&self, row: usize, col: usize) -> u8 {
        self.grid[[row, col]]
    }

    pub fn set(&mut self, row: usize, col: usize) {
        self.grid[[row, col]] = 1;
    }

    pub fn unset(&mut self, row: usize, col: usize) {
        self.grid[[row, col]] = 0;
    }

    pub fn step(&mut self) {
        let kernel =Array2::from_shape_vec((3, 3),  
            vec![0, 1, 0, 1, 0, 1, 0, 1, 0]).unwrap();
        let grow_mask = self.grid.conv_2d(&kernel, 
            PaddingSize::Same, 
            PaddingMode::Zeros).unwrap().map(|&v| v == 1u8);
        for i in 0..self.num_rows {
            for j in 0..self.num_cols {
                if grow_mask[[i, j]] {
                    self.grid[[i, j]] = 1;
                }
            }
        }
    }

    pub fn iterate(&mut self, iterations: usize) {
        for _ in 0..iterations {
            self.step();
        }
    }

    pub fn show(self) {
        for i in 0..self.num_rows {
            for j in 0..self.num_cols {
                print!("{}", {match self.grid[[i,j]] {
                    0 => " ",
                    1 => "█",
                    _ => " "
                }});
            }
            println!();
        }
    }

}
