use bevy::prelude::{App, Commands, Component, Plugin, Query, Res, ResMut, Resource, With};
use bevy::time::{Time, Timer, TimerMode};
use bevy::DefaultPlugins;

//  NOTE:
// Since other entities might have names too,
// it often makes sense to break datatypes up into
// small pieces to encourage code reuse.

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
pub struct GreetTimer(Timer);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        let gtimer = Timer::from_seconds(2.0, TimerMode::Once);
        app.insert_resource(GreetTimer(gtimer))
           .add_startup_system(add_people)
           .add_system(hello_world)
           .add_system(greet_people);
    }
}

//  NOTE:
// Startup systems are just like normal systems,
// but they run exactly once, before all other
// systems, right when our app starts.
fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Harvey Specter".to_string())));
}

//  NOTE:
//  The parameters we pass in to a "system function"
//  define what data the system runs on. In this case,
//  greet_people will run on all
//  entities with the Person and Name component!.
fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    //  INFO:
    // update our timer with the time elapsed
    // since the last update. if that caused the timer to finish,
    // we say hello to everyone.
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            let name = &name.0;
            println!("Welcome Ranger {name}!");
        }
    }
}

//  NOTE:
// "hello world!" might show up in a different order
// than expected. This is because systems run in parallel
// by default whenever possible.
fn hello_world() {
    println!("Hello, world!");
}

fn main() {
    App::new().add_plugins(DefaultPlugins)
              .add_plugin(HelloPlugin)
              .run();
}
