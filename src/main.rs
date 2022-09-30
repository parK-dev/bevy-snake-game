use bevy::{prelude::*, time::FixedTimestep};
pub mod grid;
mod snake;
pub mod input;

use grid::{position_translation, size_scaling};
use input::{KeyboardInputState, keyboard_input};
use snake::{
    food_spawner, snake_eating, snake_growth, snake_movement, snake_movement_input, spawn_snake,
    Food, GrowthEvent, LastTailPosition, SnakeSegment, SnakeSegments 
};

pub struct GameOverEvent;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Bevy Snake!".to_string(),
            width: 1000.0,
            height: 1000.0,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(SnakeSegments::default())
        .insert_resource(KeyboardInputState::default())
        .insert_resource(LastTailPosition::default())
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_system(keyboard_input.after(snake_movement_input))
        .add_system(snake_movement_input.before(snake_movement))
        .add_system(game_over.after(snake_movement))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.150))
                .with_system(snake_movement)
                .with_system(snake_eating.after(snake_movement))
                .with_system(snake_growth.after(snake_eating)),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(2.0))
                .with_system(food_spawner),
        )
        .add_event::<GrowthEvent>()
        .add_event::<GameOverEvent>()
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

pub fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    segments_res: ResMut<SnakeSegments>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<SnakeSegment>>,
) {
    if reader.iter().next().is_some() {
        for ent in food.iter().chain(segments.iter()) {
            commands.entity(ent).despawn();
        }
        spawn_snake(commands, segments_res);
    }
}
