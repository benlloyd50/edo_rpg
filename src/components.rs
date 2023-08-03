use std::time::Duration;

use bracket_terminal::prelude::{ColorPair, Point};
use specs::{Component, Entity, NullStorage, VecStorage};

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Renderable {
    pub color_pair: ColorPair,
    pub atlas_index: usize,
}

impl Renderable {
    pub fn new(color_pair: ColorPair, atlas_index: usize) -> Self {
        Self {
            color_pair,
            atlas_index,
        }
    }
}

/// Represents a position of anything that exists physically in the game world
#[derive(Debug, Component, Copy, Clone)]
#[storage(VecStorage)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl From<Point> for Position {
    /// May panic if either of the coords of `value` are negative but that should rarely be the case when used in the
    /// proper context. i.e. dont use this when dealing with delta point values (-1, -1)
    fn from(value: Point) -> Self {
        Self::new(value.x as usize, value.y as usize)
    }
}

/// TODO: This is temporary for testing out breaking things and will be replaced by a more comprehensive stat
#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Strength {
    pub amt: u32,
}

/// Prevents gameobjects from passing through it
#[derive(Debug, Component, Default)]
#[storage(NullStorage)]
pub struct Blocking;

#[derive(Debug, Component, Default)]
#[storage(NullStorage)]
pub struct Fishable;

#[derive(Component)]
#[storage(VecStorage)]
pub struct FishAction {
    pub target: Position,  // mainly just for finding where the fishing rod will be spawned
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct WaitingForFish {
    pub attempts: usize,
    pub time_since_last_attempt: Duration,
}

impl WaitingForFish {
    pub fn new(attempts: usize) -> Self {
        Self {
            attempts,
            time_since_last_attempt: Duration::new(0, 0),
        }
    }
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct FishOnTheLine;

#[derive(Debug, Component)]
#[storage(VecStorage)]
#[allow(dead_code)]
pub struct HealthStats {
    pub hp: u32,
    max_hp: u32,
    pub defense: u32,
}

impl HealthStats {
    pub fn new(max_hp: u32, defense: u32) -> Self {
        Self {
            hp: max_hp,
            max_hp,
            defense,
        }
    }
}

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Breakable {
    pub by: ToolType,
}

impl Breakable {
    pub fn new(by: ToolType) -> Self {
        Self { by }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ToolType {
    Hand,
    Pickaxe,
    Axe,
    Shovel,
}

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct BreakAction {
    pub target: Entity,
}

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct SufferDamage {
    pub amount: Vec<i32>,
}

/// Used to delete an entity when a condition is satisfied
#[derive(Component, Clone, Copy)]
#[storage(VecStorage)]
pub enum DeleteCondition {
    Timed(Duration),  // Condition is based on deleting after a specificed amount of time
    Event(Entity),    // Condition is based on when the entity finishes their activity
}

/// Used to signal to other systems that an entity finished their activity
#[derive(Component)]
#[storage(VecStorage)]
pub enum FinishedActivity { 
    Fishing, 
}
