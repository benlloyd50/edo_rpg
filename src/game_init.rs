use bracket_terminal::prelude::{BLACK, WHITE};
use specs::{Builder, World, WorldExt};

use crate::{
    components::{Blocking, Name, Position, Renderable, Strength},
    data_read::prelude::{build_being, load_simple_ldtk_level},
    player::Player,
};

pub fn initialize_game_world(ecs: &mut World) {
    let map = load_simple_ldtk_level(ecs);
    ecs.insert(map);

    ecs.create_entity()
        .with(Position::new(17, 20))
        .with(Player)
        .with(Strength { amt: 1 })
        .with(Renderable::new(WHITE, BLACK, 2))
        .with(Blocking)
        .with(Name("Tester".to_string()))
        .build();

    build_being("Bahhhby", Position::new(5, 15), ecs).ok();
    build_being("Greg Goat", Position::new(12, 20), ecs).ok();

    // debug_rocks(ecs);
}
