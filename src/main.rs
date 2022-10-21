use bevy::prelude::*;

// Plugins
pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        // add_startup_system: run once, before every other system
        // add_(startup_)system: adds the system to a `Schedule`
        // NOTE: add_system calls are run in parallel, not in order
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people)
            .add_system(greet_people);
    }
}

fn main() {
    App::new()
        // adds engine stuff (rendered, asset loading, ui, windows, input)
        // including event loop
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}

/// ECS system

/// Components
/// Rust structs that implement the Component trait
#[derive(Component)]
struct Person;
#[derive(Component)]
struct Name(String);

/// Systems
/// Normal Rust functions
/// adding people to our World
fn add_people(mut commands: Commands) {
    let people = ["Othi", "Bthi", "Cthi"];
    for person in people {
        commands
            .spawn()
            .insert(Person)
            .insert(Name(person.to_string()));
    }
}
struct GreetTimer(Timer);
fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}", name.0);
        }
    }
}

// Entities
// Simple type containing unique integer
// Struct Entity(u64);
