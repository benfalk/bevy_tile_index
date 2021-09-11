use crate::tile::Tile;

pub struct Board {
    tiles: Vec<Tile>,
}

impl Board {
    pub fn new(size: usize) -> Self {
        let tiles = vec![Tile::with_capacity(30); size * size];

        Self {
            tiles
        }
    }

    pub fn tile_at_index(&self, index: usize) -> &Tile {
        &self.tiles[index]
    }

    pub fn tile_at_index_mut(&mut self, index: usize) -> &mut Tile {
        &mut self.tiles[index]
    }
}

#[cfg(test)]
mod tests {
}
