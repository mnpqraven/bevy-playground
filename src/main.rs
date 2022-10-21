use bevy::prelude::*;

const SNAKE_COLOR: Color = Color::CRIMSON;
const FOOD_COLOR: Color = Color::ALICE_BLUE;

// TODO: refactor to grid number, pixel calculation should be done separately
const MAP_WIDTH: u32 = 50;
const MAP_HEIGHT: u32 = 50;
const RESOLUTION: (f32, f32) = (500., 500.);

/// snake game following guide from: https://mbuffett.com/posts/bevy-snake-tutorial/
fn main() {
    // main builder for game logic
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Othi's snake game".to_string(),
            width: RESOLUTION.0,
            height: RESOLUTION.1,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup_system)
        .add_startup_system(startup_camera)
        .add_startup_system(spawn_snake)
        .add_system(logic_snake_movement)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_tl)
                .with_system(math_size_scale),
        )
        .run();
}

// E
struct Entity(f32);

// resource
struct Target(Entity);
// C
#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}
#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}
/// The snake
#[derive(Component)]
struct Snake;

// S
fn startup_system(mut commands: Commands) {
    // command needs to be mut
    let center = Position { x: 0, y: 0 };
    commands.spawn().insert(center);
}
fn startup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn spawn_snake(mut commands: Commands) {
    let sprite_snake = SpriteBundle {
        sprite: Sprite {
            color: SNAKE_COLOR,
            ..default()
        },
        transform: Transform {
            scale: Vec3::new(30.0, 30.0, 30.0),
            ..default()
        },
        ..default()
    };
    commands
        .spawn_bundle(sprite_snake)
        .insert(Snake)
        .insert(Position { x: 5, y: 5 })
        .insert(Size::square(0.8));
}
fn logic_snake_movement(
    kb_input: Res<Input<KeyCode>>,
    mut head_position: Query<(&mut Position, With<Snake>)>,
) {
    let grid_per_movement: i32 = 1;
    // TODO: refactor this mess
    for mut pos in head_position.iter_mut() {
        if kb_input.pressed(KeyCode::A) {
            if pos.0.x > 0 {
                pos.0.x -= grid_per_movement;
            }
        }
        if kb_input.pressed(KeyCode::S) {
            if pos.0.x < (RESOLUTION.0 / MAP_WIDTH as f32 * 5f32 - 1f32) as i32 {
            pos.0.x += grid_per_movement;
            }
        }
        if kb_input.pressed(KeyCode::R) {
            if pos.0.y > 0 {
            pos.0.y -= grid_per_movement;
            }
        }
        if kb_input.pressed(KeyCode::W) {
            if pos.0.y < (RESOLUTION.1 / MAP_HEIGHT as f32 * 5f32 - 1f32) as i32 {
            pos.0.y += grid_per_movement;
            }
        }
    }
}
fn math_size_scale(window: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let window = window.get_primary().expect("can't get primary window");
    for (sprite_size, mut transform) in q.iter_mut() {
        // NOTE: what is sprite_size doing here ?
        // logic: something with width 4 in grid of 40, window is 400px
        // > 10px human width
        transform.scale = Vec3::new(
            sprite_size.width / MAP_WIDTH as f32 * window.width() as f32,
            sprite_size.height / MAP_HEIGHT as f32 * window.height() as f32,
            1.,
        );
    }
}
fn position_tl(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    /// ??????????
    fn convert(pos: f32, window: f32, game: f32) -> f32 {
        let tile_size = window / game;
        (pos * window / game) - (window / 2.) + (tile_size / 2.)
    }

    let window = windows.get_primary().expect("cant get primary window");
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width(), MAP_WIDTH as f32),
            convert(pos.y as f32, window.height(), MAP_HEIGHT as f32),
            0.0,
        );
    }
}
