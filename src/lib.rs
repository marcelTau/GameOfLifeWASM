mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl std::fmt::Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let sym = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", sym)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Self {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|_| Cell::Dead)
            .collect();

        Self {
            width,
            height,
            cells,
        }
    }

    pub fn setup_glider_gun(&mut self) {
        let indecies = [
            (28, 25),
            (29, 25),
            (29, 26),
            (28, 26),
            (28, 35),
            (29, 35),
            (30, 35),
            (31, 36),
            (27, 36),
            (26, 37),
            (26, 38),
            (32, 37),
            (32, 38),
            (31, 40),
            (30, 41),
            (29, 41),
            (28, 41),
            (29, 42),
            (27, 40),
            (29, 39),
            (28, 45),
            (28, 46),
            (27, 46),
            (26, 46),
            (26, 45),
            (27, 45),
            (25, 47),
            (29, 47),
            (29, 49),
            (30, 49),
            (25, 49),
            (24, 49),
            (26, 59),
            (27, 59),
            (27, 60),
            (26, 60),
        ];

        for (row, col) in indecies.iter() {
            self.toggle_cell(*row, *col);
        }
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells[idx].toggle();
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };
                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (col + delta_col) % self.width;
                let index = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[index] as u8;
            }
        }
        count
    }
}
