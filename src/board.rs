#![allow(unused_imports)]
#![allow(dead_code)]

use crate::tile::Tile;

pub struct Board {
    x_length: usize,
    y_length: usize,
    tiles: Vec<Tile>,
}

#[derive(Copy, Clone)]
pub struct Coordinate {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl Coordinate {
    #[inline]
    fn index_for_board(&self) -> usize {
        self.x + (self.y * self.x)
    }

    #[inline]
    fn validate_against_board(&self, board: &Board) {
        if self.x >= board.x_length {
            panic!("{} exceeds x_length {}", self.x, board.x_length);
        }

        if self.y >= board.y_length {
            panic!("{} exceeds y_length {}", self.y, board.y_length);
        }
    }
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

    #[inline]
    /// The x length, or also, how far to the "right"
    pub fn x_length(&self) -> usize {
        self.x_length
    }

    #[inline]
    /// The y length, or also, how far "down"
    pub fn y_length(&self) -> usize {
        self.y_length
    }

    ///
    /// Get tile reference given an x and y coordinate.
    /// Note that the coordinates start at 0,0.
    ///
    /// panics if x or y goes beyond the board bounds
    ///
    pub fn tile_at(&self, coord: &Coordinate) -> &Tile {
        coord.validate_against_board(self);
        &self.tiles[coord.index_for_board()]
    }

    ///
    /// Get mutable tile reference given an x and y coordinate.
    /// Note that the coordinates start at 0,0.
    ///
    /// panics if x or y goes beyond the board bounds
    ///
    pub fn tile_at_mut(&mut self, coord: &Coordinate) -> &mut Tile {
        coord.validate_against_board(self);
        &mut self.tiles[coord.index_for_board()]
    }

    pub fn tile_at_index(&self, index: usize) -> &Tile {
        &self.tiles[index]
    }

    pub fn tile_at_index_mut(&mut self, index: usize) -> &Tile {
        &mut self.tiles[index]
    }
}

#[cfg(test)]
mod tests {
}
