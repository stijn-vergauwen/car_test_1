use bevy::{color::palettes::tailwind::*, prelude::*};
use bevy_rapier3d::prelude::*;

use super::{body::CarBody, wheels::CarWheel};

pub struct CarSpawnerPlugin;

impl Plugin for CarSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_car);
    }
}

fn spawn_car(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let car_size = Vec3::new(1.0, 1.0, 2.4);
    let half_size = car_size / 2.0;

    let car_entity = commands
        .spawn((
            CarBody,
            PbrBundle {
                mesh: meshes.add(Cuboid::from_size(car_size)),
                material: materials.add(StandardMaterial {
                    base_color: RED_500.into(),
                    perceptual_roughness: 1.0,
                    ..default()
                }),
                transform: Transform::from_xyz(0.0, 2.0, 0.0),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::cuboid(half_size.x, half_size.y, half_size.z),
            Sleeping::disabled(),
        ))
        .id();

    let wheel_radius = 0.25;
    let wheel_width = 0.2;
    let wheel_offset = Vec3::new(0.7, -0.5, 0.8);

    let wheel_mesh = meshes.add(Cylinder::new(wheel_radius, wheel_width));

    let wheel_material = materials.add(StandardMaterial {
        base_color: YELLOW_500.into(),
        perceptual_roughness: 1.0,
        ..default()
    });

    let wheel_translations = [
        Vec3::new(-wheel_offset.x, wheel_offset.y, -wheel_offset.z),
        Vec3::new(wheel_offset.x, wheel_offset.y, -wheel_offset.z),
        Vec3::new(-wheel_offset.x, wheel_offset.y, wheel_offset.z),
        Vec3::new(wheel_offset.x, wheel_offset.y, wheel_offset.z),
    ];

    for wheel_translation in wheel_translations {
        spawn_wheel(
            &mut commands,
            wheel_mesh.clone(),
            wheel_material.clone(),
            wheel_translation,
            wheel_width,
            wheel_radius,
            car_entity,
        );
    }
}

#[allow(unused)]
fn spawn_wheel(
    commands: &mut Commands,
    // car_body: &mut ChildBuilder,
    wheel_mesh: Handle<Mesh>,
    wheel_material: Handle<StandardMaterial>,
    wheel_translation: Vec3,
    wheel_width: f32,
    wheel_radius: f32,
    car_entity: Entity,
) {
    let joint = RevoluteJointBuilder::new(Vec3::X)
        .local_anchor1(wheel_translation)
        .local_anchor2(Vec3::ZERO);

    // Wheel anchor
    commands
        .spawn((
            CarWheel,
            SpatialBundle {
                transform: Transform::from_translation(wheel_translation),
                ..default()
            },
            RigidBody::Dynamic,
            ImpulseJoint::new(car_entity, joint),
            Sleeping::disabled(),
        ))
        .with_children(|wheel_anchor| {
            // Wheel body
            wheel_anchor.spawn((
                CarWheel,
                PbrBundle {
                    mesh: wheel_mesh,
                    material: wheel_material,
                    transform: Transform::from_rotation(Quat::from_axis_angle(
                        Vec3::Z,
                        (90.0 as f32).to_radians(),
                    )),
                    ..default()
                },
                Collider::cylinder(wheel_width / 2.0, wheel_radius),
                Restitution::coefficient(0.0),
            ));
        });
}
