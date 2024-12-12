/* ANCHOR: all */
// Rust sokoban
// main.rs

use ggez::{
    conf,
    event::{self, KeyCode},
    graphics::{self, DrawParam, Image},
    input::keyboard,
    Context, GameResult,
};
use glam::Vec2;
use hecs::{Entity, World};

use std::path;
use std::HashMap;

const TILE_WIDTH: f32 = 32.0;
const MAP_WIDTH: u8 = 8;
const MAP_HEIGHT: u8 = 9;

// ANCHOR: components
#[derive(Clone, Copy)]
pub struct Position {
    x: u8,
    y: u8,
    z: u8,
}

pub struct Renderable {
    path: String,
}

pub struct Wall {}

pub struct Player {}

pub struct Box {}

pub struct BoxSpot {}

pub struct Movable;

pub struct Immovable;

// ANCHOR_END: components

// ANCHOR: game
// This struct will hold all our game state
// For now there is nothing to be held, but we'll add
// things shortly.
struct Game {
    world: World,
}
// ANCHOR_END: game

// ANCHOR: init
// Initialize the level// Initialize the level
pub fn initialize_level(world: &mut World) {
    const MAP: &str = "
    N N W W W W W W
    W W W . . . . W
    W . . . B . . W
    W . . . . . . W 
    W . P . . . . W
    W . . . . . . W
    W . . S . . . W
    W . . . . . . W
    W W W W W W W W
    ";

    load_map(world, MAP.to_string());
}

pub fn load_map(world: &mut World, map_string: String) {
    // read all lines
    let rows: Vec<&str> = map_string.trim().split('\n').map(|x| x.trim()).collect();

    for (y, row) in rows.iter().enumerate() {
        let columns: Vec<&str> = row.split(' ').collect();

        for (x, column) in columns.iter().enumerate() {
            // Create the position at which to create something on the map
            let position = Position {
                x: x as u8,
                y: y as u8,
                z: 0, // we will get the z from the factory functions
            };

            // Figure out what object we should create
            match *column {
                "." => {
                    create_floor(world, position);
                }
                "W" => {
                    create_floor(world, position);
                    create_wall(world, position);
                }
                "P" => {
                    create_floor(world, position);
                    create_player(world, position);
                }
                "B" => {
                    create_floor(world, position);
                    create_box(world, position);
                }
                "S" => {
                    create_floor(world, position);
                    create_box_spot(world, position);
                }
                "N" => (),
                c => panic!("unrecognized map item {}", c),
            }
        }
    }
}
// ANCHOR_END: init

// ANCHOR: handler
impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, context: &mut Context) -> GameResult {
        // Run input system
        {
            run_input(&self.world, context);
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        // Render game entities
        {
            run_rendering(&self.world, context);
        }

        Ok(())
    }
}
// ANCHOR_END: handler

// ANCHOR: entities
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

pub fn create_box(world: &mut World, position: Position) -> Entity {
    world.spawn((
        Position { z: 10, ..position },
        Renderable {
            path: "/images/box.png".to_string(),
        },
        Box {},
        Movable {},
    ))
}

pub fn create_box_spot(world: &mut World, position: Position) -> Entity {
    world.spawn((
        Position { z: 9, ..position },
        Renderable {
            path: "/images/box_spot.png".to_string(),
        },
        BoxSpot {},
        Immovable {},
    ))
}

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
// ANCHOR_END: entities

