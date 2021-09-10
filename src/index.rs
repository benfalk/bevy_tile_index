#![allow(unused_imports)]
#![allow(dead_code)]

use bevy::math::Vec3;
use bevy::ecs::entity::Entity;
use std::collections::HashMap;
use crate::board::Board;


type EntityLookup = HashMap<Entity, EntityIndex>;

struct EntityIndex{
    board_index: usize,
    tile_index: usize,
}

pub struct Index {
    entity_lookup: EntityLookup,
    board: Board,
    x_units_width: usize,
    y_units_width: usize,
    cell_shift: usize,
    y_shift: usize,
}

impl Index {
    pub fn square(unit_size: usize, subsection_size: usize) -> Self {
        let cell_shift = (
            (((unit_size / subsection_size) as f32).ln() / (2.0 as f32).ln()).round()
        ) as usize;

        let y_shift = (
            ((subsection_size as f32).ln() / (2.0 as f32).ln()).round()
        ) as usize;


        Self {
            entity_lookup: EntityLookup::with_capacity(200),
            board: Board::new(subsection_size, subsection_size),
            x_units_width: unit_size,
            y_units_width: unit_size,
            cell_shift,
            y_shift,
        }
    }

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
                let lookup = entity_lookup.get_mut(entity).unwrap();
                lookup.board_index = board_index;
                lookup.tile_index = tile_index;
            }
        }
    }

    fn board_index_for_vector(&self, position: &Vec3) -> usize {
        let y = position.y as usize >> self.cell_shift;
        let x = position.x as usize >> self.cell_shift;
        x + (y << self.y_shift)
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::Vec3;
    use super::*;

    #[test]
    fn the_index_system_works() {
        let index = Index::square(4096, 64);
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
        // TODO: Write these tests
    }
}
