//! A windowless Bevy app for tests, bots and tools: `MinimalPlugins` plus the game's own plugins,
//! on a fixed clock where each `app.update()` is exactly one `FixedUpdate` tick. Game rules then
//! test with no window, no GPU and no real time passing, and the same inputs give the same run.
//!
//! Bevy 0.20 (also builds on 0.19.1); needs only Bevy's `std` feature.

use std::time::Duration;

use bevy::app::Plugins;
use bevy::ecs::world::CommandQueue;
use bevy::prelude::*;
use bevy::time::TimeUpdateStrategy;

/// A headless app with `plugins` added after `MinimalPlugins`. See [`headless_app_with`].
pub fn headless_app<M>(plugins: impl Plugins<M>) -> App {
    headless_app_with(|app| {
        app.add_plugins(plugins);
    })
}

/// A headless app: `MinimalPlugins`, then `setup` adds the game's plugins, resources and systems.
/// It returns after one update, which runs `Startup` and only starts the clock, so no
/// `FixedUpdate` tick has run yet. From then on every `app.update()` runs exactly one.
///
/// The tick rate is whatever `Time<Fixed>` holds (Bevy's default is 64 Hz). Set it in `setup`,
/// `app.insert_resource(Time::<Fixed>::from_hz(60.0))`, or let the game's plugin set it.
pub fn headless_app_with(setup: impl FnOnce(&mut App)) -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    setup(&mut app);
    // Each update moves the clock on by exactly one fixed timestep, read afresh every update, so a
    // plugin that sets or changes the rate can't put updates and ticks out of step. This is Bevy's
    // own form of the `ManualDuration(1 / HZ)` the games used, which needs the two kept equal by hand.
    app.insert_resource(TimeUpdateStrategy::FixedTimesteps(1));
    // `App::run` would do these two; some plugins (Avian physics) finish setting up here.
    app.finish();
    app.cleanup();
    // The first update runs Startup. The clock only starts, so no fixed tick runs.
    app.update();
    allow_long_steps(&mut app);
    app
}

/// Virtual time drops the part of a step past its max delta (250 ms by default), which would
/// starve a clock slower than 4 Hz. Raise it to fit one step. Call again after slowing the clock.
pub fn allow_long_steps(app: &mut App) {
    let step = app.world().resource::<Time<Fixed>>().timestep();
    let mut virt = app.world_mut().resource_mut::<Time<Virtual>>();
    if virt.max_delta() < step {
        virt.set_max_delta(step);
    }
}

/// Run `ticks` fixed ticks (one update each).
pub fn step(app: &mut App, ticks: u32) {
    for _ in 0..ticks {
        app.update();
    }
}

/// The whole number of ticks that covers `secs` of game time at the current rate, rounded up.
pub fn ticks_for(app: &App, secs: f64) -> u32 {
    let step = app
        .world()
        .resource::<Time<Fixed>>()
        .timestep()
        .as_secs_f64();
    // Time<Fixed> holds the step in whole nanoseconds, so at 120 Hz it is a third of a nanosecond
    // short and 120 ticks come to 40 ns under 1 s. Allowing up to 1 ns per tick keeps whole seconds
    // at the nominal count (120, not 121) at every rate.
    (secs / (step + 1e-9)).ceil().max(0.0) as u32
}

/// Run `secs` of game time ([`ticks_for`]); returns the ticks run.
pub fn step_secs(app: &mut App, secs: f64) -> u32 {
    let ticks = ticks_for(app, secs);
    step(app, ticks);
    ticks
}

/// Update until `done` holds, checking before the first tick and after each one, for at most
/// `max_ticks`. `Some(ticks run)` when it held (0 if it already did), `None` when it gave up.
pub fn run_until(
    app: &mut App,
    max_ticks: u32,
    mut done: impl FnMut(&mut App) -> bool,
) -> Option<u32> {
    if done(app) {
        return Some(0);
    }
    for ticks in 1..=max_ticks {
        app.update();
        if done(app) {
            return Some(ticks);
        }
    }
    None
}

/// Game time on the fixed clock: every tick run so far, added up.
pub fn sim_time(app: &App) -> Duration {
    app.world().resource::<Time<Fixed>>().elapsed()
}

/// Run `f` with `Commands` on the app's world and apply them at once, so a test can spawn with
/// the game's own bundles and helpers between ticks.
pub fn with_commands<T>(app: &mut App, f: impl FnOnce(&mut Commands) -> T) -> T {
    let world = app.world_mut();
    let mut queue = CommandQueue::default();
    let out = {
        let mut commands = Commands::new(&mut queue, world);
        f(&mut commands)
    };
    queue.apply(world);
    out
}

/// The one entity with component `C`. Panics, saying how many there are, if that isn't one.
pub fn single<C: Component>(app: &mut App) -> Entity {
    let world = app.world_mut();
    let found: Vec<Entity> = world
        .query_filtered::<Entity, With<C>>()
        .iter(world)
        .collect();
    assert_eq!(
        found.len(),
        1,
        "expected one {}, found {}",
        std::any::type_name::<C>(),
        found.len()
    );
    found[0]
}

/// Every `M` message seen since [`record`] was called, oldest first.
#[derive(Resource)]
pub struct Recorded<M: Message>(pub Vec<M>);

/// Keep every `M` message written from now on, from any schedule (`FixedUpdate` too), for checks.
/// Messages still buffered from the update or two before are kept as well.
pub fn record<M: Message + Clone>(app: &mut App) {
    if app.world().contains_resource::<Recorded<M>>() {
        return;
    }
    app.add_message::<M>()
        .insert_resource(Recorded::<M>(Vec::new()))
        .add_systems(
            Last,
            |mut seen: MessageReader<M>, mut out: ResMut<Recorded<M>>| {
                out.0.extend(seen.read().cloned());
            },
        );
}

/// The `M` messages recorded so far (call [`record`] first).
pub fn recorded<M: Message>(app: &App) -> &[M] {
    &app.world().resource::<Recorded<M>>().0
}

/// The `M` messages recorded so far, emptying the record.
pub fn take_recorded<M: Message>(app: &mut App) -> Vec<M> {
    std::mem::take(&mut app.world_mut().resource_mut::<Recorded<M>>().0)
}
