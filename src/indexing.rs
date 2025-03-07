/* Indexing.rs
 *   Contains the many systems that index entities or tiles in order to update the index when
 *   needed.
 * */

use bracket_lib::terminal::Point;
use log::warn;
use specs::{Entities, Join, ReadStorage, System, WriteExpect};

use crate::{
    components::{Blocking, Breakable, Fishable, Item, Position},
    map::{MapRes, TileEntity},
};

/// Clears the entity contents of every tile in the map
pub struct IndexReset;

impl<'a> System<'a> for IndexReset {
    type SystemData = (WriteExpect<'a, MapRes>,);

    fn run(&mut self, (mut map,): Self::SystemData) {
        for content in map.0.tile_entities.iter_mut() {
            content.clear();
        }
    }
}

pub struct IndexBlockedTiles;

impl<'a> System<'a> for IndexBlockedTiles {
    type SystemData = (WriteExpect<'a, MapRes>, ReadStorage<'a, Position>, ReadStorage<'a, Blocking>, Entities<'a>);

    fn run(&mut self, (mut map, pos, blocking, entities): Self::SystemData) {
        for (pos, _, e) in (&pos, &blocking, &entities).join() {
            let idx = map.0.xy_to_idx(pos.x, pos.y);
            match map.0.tile_entities.get_mut(idx) {
                Some(entities) => {
                    entities.push(TileEntity::Blocking(e));
                }
                None => warn!("Idx: {} was out of bounds, {:?}", idx, pos),
            }
        }
    }
}

pub struct IndexBreakableTiles;

impl<'a> System<'a> for IndexBreakableTiles {
    type SystemData = (WriteExpect<'a, MapRes>, ReadStorage<'a, Position>, ReadStorage<'a, Breakable>, Entities<'a>);

    fn run(&mut self, (mut map, pos, breakable, entities): Self::SystemData) {
        for (id, pos, _) in (&entities, &pos, &breakable).join() {
            let idx = map.0.xy_to_idx(pos.x, pos.y);
            match map.0.tile_entities.get_mut(idx) {
                Some(entities) => {
                    entities.push(TileEntity::Breakable(id));
                }
                None => warn!("Idx: {} was out of bounds, {:?}", idx, pos),
            }
        }
    }
}

pub struct IndexFishableTiles;

impl<'a> System<'a> for IndexFishableTiles {
    type SystemData = (WriteExpect<'a, MapRes>, ReadStorage<'a, Position>, ReadStorage<'a, Fishable>, Entities<'a>);

    fn run(&mut self, (mut map, pos, fishable, entities): Self::SystemData) {
        for (entity, pos, _) in (&entities, &pos, &fishable).join() {
            let idx = map.0.xy_to_idx(pos.x, pos.y);
            match map.0.tile_entities.get_mut(idx) {
                Some(entities) => {
                    entities.push(TileEntity::Fishable(entity));
                }
                None => warn!("Idx: {} was out of bounds, {:?}", idx, pos),
            }
        }
    }
}

pub struct IndexItemTiles;

impl<'a> System<'a> for IndexItemTiles {
    type SystemData = (WriteExpect<'a, MapRes>, ReadStorage<'a, Position>, ReadStorage<'a, Item>, Entities<'a>);

    fn run(&mut self, (mut map, pos, items, entities): Self::SystemData) {
        for (entity, pos, _) in (&entities, &pos, &items).join() {
            let idx = map.0.xy_to_idx(pos.x, pos.y);
            match map.0.tile_entities.get_mut(idx) {
                Some(entities) => {
                    entities.push(TileEntity::Item(entity));
                }
                None => warn!("Idx: {} was out of bounds, {:?}", idx, pos),
            }
        }
    }
}

pub fn idx_to_point(idx: usize, width: usize) -> Point {
    let x = idx % width;
    let y = idx / width;

    Point::new(x, y)
}
