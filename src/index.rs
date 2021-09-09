#![allow(unused_imports)]
#![allow(dead_code)]

use bevy::math::Vec3;
use bevy::ecs::entity::Entity;
use std::collections::HashMap;
use crate::board::{Board, Coordinate};


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
}

impl Index {
    pub fn one_hundred_square_ten_unit_titles() -> Self {
        Self {
            entity_lookup: EntityLookup::with_capacity(200),
            board: Board::new(10, 10),
            x_units_width: 100,
            y_units_width: 100,
        }
    }

    pub fn add_or_update_entity(&mut self, entity: &Entity, position: &Vec3) {
        let Self { ref mut entity_lookup, ref mut board, .. } = self;

        match entity_lookup.get(entity) {
            None => {
            },
            Some(entity_index) => {
            }
        }
    }
}
