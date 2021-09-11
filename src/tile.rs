#![allow(unused_imports)]
#![allow(dead_code)]

use std::slice::Iter;
use bevy::math::Vec3;
use bevy::ecs::entity::Entity;

#[derive(Copy, Clone)]
pub struct EntityPosition {
    pub entity: Entity,
    pub position: Vec3,
}

#[derive(Clone)]
pub struct Tile {
    data: Vec<EntityPosition>,
}

impl Tile {
    ///
    /// Prepares a title to hold a certain amount of
    /// entities without needing to reallocate
    ///
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    ///
    /// Adds an entity with position to the tile and
    /// returns it's internal index.  It's upon the caller
    /// to save this index for `remove` and `update` calls
    ///
    pub fn add(&mut self, entity: Entity, position: Vec3) -> usize {
        let next_slot = self.data.len();
        self.data.push(EntityPosition { entity, position });
        next_slot
    }

    ///
    /// Removes the entity with it's position at the given
    /// index.  It returns an optional Entity that has taken
    /// that index.  It is the callers responsibility to
    /// note the entity's index has changed
    ///
    pub fn remove(&mut self, index: usize) -> Option<Entity> {
        if self.data.len() - 1 == index {
            self.data.pop();
            None
        }
        else {
            self.data.swap_remove(index);
            Some(self.data[index].entity)
        }
    }

    ///
    /// Updates the entity's position with the provided index
    ///
    pub fn update_position(&mut self, index: usize, position: Vec3) {
        self.data[index].position = position;
    }

    ///
    /// Provides a read-only iterator over all of the `EntityPosition`
    /// that are located in this title
    ///
    pub fn iter(&self) -> Iter<EntityPosition> {
        self.data.iter()
    }
}
