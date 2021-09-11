use bevy::math::Vec3;
use bevy::ecs::entity::Entity;
use std::collections::HashMap;
use crate::board::Board;
use crate::coord::{Coord, SurroundingCoords};
use crate::tile::{EntityPosition, Tile};
use std::slice::Iter;

type EntityLookup = HashMap<Entity, EntityIndex>;

#[derive(Debug)]
struct EntityIndex{
    board_index: usize,
    tile_index: usize,
}

#[allow(dead_code)]
pub struct Index {
    entity_lookup: EntityLookup,
    board: Board,
    unit_size: usize,
    subsection_size: usize,
    cell_shift: usize,
    y_shift: usize,
}

#[allow(dead_code)]
pub struct SurroundingEntites<'a> {
    index: &'a Index,
    coords: SurroundingCoords,
    entities: Iter<'a, EntityPosition>,
    radius: f32,
}

impl Index {
    ///
    /// Build a new Index
    ///
    /// The unit size given must be a power of two, this can
    /// be thought of as the game distance you expect to cover
    /// in a maximum amount.
    ///
    /// The subsection size must also be a power of two and
    /// less then the unit size.  This you can this of this
    /// as a lower resolution max distance of the unit size.
    ///
    pub fn new(unit_size: usize, subsection_size: usize) -> Self {
        if unit_size.next_power_of_two() != unit_size {
            panic!("unit_size must be a power of two");
        }

        if subsection_size.next_power_of_two() != subsection_size {
            panic!("subsection_size must be a power of two");
        }

        if subsection_size >= unit_size {
            panic!("subsection_size must be less than unit_size");
        }

        let cell_shift = (
            (((unit_size / subsection_size) as f32).ln() / (2.0 as f32).ln()).round()
        ) as usize;

        let y_shift = (
            ((subsection_size as f32).ln() / (2.0 as f32).ln()).round()
        ) as usize;

        Self {
            entity_lookup: EntityLookup::with_capacity(200),
            board: Board::new(subsection_size),
            unit_size,
            subsection_size,
            cell_shift,
            y_shift,
        }
    }

    ///
    /// Apply an entity with a position.
    ///
    /// This is either an update or insert into the index.  It's
    /// time is O(1) at the cost of a HasMap lookup and insert.
    /// As well as a jump to the correct vector skip-list where
    /// the data is.
    ///
    pub fn add_or_update_entity(&mut self, entity: &Entity, position: &Vec3) {
        let board_index = self.board_index_for_vector(position);
        let Self { ref mut entity_lookup, ref mut board, .. } = self;

        match entity_lookup.get(entity) {
            None => {
                let tile = board.tile_at_index_mut(board_index);
                let tile_index = tile.add(*entity, *position);
                entity_lookup.insert(*entity, EntityIndex { board_index, tile_index });
            },
            Some(same) if same.board_index == board_index => {
                let tile = board.tile_at_index_mut(board_index);
                tile.update_position(same.tile_index, *position);
            },
            Some(diff) => {
                let tile = board.tile_at_index_mut(diff.board_index);
                if let Some(displaced) = tile.remove(diff.tile_index) {
                    entity_lookup.get_mut(&displaced).unwrap().tile_index = diff.tile_index;
                }
                let new_tile = board.tile_at_index_mut(board_index);
                let tile_index = new_tile.add(*entity, *position);
                entity_lookup.insert(*entity, EntityIndex { board_index, tile_index });
            }
        }
    }

    ///
    /// Removes an entity from the index.
    ///
    pub fn remove(&mut self, entity: &Entity) {
        let Self { ref mut entity_lookup, ref mut board, .. } = self;
        match entity_lookup.get(entity) {
            None => (),
            Some(idx) => {
                let tile = board.tile_at_index_mut(idx.board_index);
                if let Some(displaced) = tile.remove(idx.tile_index) {
                    entity_lookup.get_mut(&displaced).unwrap().tile_index = idx.tile_index;
                }
                entity_lookup.remove(entity);
            }
        }
    }

    pub fn nearby_entities(&self, position: &Vec3, radius: f32) -> SurroundingEntites {
        let y = position.y as usize >> self.cell_shift;
        let x = position.x as usize >> self.cell_shift;
        let distance = ((radius as usize) >> self.cell_shift) + 1;
        let mut coords = Coord::new(x, y).surrounding_coords(distance, self.subsection_size - 1);
        let entities = coords.next().unwrap().tile(self).iter();

        SurroundingEntites {
            index: self,
            coords,
            entities,
            radius,
        }
    }

    fn board_index_for_vector(&self, position: &Vec3) -> usize {
        let y = position.y as usize >> self.cell_shift;
        let x = position.x as usize >> self.cell_shift;
        x + (y << self.y_shift)
    }
}

impl Coord {
    fn tile<'a, 'b>(&'a self, index: &'b Index) -> &'b Tile {
        let idx = self.x + (self.y << index.y_shift);
        index.board.tile_at_index(idx)
    }
}

impl<'a> Iterator for SurroundingEntites<'a> {
    type Item = &'a EntityPosition;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.entities.next();

            // TODO: Range Check
            if next.is_some() { return next; }

