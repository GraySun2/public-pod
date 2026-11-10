//! Fly the six-axis model in Bevy: one ship, a chase camera and a field of marker cubes.
//! cargo run --release (from this folder)
//!
//! Keys: W main, S retro (the brake with Flight Assist on), Space / Ctrl lift and drop,
//! A / D strafe, Up / Down arrows nose down / up (like a flight stick), Left / Right yaw,
//! Q / E roll, Shift boost (with A, D, Space or Ctrl held it jinks, and with S too when
//! Flight Assist is off), X stop the spin (FA off), F toggle Flight Assist.
//! Needs Rust 1.97.1 for Bevy 0.20 (or cargo run --release --ignore-rust-version).
//!
//! The flight model is z-up; Bevy is y-up. `to_bevy` turns one into the other, so the model
//! stays engine-free and the game keeps Bevy's axes. The ship mesh is built with its nose
//! along +X and its deck up along +Y.

#[path = "../src/flight6.rs"]
#[allow(dead_code)]
mod flight6;

use bevy::prelude::*;
use flight6::{Body, Controls, Ctx, Engine, FlyState, Tuning, V3, step6};
use std::f32::consts::FRAC_PI_2;

#[derive(Component)]
struct Ship {
    fly: FlyState,
    body: Body,
}

/// The pilot's controls, read each frame and used by the next fixed tick.
#[derive(Resource, Default)]
struct Stick(Controls);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .init_resource::<Stick>()
        .add_systems(Startup, setup)
        .add_systems(Update, (read_keys, chase_camera))
        .add_systems(FixedUpdate, fly)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let (engine, tuning) = (Engine::default(), Tuning::default());
    commands.spawn((
        Ship {
            fly: FlyState::new(0.0, &engine, &tuning),
            body: Body { pos: V3::ZERO, vel: V3::ZERO, fuel: 1000.0 },
        },
        Mesh3d(meshes.add(Cuboid::new(4.0, 1.0, 3.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.9, 0.5, 0.2))),
        Transform::default(),
    ));
    // Marker cubes, so speed and turns can be seen.
    let marker = meshes.add(Cuboid::new(2.0, 2.0, 2.0));
    let grey = materials.add(Color::srgb(0.6, 0.6, 0.7));
    for i in -10..=10 {
        for j in -10..=10 {
            for k in -2..=2 {
                commands.spawn((
                    Mesh3d(marker.clone()),
                    MeshMaterial3d(grey.clone()),
                    Transform::from_xyz(i as f32 * 60.0, k as f32 * 60.0, j as f32 * 60.0),
                ));
            }
        }
    }
    commands.spawn((DirectionalLight::default(), Transform::from_xyz(1.0, 3.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y)));
    commands.spawn((Camera3d::default(), Transform::from_xyz(-20.0, 6.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y)));
}

fn read_keys(keys: Res<ButtonInput<KeyCode>>, mut stick: ResMut<Stick>, mut ships: Query<&mut Ship>) {
    let held = |k: KeyCode| if keys.pressed(k) { 1.0 } else { 0.0 };
    let pair = |plus: KeyCode, minus: KeyCode| held(plus) - held(minus);
    stick.0 = Controls {
        pitch: pair(KeyCode::ArrowDown, KeyCode::ArrowUp),
        roll: pair(KeyCode::KeyE, KeyCode::KeyQ),
        yaw: pair(KeyCode::ArrowRight, KeyCode::ArrowLeft),
        main: held(KeyCode::KeyW),
        retro: held(KeyCode::KeyS),
        lift: pair(KeyCode::Space, KeyCode::ControlLeft),
        strafe: pair(KeyCode::KeyD, KeyCode::KeyA),
        boost: keys.pressed(KeyCode::ShiftLeft),
        kill_rot: keys.pressed(KeyCode::KeyX),
        no_grip: false,
    };
    if keys.just_pressed(KeyCode::KeyF) {
        for mut ship in &mut ships {
            ship.fly.fa = !ship.fly.fa;
            info!("Flight Assist {}", if ship.fly.fa { "on" } else { "off" });
        }
    }
}

/// One fixed tick of the flight model, copied onto the ship's transform.
fn fly(time: Res<Time>, stick: Res<Stick>, mut ships: Query<(&mut Ship, &mut Transform)>) {
    let cx = Ctx::open(time.delta_secs());
    for (mut ship, mut tf) in &mut ships {
        let Ship { fly, body } = &mut *ship;
        step6(fly, body, &stick.0, &cx);
        *tf = to_bevy(body.pos, fly.att.quat());
    }
}

/// The model's z-up position and attitude as a y-up Bevy transform.
fn to_bevy(p: V3, q: [f32; 4]) -> Transform {
    // C turns z-up into y-up: (x, y, z) -> (x, z, -y).
    let c = Quat::from_rotation_x(-FRAC_PI_2);
    let q = Quat::from_xyzw(q[0], q[1], q[2], q[3]);
    Transform::from_translation(Vec3::new(p.x, p.z, -p.y)).with_rotation(c * q * c.inverse())
}

/// A camera behind and above the ship, turning with it.
fn chase_camera(ships: Query<&Transform, With<Ship>>, mut cams: Query<&mut Transform, (With<Camera3d>, Without<Ship>)>) {
    let (Ok(ship), Ok(mut cam)) = (ships.single(), cams.single_mut()) else { return };
    let back = ship.rotation * Vec3::new(-20.0, 6.0, 0.0);
    *cam = Transform::from_translation(ship.translation + back).looking_at(ship.translation, ship.rotation * Vec3::Y);
}

#[cfg(test)]
mod tests {
    use super::*;
    use flight6::Attitude;

    #[test]
    fn to_bevy_keeps_nose_deck_and_wing() {
        // Model z-up (x, y, z) is Bevy y-up (x, z, -y), for points and for the ship's axes.
        let b = |v: V3| Vec3::new(v.x, v.z, -v.y);
        for (h, p, roll) in [(0.0, 0.0, 0.0), (0.7, 0.3, 0.0), (-2.0, -1.1, 0.9), (3.0, 1.4, -2.5)] {
            let mut a = Attitude::from_heading_pitch(h, p);
            a.rotate(V3::new(roll, 0.0, 0.0), 1.0);
            let tf = to_bevy(V3::new(1.0, 2.0, 3.0), a.quat());
            assert!(tf.translation.distance(Vec3::new(1.0, 3.0, -2.0)) < 1e-5);
            // The mesh's nose is +X, its deck up +Y, so its right wing is +Z.
            assert!((tf.rotation * Vec3::X).distance(b(a.f)) < 1e-4, "nose at {h} {p} {roll}");
            assert!((tf.rotation * Vec3::Y).distance(b(a.u)) < 1e-4, "deck at {h} {p} {roll}");
            assert!((tf.rotation * Vec3::Z).distance(b(a.r())) < 1e-4, "wing at {h} {p} {roll}");
        }
    }
}
