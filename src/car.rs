mod body;
mod spawner;
mod wheels;

use bevy::prelude::*;
use body::CarBodyPlugin;
use spawner::CarSpawnerPlugin;
use wheels::CarWheelsPlugin;

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CarBodyPlugin, CarWheelsPlugin, CarSpawnerPlugin));
    }
}
