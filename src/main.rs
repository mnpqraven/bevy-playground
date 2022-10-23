use bevy::prelude::*;
use bevy::time::FixedTimestep;
use rand::random;

const SNAKE_COLOR: Color = Color::CRIMSON;
const TAIL_COLOR: Color = Color::PINK;
const FOOD_COLOR: Color = Color::GREEN;

// TODO: refactor to grid number, pixel calculation should be done separately
const MAP_WIDTH: i32 = 50;
const MAP_HEIGHT: i32 = 50;
const NODE_ENTITY_SCALE: f32 = 0.8;
const RESOLUTION: (f32, f32) = (500., 500.);
const TPS: f32 = 60. / 60.;
const MOVE_SPEED: i32 = 1;

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
        .insert_resource(SnakeTailVec::default())
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup_system)
        .add_startup_system(startup_camera)
        .add_startup_system(spawn_snake)
        .add_system(snake_movement_input.before(logic_snake_movement))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.15))
                .with_system(logic_snake_movement)
                .with_system(logic_hitting_wall_tail),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_tl)
                .with_system(math_size_scale),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.))
                .with_system(spawn_food),
        )
        .run();
}

// ENTITY =====================================================================

/// RESOURCE
/// movement direction
#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
impl Direction {
    fn opposite(self) -> Self {
        match self {
            Direction::Left => Self::Right,
            Direction::Right => Self::Left,
            Direction::Up => Self::Down,
            Direction::Down => Self::Up,
        }
    }
}
#[derive(Default, Deref, DerefMut)]
struct SnakeTailVec(Vec<Entity>);

// COMPONENT ==================================================================
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
    pub fn square_scale(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}
/// The snake
#[derive(Component)]
struct Snake {
    direction: Direction,
}
#[derive(Component)]
struct SnakeTail;
/// food Component
/// spawns periodically, on random Position
/// despawns when Snake lands on the Position
#[derive(Component)]
struct Food;

// SYSTEM =====================================================================
fn startup_system(mut commands: Commands) {
    // command needs to be mut
    let center = Position { x: 0, y: 0 };
    commands.spawn().insert(center);
}
fn startup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn spawn_snake(mut commands: Commands, mut tails: ResMut<SnakeTailVec>) {
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
    *tails = SnakeTailVec(vec![
        commands
            .spawn_bundle(sprite_snake)
            .insert(Snake {
                direction: Direction::Right,
            })
            .insert(SnakeTail)
            .insert(Position {
                x: MAP_WIDTH as i32 / 2,
                y: MAP_HEIGHT as i32 / 2,
            })
            .insert(Size::square_scale(NODE_ENTITY_SCALE))
            .id(),
        spawn_tail(
            commands,
            Position {
                x: (MAP_WIDTH as i32 / 2) - 1,
                y: MAP_HEIGHT as i32 / 2,
            },
        ),
    ]);
}
fn spawn_food(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            // spawns entity
            sprite: Sprite {
                color: FOOD_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Food) // assigning as Food
        .insert(Position {
            x: (random::<f32>() * MAP_HEIGHT as f32) as i32,
            y: (random::<f32>() * MAP_WIDTH as f32) as i32,
        }) // assigning Position
        .insert(Size::square_scale(NODE_ENTITY_SCALE));
}
fn spawn_tail(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: TAIL_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(SnakeTail)
        .insert(position)
        .insert(Size::square_scale(0.6))
        .id()
}
// TODO: refactor this mess
fn snake_movement_input(kb_input: Res<Input<KeyCode>>, mut head_position: Query<&mut Snake>) {
    if let Some(mut head) = head_position.iter_mut().next() {
        let dir: Direction = if kb_input.pressed(KeyCode::A) {
            Direction::Left
        } else if kb_input.pressed(KeyCode::S) {
            Direction::Right
        } else if kb_input.pressed(KeyCode::R) {
            Direction::Down
        } else if kb_input.pressed(KeyCode::W) {
            Direction::Up
        } else {
            head.direction
        };
        // doesn't allow snake to do do 180
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}
// TODO: tails following head
fn logic_snake_movement(mut heads: Query<(&mut Position, &Snake)>) {
    if let Some((mut head_pos, snake)) = heads.iter_mut().next() {
        // keep going in previous facing direction
        match &snake.direction {
            Direction::Left => head_pos.x -= MOVE_SPEED,
            Direction::Right => head_pos.x += MOVE_SPEED,
            Direction::Up => head_pos.y += MOVE_SPEED,
            Direction::Down => head_pos.y -= MOVE_SPEED,
        }
    }
}
/// logic @hitting a wall
fn logic_hitting_wall_tail(mut heads: Query<(&mut Position, &Snake)>) {
    if let Some((mut head_pos, snake)) = heads.iter_mut().next() {
        // teleport to other side when touching edge
        match (head_pos.x, head_pos.y, snake.direction) {
            (MAP_WIDTH, _, Direction::Right) => head_pos.x = 0,
            (_, MAP_HEIGHT, Direction::Up) => head_pos.y = 0,
            (0, _, Direction::Left) => head_pos.x = MAP_WIDTH,
            (_, 0, Direction::Down) => head_pos.y = MAP_HEIGHT,
            _ => {}
        }
    }
}
/// snake consuming a food entity
fn logic_consume(mut commands: Commands, mut snake: Query<&mut Position, &Snake>) {}
// scales everything up/down to viewport
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
