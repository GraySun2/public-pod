//! The smallest use: run a game plugin with no window and check when its rule fires.
//! cargo run --example fuse (from ../tests). In a game, the same lines go in a #[test].
use bevy::prelude::*;
use headless::{headless_app, run_until, sim_time};

/// Stands in for the game's own plugin: a fuse that burns one unit per fixed tick.
struct FusePlugin;

#[derive(Resource)]
struct Fuse(u32);

impl Plugin for FusePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(60.0))
            .insert_resource(Fuse(90))
            .add_systems(FixedUpdate, |mut f: ResMut<Fuse>| {
                f.0 = f.0.saturating_sub(1)
            });
    }
}

fn main() {
    let mut app = headless_app(FusePlugin);
    let ticks = run_until(&mut app, 600, |app| app.world().resource::<Fuse>().0 == 0);
    assert_eq!(ticks, Some(90));
    let secs = sim_time(&app).as_secs_f32();
    println!("the fuse burned out after {ticks:?} ticks, {secs:.2} s of game time");
}
