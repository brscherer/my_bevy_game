use bevy::prelude::*;

// This resource tracks the game's score
#[derive(Resource, Deref, DerefMut)]
pub struct Score(pub usize);

// Stores the sound played when a collision occurs
#[derive(Resource, Deref)]
pub struct CollisionSound(pub Handle<AudioSource>);