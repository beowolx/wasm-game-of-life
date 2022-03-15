use std::fmt;
use wasm_bindgen::prelude::*;

mod utils;
use fixedbitset::FixedBitSet;

// When the `wee_alloc` feature is enabled, use `wee_alloc`as the global
// allocator
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

/// Public methods, exported to JS
#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                // let next_cell = match (cell, live_neighbors) {
                //     // Rule 1: Any live cell with fewer than two live neighbours
                //     // dies, as if caused by underpopulation.
                //     (Cell::Alive, x) if x < 2 => Cell::Dead,
                //     // Rule 2: Any live cell with two or three live neighbours
                //     (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                //     // Rule 3: Any live cell with more than three live
                //     // neighbours dies, as if by overpopulation.
                //     (Cell::Alive, x) if x > 3 => Cell::Dead,
                //     // Rule 4: Any dead cell with exactly three live neighbours
                //     // becomes a live cell, as if by reproduction.
                //     (Cell::Dead, 3) => Cell::Alive,
                //     // All other cells remain in the same state.
                //     (otherwise, _) => otherwise,
                // };

                // next[idx] = next_cell;

                next.set(
                    idx,
                    match (cell, live_neighbors) {
                        (true, x) if x < 2 => false,
                        (true, 2) | (true, 3) => true,
                        (true, x) if x > 3 => false,
                        (false, 3) => true,
                        (otherwise, _) => otherwise,
                    },
                )
            }
        }
        self.cells = next;
    }

    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, i % 2 == 0 || i % 7 == 0);
        }

        // let cells = (0..width * height)
        //     .map(|_i| {
        //         if js_sys::Math::random() < 0.5 {
        //             Cell::Alive
        //         } else {
        //             Cell::Dead
        //         }
        //     })
        //     .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].into_iter() {
            for delta_col in [self.width - 1, 0, 1].into_iter() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                // let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                let symbol = if js_sys::Math::random() < 0.5 {
                    '◼'
                    // Alive...
                } else {
                    '◻'
                    // Dead...
                };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
