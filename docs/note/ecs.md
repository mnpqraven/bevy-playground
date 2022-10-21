# ECS system
- **Entities** are unique "things" that are assigned groups of **Components**, which are then processed using **Systems**

Example:
- An **Entity** possessing `Velocity` and `Position` **Component** (attributes)
- Another **Entity** possessing `UI` and `Position` **Component** (attributes)
- Both entities might or might not get processes by a **System**

## Parts
### Components
- Rust structs that implement the Component trait
- Data (health, mana, movespeed .etc)
- **NO** functionality (walk, fly .etc)
```rust
#[derive(Component)]
struct Person;
```
### Systems
- Normal Rust functions
- Subscribes **Entities** with **Components** and implement logic when there are new/updated/removed entities.

```rust
fn add_people(mut commands: Commands) {
    let people = ["Othi", "Bthi", "Cthi"];
    for person in people {
        commands
            .spawn()
            .insert(Person)
            .insert(Name(person.to_string()));
    }
}
```

### Entities
- Simple type containing unique integer
- Identifier
- Can have one or more **Component**
```rust
Struct Entity(u64);
```

## System loop