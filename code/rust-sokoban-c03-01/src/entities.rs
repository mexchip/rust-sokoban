use crate::components::*;
use hecs::{Entity, World};

pub fn create_wall(world: &mut World, position: Position) -> Entity {
    world.spawn((
        Position { z: 10, ..position },
        Renderable {
            path: "/images/wall.png".to_string(),
        },
        Wall {},
        Immovable {},
    ))
}

pub fn create_floor(world: &mut World, position: Position) -> Entity {
    world.spawn((
        Position { z: 5, ..position },
        Renderable {
            path: "/images/floor.png".to_string(),
        },
    ))
}

// ANCHOR: create_box
pub fn create_box(world: &mut World, position: Position, colour: BoxColour) -> Entity {
    world.spawn((
        Position { z: 10, ..position },
        Renderable {
            path: format!("/images/box_{}.png", colour),
        },
        Box { colour },
        Movable {},
    ))
}

pub fn create_box_spot(world: &mut World, position: Position, colour: BoxColour) -> Entity {
    world.spawn((
        Position { z: 9, ..position },
        Renderable {
            path: format!("/images/box_spot_{}.png", colour),
        },
        BoxSpot { colour },
    ))
}
// ANCHOR_END: create_box

pub fn create_player(world: &mut World, position: Position) -> Entity {
    world.spawn((
        Position { z: 10, ..position },
        Renderable {
            path: "/images/player.png".to_string(),
        },
        Player {},
        Movable {},
    ))
}

pub fn create_gameplay(world: &mut World) -> Entity {
    world.spawn((Gameplay::default(),))
}
