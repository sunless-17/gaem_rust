use bevy::prelude::*;

// main app launch
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_people)
        // chain overrides the default parallelism of bevy
        .add_systems(Update, (hello_world, (update_people, greet_people).chain()))
        .run();
}

// testing my first system
fn hello_world() {
    println!("hello world!");
}

// first component
#[derive(Component)]
struct Person;
// "break down" instead of adding it as a field to the component, okay?
#[derive(Component)]
struct Name(String);
// system to "spawn" components
fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name(String::from("Elaina Proctor"))));
    commands.spawn((Person, Name(String::from("Renzo Hume"))));
    commands.spawn((Person, Name(String::from("Zayna Nieves"))));
}

// first query for the system
fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

// update query by iterating with for loop and breaking if the "if" condition is met
fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break; // We don't need to change any other names.
        }
    }
}
