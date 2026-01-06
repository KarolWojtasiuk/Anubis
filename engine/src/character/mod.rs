use crate::prelude::*;

pub mod player;

#[derive(Default)]
pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(player::PlayerPlugin);
    }
}

#[derive(Component, Default)]
#[require(Transform, RigidBody::Dynamic, Speed(50))]
pub struct Character; // TODO: Id

#[derive(Component, Default)]
pub struct Speed(pub u32);
