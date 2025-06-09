use bevy::prelude::*;
use crate::{
    constants::BACKGROUND_COLOR,
    events::CollisionEvent,
    resources::Score,
    systems::{
        velocity::apply_velocity,
        paddle::move_paddle,
        collision::check_for_collisions,
        audio::play_collision_sound,
        scoreboard::update_scoreboard,
        setup::setup,
    },
};

mod components;
mod constants;
mod events;
mod resources;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Score(0))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_event::<CollisionEvent>()
        .add_systems(Startup, setup)
        // Add our gameplay simulation systems to the fixed timestep schedule
        // which runs at 64 Hz by default
        .add_systems(
            FixedUpdate,
            (
                apply_velocity,
                move_paddle,
                check_for_collisions,
                play_collision_sound,
            )
                // `chain`ing systems together runs them in order
                .chain(),
        )
        .add_systems(Update, update_scoreboard)
        .run();
}
