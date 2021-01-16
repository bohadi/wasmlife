#[macro_use]
mod utils;

//use std::fmt;
use wasm_bindgen::prelude::*;
extern crate js_sys;
extern crate web_sys;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Live = 1,
}
impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Live,
            Cell::Live => Cell::Dead,
        };
    }
}

#[wasm_bindgen]
pub struct Universe {
    width:  usize,
    height: usize,
    cells: Vec<Cell>,
}
#[wasm_bindgen]
impl Universe {
    pub fn toggle_cell(&mut self, row: usize, col: usize) {
        let idx = self.get_index(row, col);
        self.cells[idx].toggle();
    }

    fn set_cells(&mut self, cells: &[(usize, usize)]) {
        let mut next = self.cells.clone();
        for (row, col) in cells.iter() {
            let idx = self.get_index(*row, *col);
            next[idx] = Cell::Live;
        }
        self.cells = next;
    }

    // doodads
    pub fn spawn_block_at(&mut self, cx: usize, cy: usize) {
    }
    pub fn spawn_beehive_at(&mut self, cx: usize, cy: usize) {
    }
    pub fn spawn_blinker_at(&mut self, cx: usize, cy: usize) {
    }
    // megastructures

    // armor
    // reflectors, 

    // weapons
    // guns, waves
    
    // projectiles
    pub fn spawn_spaceship_at(&mut self, cx: usize, cy: usize) {
    }

    pub fn spawn_glider_at(&mut self, cx: usize, cy: usize, dir: usize) {
        log!("spawn dir{} glider at ({},{})", dir,cx,cy);
        let cxm = (cx-1+self.width ) % self.width;  // wrap edges
        let cxp = (cx+1)             % self.width;
        let cym = (cy-1+self.height) % self.height;
        let cyp = (cy+1)             % self.height;
        let cells: [(usize,usize); 5];
        match dir {
            0 => { // southeasterly
                cells = [(cym,cxm), (cym,cxp), (cy,cx), (cy,cxp), (cyp,cx)];
            }
            1 => { // northwesterly
                cells = [(cyp,cxp), (cyp,cxm), (cy,cx), (cy,cxm), (cym,cx)];
            }
            2 => { // northeasterly
                cells = [(cyp,cxm), (cym,cxm), (cy,cx), (cym,cx), (cy,cxp)];
            }
            _ => { // southwesterly
                cells = [(cym,cxp), (cyp,cxp), (cy,cx), (cyp,cx), (cy,cxm)];
            }
        }
        self.set_cells(&cells);
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let nlive = self.live_neighbor_count(row, col);
                let next_cell = match (cell, nlive) {
                    (Cell::Live, x) if x < 2 => Cell::Dead,
                    (Cell::Live, 2) | (Cell::Live, 3) => Cell::Live,
                    (Cell::Live, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Live,
                    (otherwise, _) => otherwise,
                };
                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }


    pub fn new(width: usize, height: usize) -> Universe {
        let cells = vec![Cell::Dead; width*height];
        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn init_random(&mut self, p: f64) {
        self.cells = (0..self.width*self.height)
            .map(|_| {
                if js_sys::Math::random() > p {
                    Cell::Live
                } else {
                    Cell::Dead
                }
            })
            .collect();
    }

    pub fn init_gliders(&mut self, dir: usize) {
        self.cells = vec![Cell::Dead; self.width*self.height];

        self.spawn_glider_at( 5, 5, 0);
        self.spawn_glider_at(10,10, 1);

        self.spawn_glider_at( 3,10, 2);
        self.spawn_glider_at(10, 3, 3);
        
        //self.spawn_glider_at(20,20, dir); // alpha squad
        //self.spawn_glider_at(21,15, dir);
        //self.spawn_glider_at(15,21, dir);
        //self.spawn_glider_at(20, 7, dir); // right squad
        //self.spawn_glider_at(21, 2, dir);
        //self.spawn_glider_at(15, 8, dir);
        //self.spawn_glider_at( 7,20, dir); // left squad
        //self.spawn_glider_at( 2,21, dir);
        //self.spawn_glider_at( 8,15, dir);
    }

    pub fn init_modulo(&mut self) {
        self.cells = (0..self.width*self.height)
            .map(|i| {
                if i % (utils::get_random_int(2.0,2.0) as usize) == 0 ||
                   i % (utils::get_random_int(7.0,7.0) as usize) == 0 {
                    Cell::Live
                } else {
                    Cell::Dead
                }
            })
            .collect();
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    fn live_neighbor_count(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;
        for delta_row in [self.height -1, 0, 1].iter().cloned() { //avoid u8 underflow
            for delta_col in [self.width -1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbor_row = (row + delta_row) % self.height; //wrap edges
                let neighbor_col = (col + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    fn get_index(&self, row: usize, col: usize) -> usize {
        (row * self.width + col) as usize
    }
}