            if let Some(coord) = self.coords.next() {
                self.entities = coord.tile(self.index).iter();
            }
            else{
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::Vec3;
    use bevy::ecs::entity::Entity;
    use super::*;

    #[test]
    fn the_index_system_works() {
        let index = Index::new(4096, 64);
        let near_zero = Vec3::new(1.0, 1.0, 0.0);
        let near_sixtyfour = Vec3::new(63.9, 63.9, 0.0);
        let over_one = Vec3::new(64.0, 18.0, 0.0);
        let down_one = Vec3::new(18.0, 64.0, 0.0);
        let over_one_down_one = Vec3::new(64.0, 64.0, 0.0);
        let end_of_world = Vec3::new(4095.9, 4095.9, 0.0);

        assert_eq!(
            index.board_index_for_vector(&near_zero),
            0
        );

        assert_eq!(
            index.board_index_for_vector(&near_sixtyfour),
            0
        );

        assert_eq!(
            index.board_index_for_vector(&over_one),
            1
        );

        assert_eq!(
            index.board_index_for_vector(&down_one),
            64
        );

        assert_eq!(
            index.board_index_for_vector(&over_one_down_one),
            65
        );

        assert_eq!(
            index.board_index_for_vector(&end_of_world),
            64 * 64 - 1
        )
    }

    #[test]
    fn adding_entities_works() {
        let mut index = Index::new(4096, 64);
        let entity_1 = Entity::new(1);
        let entity_2 = Entity::new(2);
        let entity_3 = Entity::new(3);

        let somewhere = Vec3::new(42.0, 196.0, 12.0);
        let expected_index = index.board_index_for_vector(&somewhere);

        index.add_or_update_entity(&entity_1, &somewhere);
        index.add_or_update_entity(&entity_2, &somewhere);
        index.add_or_update_entity(&entity_3, &somewhere);

        assert_eq!(
            index.board.tile_at_index(expected_index).iter().count(),
            3
        );

        assert!(
            index.board.tile_at_index(expected_index).iter().all(|&enity_loc|{
                enity_loc.position == somewhere
            })
        )
    }

    #[test]
    fn entity_moving_boundaries() {
        let mut index = Index::new(4096, 64);
        let entity_1 = Entity::new(1);
        let entity_2 = Entity::new(2);
        let somewhere = Vec3::new(42.0, 196.0, 12.0);
        let somewhere_new = Vec3::new(42.0, 2000.0, 13.0);

        let somewhere_idx = index.board_index_for_vector(&somewhere);
        let new_idx = index.board_index_for_vector(&somewhere_new);

        index.add_or_update_entity(&entity_1, &somewhere);
        index.add_or_update_entity(&entity_2, &somewhere);
        index.add_or_update_entity(&entity_1, &somewhere_new);

        assert_eq!(
            index.board.tile_at_index(somewhere_idx).iter().count(),
            1
        );

        assert_eq!(
            index.board.tile_at_index(new_idx).iter().count(),
            1
        );

        index.add_or_update_entity(&entity_1, &somewhere);

        assert_eq!(
            index.board.tile_at_index(somewhere_idx).iter().count(),
            2
        );

        assert_eq!(
            index.board.tile_at_index(new_idx).iter().count(),
            0
        );

        index.add_or_update_entity(&entity_1, &somewhere_new);
        index.add_or_update_entity(&entity_2, &somewhere_new);

        assert_eq!(
            index.board.tile_at_index(somewhere_idx).iter().count(),
            0
        );

        assert_eq!(
            index.board.tile_at_index(new_idx).iter().count(),
            2
        );
    }

    #[test]
    fn removing_entities() {
        let mut index = Index::new(4096, 64);
        let entity_1 = Entity::new(1);
        let entity_2 = Entity::new(2);
        let somewhere = Vec3::new(42.0, 196.0, 12.0);
        let somewhere_new = Vec3::new(42.0, 2000.0, 13.0);

        let somewhere_idx = index.board_index_for_vector(&somewhere);
        let new_idx = index.board_index_for_vector(&somewhere_new);

        index.add_or_update_entity(&entity_1, &somewhere);
        index.add_or_update_entity(&entity_2, &somewhere);

        index.add_or_update_entity(&entity_1, &somewhere_new);
        index.add_or_update_entity(&entity_2, &somewhere_new);

        index.remove(&entity_2);
        index.remove(&entity_1);

        assert_eq!(
            index.board.tile_at_index(somewhere_idx).iter().count(),
            0
        );

        assert_eq!(
            index.board.tile_at_index(new_idx).iter().count(),
            0
        );
    }

    #[test]
    fn nearby_entities() {
        let mut index = Index::new(4096, 64);
        let entity_1 = Entity::new(1);
        let entity_2 = Entity::new(2);
        let entity_3 = Entity::new(3);
        let somewhere = Vec3::new(196.0, 196.0, 12.0);
        let somewhere_a_little_different = Vec3::new(200.0, 200.0, 42.0);
        let somewhere_new = Vec3::new(2000.0, 2000.0, 13.0);

        index.add_or_update_entity(&entity_1, &somewhere);
        index.add_or_update_entity(&entity_2, &somewhere);
        index.add_or_update_entity(&entity_3, &somewhere_new);

        assert_eq!(
            index.nearby_entities(&somewhere, 60.0).count(),
            2
        );

        assert_eq!(
            index.nearby_entities(&somewhere_new, 60.0).count(),
            1
        );

        index.add_or_update_entity(&entity_3, &somewhere_a_little_different);

        assert_eq!(
            index.nearby_entities(&somewhere, 60.0).count(),
            3
        );

        assert_eq!(
            index.nearby_entities(&somewhere_new, 60.0).count(),
            0
        );
    }
}
