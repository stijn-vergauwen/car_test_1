use bevy::prelude::*;

pub struct CarBodyPlugin;

impl Plugin for CarBodyPlugin {
    #[allow(unused)]
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, ());
    }
}

#[derive(Component, Clone, Copy)]
pub struct CarBody;