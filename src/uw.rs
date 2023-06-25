use std::cmp;

use ndarray::{Array2};
use ndarray_conv::*;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};


#[derive(Resource)]
pub struct AdvancedUlamWarburtonCA {
    pub num_rows: usize,
    pub num_cols: usize,
    grid: Array2<u8>,
    age: Array2<usize>,
    waiting_age: Array2<usize>,
    death_time: usize,
    wait_time: usize
}

impl AdvancedUlamWarburtonCA {
    pub fn new(rows: usize, cols: usize, wait_time: usize, death_time: usize) -> Self {
        Self {num_rows: rows, 
            num_cols: cols, 
            grid: Array2::zeros((rows, cols)),
            age: Array2::zeros((rows, cols)),
            waiting_age: Array2::ones((rows, cols)),
            death_time: death_time, 
            wait_time: wait_time            
        }
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
        let kernel = Array2::from_shape_vec((3, 3),  
            vec![0, 1, 0, 1, 0, 1, 0, 1, 0]).unwrap();

        let grow_mask = self.grid.conv_2d(&kernel, 
            PaddingSize::Same, 
            PaddingMode::Zeros).unwrap().map(|&v| v == 1u8);

        for i in 0..self.num_rows {
            for j in 0..self.num_cols {
                if self.grid[[i, j]] == 1 {
                    self.age[[i, j]] += 1;
                } else {
                    if self.waiting_age[[i, j]] >= 1 {
                        self.waiting_age[[i, j]] -= 1;
                    }
                }
                
                if grow_mask[[i, j]] {
                    if self.waiting_age[[i, j]] <= 0 {
                        self.grid[[i, j]] = 1;
                        self.age[[i, j]] = 1;
                    }
                }
                
                if self.age[[i, j]] > self.death_time {
                    self.age[[i, j]] = 0;
                    self.waiting_age[[i, j]] = self.wait_time;
                    self.grid[[i, j]] = 0;
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
