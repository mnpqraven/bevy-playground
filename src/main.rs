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
const MOVE_SPEED: i32 = 1;

/// snake game following guide from: https://mbuffett.com/posts/bevy-snake-tutorial/
fn main() {
    // main builder for game logic
    App::new()
        // RESOURCE ===========================================================
        .insert_resource(WindowDescriptor {
            title: "Othi's snake game".to_string(),
            width: RESOLUTION.0,
            height: RESOLUTION.1,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(SnakeTailVec::default())
        .insert_resource(LastTailPosition::default())
        .insert_resource(LastFacingDirection::default())
        // PLUGINS ============================================================
        .add_plugins(DefaultPlugins)
        // STARTUP ============================================================
        .add_startup_system(startup_system)
        .add_startup_system(startup_camera)
        .add_startup_system(spawn_snake)
        // SYSTEM =============================================================
        .add_system(snake_movement_input.before(logic_snake_movement))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.15))
                .with_system(logic_snake_movement)
                .with_system(logic_collision_wall.after(logic_snake_movement))
                .with_system(
                    logic_collision_tail
                        .after(logic_snake_movement)
                        .before(logic_consume),
                )
                .with_system(logic_consume.after(logic_snake_movement))
                .with_system(
                    logic_snake_growth
                        .after(logic_snake_movement)
                        .after(logic_consume),
                ),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_tl)
                .with_system(math_size_scale),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.5))
                .with_system(spawn_food),
        )
        .add_system(game_over.after(logic_snake_movement))
        // EVENT ==============================================================
        .add_event::<GrowthEvent>()
        .add_event::<GameOverEvent>()
        .run();
}

// ENTITY =====================================================================

/// RESOURCE
/// movement direction
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
impl Default for Direction {
    fn default() -> Self {
        Direction::Right
    }
}
impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}
#[derive(Default, Deref, DerefMut)]
struct SnakeTailVec(Vec<Entity>);
/// Res, Event
struct GrowthEvent;
/// Res
#[derive(Default)]
struct LastTailPosition(Option<Position>);
#[derive(Default)]
struct LastFacingDirection(Direction);
/// Res, Event
struct GameOverEvent;

// COMPONENT ==================================================================
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
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
/// Snake CP
/// User-controlled component
#[derive(Component)]
struct Snake {
    direction: Direction,
}
/// SnakeTail CP
/// Snake body part that can follow the snake head
#[derive(Component)]
struct SnakeTail;

/// Food CP
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

fn spawn_snake(
    mut commands: Commands,
    mut tails: ResMut<SnakeTailVec>,
    mut last_facing_direction: ResMut<LastFacingDirection>,
) {
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
    *last_facing_direction = LastFacingDirection::default();
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
fn snake_movement_input(
    kb_input: Res<Input<KeyCode>>,
    mut head_position: Query<&mut Snake>,
    last_facing_direction: Res<LastFacingDirection>,
) {
    if let Some(mut head) = head_position.iter_mut().next() {
        fn flush_input(
            intent_direction: Direction,
            last_direction: Direction,
        ) -> Direction {
            if intent_direction == last_direction.opposite() {
                intent_direction.opposite()
            } else {
                intent_direction
            }
        }
        let dir: Direction = match kb_input.get_pressed().next() {
            Some(KeyCode::A) => flush_input(Direction::Left, last_facing_direction.0),
            Some(KeyCode::S) => flush_input(Direction::Right, last_facing_direction.0),
            Some(KeyCode::W) => flush_input(Direction::Up, last_facing_direction.0),
            Some(KeyCode::R) => flush_input(Direction::Down, last_facing_direction.0),
            _ => head.direction,
        };
        // update head
        head.direction = dir;
    }
}
fn logic_snake_movement(
    mut heads: Query<(Entity, &Snake)>,
    tails: ResMut<SnakeTailVec>,
    mut positions: Query<&mut Position>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut last_facing_direction: ResMut<LastFacingDirection>,
) {
    // grabbing the head, we only have a single heads so next() gives the one
    // we want
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        // struct SnakeTailVec(Vec<Entity>)
        // for all entities in SnakeTailVec, get <Position> then save as new vec
        let tail_positions: Vec<Position> = tails
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap()) // query of Position
            .collect::<Vec<Position>>();
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        match &head.direction {
            Direction::Left => head_pos.x -= MOVE_SPEED,
            Direction::Right => head_pos.x += MOVE_SPEED,
            Direction::Up => head_pos.y += MOVE_SPEED,
            Direction::Down => head_pos.y -= MOVE_SPEED,
        }
        // ???
        tail_positions
            .iter()
            .zip(tails.iter().skip(1))
            .for_each(|(pos, tails)| {
                *positions.get_mut(*tails).unwrap() = *pos;
            });
        // updates fields
        last_facing_direction.0 = head.direction;
        *last_tail_position = LastTailPosition(Some(*tail_positions.last().unwrap()));
    }
}
/// logic @hitting a wall
fn logic_collision_wall(mut heads: Query<(&mut Position, &Snake)>) {
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
fn logic_consume(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    head_positions: Query<&Position, With<Snake>>,
    food_positions: Query<(Entity, &Position), With<Food>>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            // when head is on the same tile as food
            if food_pos == head_pos {
                // triggers GrowthEvent
                commands.entity(ent).despawn();
                growth_writer.send(GrowthEvent);
            }
        }
    }
}
fn logic_snake_growth(
    commands: Commands,
    last_tail_position: ResMut<LastTailPosition>,
    mut tails: ResMut<SnakeTailVec>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    // a GrowthEvent exists
    if growth_reader.iter().next().is_some() {
        tails.push(spawn_tail(commands, last_tail_position.0.unwrap()));
        println!("logic_snake_growth");
    }
}
fn logic_collision_tail(
    heads: Query<(Entity, &Snake)>,
    tails: ResMut<SnakeTailVec>,
    mut positions: Query<&mut Position>,
    mut game_over_writer: EventWriter<GameOverEvent>,
) {
    if let Some((head_entity, _)) = heads.iter().next() {
        let tail_positions: Vec<Position> = tails
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap()) // query of Position
            .collect::<Vec<Position>>();
        println!("{:?}", tail_positions);
        let head_pos = positions.get(head_entity).unwrap();
        if let Some((_, tail_vec)) = tail_positions.split_first() {
            if tail_vec.contains(&head_pos) {
                game_over_writer.send(GameOverEvent);
            }
        }
    }
}
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
fn game_over(
    mut commands: Commands,
    mut event_reader: EventReader<GameOverEvent>,
    tails_res: ResMut<SnakeTailVec>,
    last_facing_direction: ResMut<LastFacingDirection>,
    food: Query<Entity, With<Food>>,
    tails: Query<Entity, With<SnakeTail>>,
) {
    if event_reader.iter().next().is_some() {
        println!("GameOverEvent");
        // clears all food and tails on screen
        for ent in food.iter().chain(tails.iter()) {
            commands.entity(ent).despawn();
        }
        spawn_snake(commands, tails_res, last_facing_direction);
    }
}
