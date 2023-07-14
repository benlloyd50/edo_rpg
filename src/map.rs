use bracket_terminal::prelude::{DrawBatch, WHITE, ColorPair, BLACK, Point};
use specs::World;


pub struct Map {
    tiles: Vec<WorldTile>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Map {
            tiles: vec![WorldTile::default(); width * height],
            width,
            height
        }
    }

    pub fn xy_to_idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

/// Renders the current map resource to the current console layer
pub fn render_map(ecs: &World, batch: &mut DrawBatch) {
    let map = ecs.fetch::<Map>();

    for x in 0..map.width {
        for y in 0..map.height {
            batch.set(Point::new(x, y), ColorPair::new(WHITE, BLACK), map.tiles[map.xy_to_idx(x, y)].atlas_index);
        }
    }
}

#[derive(Clone)]
struct WorldTile {
    atlas_index: usize
}

impl WorldTile {
    pub fn default() -> Self {
        Self {
            atlas_index: 4,
        }
    }
}
