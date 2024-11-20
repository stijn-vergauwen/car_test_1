mod camera;
mod car;
mod world;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use camera::CameraPlugin;
use car::CarPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            CameraPlugin,
            WorldPlugin,
            CarPlugin,
        ))
        .run();
}

/*  --- Project Plans ---

    Goal: A simple car physics simulation
        - WASD controls for driving and steering
        - A simple suspension system that just goes up and down
        - Wheels that carry the cars forces into the ground etc.

        Wheels should be able to slip / drift, and I want FWD / RWD / AWD,
        but the amount of grip each tire has on the ground doesn't need to be realistic

        The camera should smoothly follow the car, following the cars rotation.
        The player should be able to rotate the camera around and the camera should then stay in that adjusted position relative to the car.

*/