// ANCHOR: rendering_system
fn run_rendering(world: &World, context: &mut Context) {
    // Clearing the screen (this gives us the background colour)
    graphics::clear(context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

    // Get all the renderables with their positions and sort by the position z
    // This will allow us to have entities layered visually.
    let mut query = world.query::<(&Position, &Renderable)>();
    let mut rendering_data: Vec<(Entity, (&Position, &Renderable))> = query.into_iter().collect();
    rendering_data.sort_by_key(|&k| k.1 .0.z);

    // Iterate through all pairs of positions & renderables, load the image
    // and draw it at the specified position.
    for (_, (position, renderable)) in rendering_data.iter() {
        // Load the image
        let image = Image::new(context, renderable.path.clone()).expect("expected image");
        let x = position.x as f32 * TILE_WIDTH;
        let y = position.y as f32 * TILE_WIDTH;

        // draw
        let draw_params = DrawParam::new().dest(Vec2::new(x, y));
        graphics::draw(context, &image, draw_params).expect("expected render");
    }

    // Finally, present the context, this will actually display everything
    // on the screen.
    graphics::present(context).expect("expected to present");
}
// ANCHOR_END: rendering_system

// ANCHOR: input_system
fn run_input(world: &World, context: &mut Context) {
    let mut to_move = Vec::new();

    // get all the movables and immovables
    let mov: HashMap<(u8, u8), Index> = (&entities, &movables, &positions)
        .join()
        .map(|t| ((t.2.x, t.2.y), t.0.id()))
        .collect::<HashMap<_, _>>();
    let immov: HashMap<(u8, u8), Index> = (&entities, &immovables, &positions)
        .join()
        .map(|t| ((t.2.x, t.2.y), t.0.id()))
        .collect::<HashMap<_, _>>();

    let relevant_keys = vec![
        KeyCode::Up,
        KeyCode::Down,
        KeyCode::Left,
        KeyCode::Right,
    ];

    for (_, (position, _player)) in world.query::<(&mut Position, &Player)>().iter() {
        if keyboard::is_key_repeated(context) {
            continue;
        }

        if relevant_keys.iter().none(|key| keyboard::is_key_pressed(context, *key)))
        {
            continue;
        }

                // Now iterate through current position to the end of the map
        // on the correct axis and check what needs to move.
        let (start, end, is_x) = {
            if keyboard::is_key_pressed(context, KeyCode::Up) {
                (position.y, 0, false)
            }
            if keyboard::is_key_pressed(context, KeyCode::Down) {
                (position.y, MAP_HEIGHT, false)
            }
            if keyboard::is_key_pressed(context, KeyCode::Left) {
                (position.x, 0, true)
            }
            if keyboard::is_key_pressed(context, KeyCode::Right) {
                (position.x, MAP_WIDTH, true)
            }
        };

        let range = if start < end {
            (start..=end).collect::<Vec<_>>()
        } else {
            (end..=start).rev().collect::<Vec<_>>()
        };

        for x_or_y in range {
            let pos = if is_x {
                (x_or_y, position.y)
            } else {
                (position.x, x_or_y)
            };

            // find a movable
            // if it exists, we try to move it and continue
            // if it doesn't exist, we continue and try to find an immovable instead
            match mov.get(&pos) {
                Some(id) => to_move.push((key, id.clone())),
                None => {
                    // find an immovable
                    // if it exists, we need to stop and not move anything
                    // if it doesn't exist, we stop because we found a gap
                    match immov.get(&pos) {
                        Some(_id) => to_move.clear(),
                        None => break,
                    }
                }
            }
        }
    }

    // Now actually move what needs to be moved
    for (key, id) in to_move {
        let position = positions.get_mut(entities.entity(id));
        if let Some(position) = position {
            match key {
                KeyCode::Up => position.y -= 1,
                KeyCode::Down => position.y += 1,
                KeyCode::Left => position.x -= 1,
                KeyCode::Right => position.x += 1,
                _ => (),
            }
        }
    }
}
// ANCHOR_END: input_system

// ANCHOR: main
pub fn main() -> GameResult {
    let mut world = World::new();
    initialize_level(&mut world);

    // Create a game context and event loop
    let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (context, event_loop) = context_builder.build()?;

    // Create the game state
    let game = Game { world };
    // Run the main event loop
    event::run(context, event_loop, game)
}
// ANCHOR_END: main

/* ANCHOR_END: all */