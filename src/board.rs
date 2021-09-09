#![allow(unused_imports)]
#![allow(dead_code)]

use crate::tile::Tile;

pub struct Board {
    x_length: usize,
    y_length: usize,
    tiles: Vec<Tile>,
}

impl Board {
    ///
    /// Create a new board of a given x and y size
    ///
    /// panics if either dimension is zero
    ///
    pub fn new(x_length: usize, y_length: usize) -> Self {
        if x_length == 0 {
            panic!("x_length may not be zero");
        }

        if y_length == 0 {
            panic!("y_length may not be zero");
        }

        let tiles = vec![Tile::with_capacity(30); x_length * y_length];

        Self {
            x_length,
            y_length,
            tiles
        }
    }

    ///
    /// A read-only reference to the tile at the given index
    ///
    pub fn tile_at_index(&self, index: usize) -> &Tile {
        &self.tiles[index]
    }

    ///
    /// A mutable reference to the tile at the given index
    ///
    pub fn tile_at_index_mut(&mut self, index: usize) -> &mut Tile {
        &mut self.tiles[index]
    }
}

#[cfg(test)]
mod tests {
}
