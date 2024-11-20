use bevy::prelude::*;

pub struct CarWheelsPlugin;

impl Plugin for CarWheelsPlugin {
    #[allow(unused)]
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, ());
    }
}

#[derive(Component, Clone, Copy)]
pub struct CarWheel;