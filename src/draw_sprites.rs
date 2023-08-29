use bracket_terminal::prelude::{DrawBatch, Point, *};
use specs::{Join, World, WorldExt};

use crate::{
    camera::get_player_camera, components::Renderable, map::render_map, Position, CL_INTERACTABLES,
    CL_WORLD,
};

pub fn draw_sprites(ecs: &World, draw_batch: &mut DrawBatch) {
    let positions = ecs.read_storage::<Position>();
    let renderables = ecs.read_storage::<Renderable>();

    let bounding_box = get_player_camera(ecs);

    let data = (&positions, &renderables).join().collect::<Vec<_>>();
    for (pos, render) in data.iter() {
        if !(pos.x as i32 >= bounding_box.x1
            && bounding_box.x2 > pos.x as i32
            && pos.y as i32 >= bounding_box.y1
            && bounding_box.y2 > pos.y as i32)
        {
            continue;
        }
        draw_batch.set_with_z(
            Point::new(
                pos.x as i32 - bounding_box.x1,
                pos.y as i32 - bounding_box.y1,
            ),
            render.color_pair,
            render.atlas_index,
            render.z_priority,
        );
    }
}

/// Draws the CL_INTERACTABLES and CL_WORLD sprites to the screen
pub fn draw_sprite_layers(ecs: &World) {
    let mut draw_batch = DrawBatch::new();

    draw_batch.target(CL_INTERACTABLES);
    draw_batch.cls();

    draw_sprites(&ecs, &mut draw_batch);
    draw_batch.submit(CL_INTERACTABLES).expect("Batch error??");

    draw_batch.target(CL_WORLD);
    draw_batch.cls();
    render_map(&ecs, &mut draw_batch);
    draw_batch.submit(CL_WORLD).expect("Batch error??");
}

#[allow(dead_code)]
const COLORS: [&'static (u8, u8, u8); 11] = [
    &ROSYBROWN,
    &DARKSALMON,
    &BURLYWOOD,
    &CADETBLUE4,
    &ANTIQUEWHITE,
    &DARKGOLDENROD1,
    &CORNFLOWER_BLUE,
    &LIGHTPINK,
    &LIGHTCYAN4,
    &LIGHTGOLDENRODYELLOW,
    &LIGHTSALMON,
];
