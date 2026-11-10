//! Tests for code/bevy/headless-app, on tiny plugins that count ticks and burn a fuse.
use std::time::Duration;

use bevy::prelude::*;
use headless::*;

/// Ticks of FixedUpdate, frames of Update, and whether Startup ran.
#[derive(Resource, Default)]
struct Counts {
    fixed: u32,
    frames: u32,
    started: bool,
    /// Time<Virtual>'s delta in each Update.
    deltas: Vec<Duration>,
}

struct CountPlugin;

impl Plugin for CountPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Counts>()
            .add_systems(Startup, |mut c: ResMut<Counts>| c.started = true)
            .add_systems(FixedUpdate, |mut c: ResMut<Counts>| c.fixed += 1)
            .add_systems(Update, |mut c: ResMut<Counts>, time: Res<Time>| {
                c.frames += 1;
                c.deltas.push(time.delta());
            });
    }
}

fn counts(app: &App) -> &Counts {
    app.world().resource::<Counts>()
}

#[test]
fn one_update_is_one_fixed_tick_at_any_rate() {
    for hz in [
        None,
        Some(60.0),
        Some(30.0),
        Some(144.0),
        Some(1000.0 / 7.0),
        Some(2.0),
    ] {
        let mut app = headless_app_with(|app| {
            app.add_plugins(CountPlugin);
            if let Some(hz) = hz {
                app.insert_resource(Time::<Fixed>::from_hz(hz));
            }
        });
        let c = counts(&app);
        assert!(c.started, "{hz:?}: Startup ran");
        assert_eq!(
            (c.fixed, c.frames),
            (0, 1),
            "{hz:?}: the first update only starts the clock"
        );
        for n in 1..=50 {
            app.update();
            assert_eq!(counts(&app).fixed, n, "{hz:?}: update {n}");
        }
        let timestep = app.world().resource::<Time<Fixed>>().timestep();
        assert_eq!(sim_time(&app), timestep * 50, "{hz:?}");
        // Update sees the same step, so games that run their rules in Update test the same way.
        assert!(
            counts(&app).deltas[1..].iter().all(|d| *d == timestep),
            "{hz:?}"
        );
    }
}

#[test]
fn the_game_can_set_or_change_the_rate() {
    // Set in the plugin's build, as most games do.
    struct Rate20;
    impl Plugin for Rate20 {
        fn build(&self, app: &mut App) {
            app.insert_resource(Time::<Fixed>::from_hz(20.0));
        }
    }
    let mut app = headless_app((CountPlugin, Rate20));
    step(&mut app, 10);
    assert_eq!(counts(&app).fixed, 10);
    assert_eq!(sim_time(&app), Duration::from_millis(500));

    // Set in a Startup system.
    let mut app = headless_app_with(|app| {
        app.add_plugins(CountPlugin)
            .add_systems(Startup, |mut t: ResMut<Time<Fixed>>| {
                t.set_timestep_hz(25.0)
            });
    });
    step(&mut app, 25);
    assert_eq!(counts(&app).fixed, 25);
    assert_eq!(sim_time(&app), Duration::from_secs(1));

    // Changed mid-run, slower than Time<Virtual>'s 250 ms cap.
    app.world_mut()
        .resource_mut::<Time<Fixed>>()
        .set_timestep_hz(2.0);
    allow_long_steps(&mut app);
    step(&mut app, 4);
    assert_eq!(counts(&app).fixed, 29);
    assert_eq!(sim_time(&app), Duration::from_secs(3));
}

#[test]
fn step_secs_rounds_up_to_whole_ticks() {
    let mut app = headless_app_with(|app| {
        app.add_plugins(CountPlugin)
            .insert_resource(Time::<Fixed>::from_hz(60.0));
    });
    assert_eq!(ticks_for(&app, 1.0), 60);
    assert_eq!(ticks_for(&app, 0.0), 0);
    assert_eq!(ticks_for(&app, 0.01), 1);
    assert_eq!(ticks_for(&app, 0.5), 30);
    assert_eq!(step_secs(&mut app, 2.0), 120);
    assert_eq!(counts(&app).fixed, 120);
    let app64 = headless_app(CountPlugin);
    assert_eq!(ticks_for(&app64, 1.0), 64, "Bevy's default rate");
}

#[test]
fn whole_seconds_give_the_nominal_count_at_common_rates() {
    // Time<Fixed> rounds the step to whole nanoseconds: a hair short of 1/hz at 30, 90, 120, 144
    // and 165 Hz, a hair long at 24, 60 and 240.
    for hz in [24u32, 30, 60, 64, 90, 120, 144, 165, 240] {
        let mut app = headless_app_with(|app| {
            app.add_plugins(CountPlugin)
                .insert_resource(Time::<Fixed>::from_hz(hz as f64));
        });
        let step = app
            .world()
            .resource::<Time<Fixed>>()
            .timestep()
            .as_secs_f64();
        for secs in [1u32, 2, 5, 10, 100, 3600] {
            let n = secs * hz;
            let s = secs as f64;
            assert_eq!(ticks_for(&app, s), n, "{hz} Hz, {secs} s");
            // Still rounds up: half a tick more is one more tick, half a tick less is the same.
            assert_eq!(
                ticks_for(&app, s + step / 2.0),
                n + 1,
                "{hz} Hz, {secs} s + half"
            );
            assert_eq!(
                ticks_for(&app, s - step / 2.0),
                n,
                "{hz} Hz, {secs} s - half"
            );
        }
        assert_eq!(step_secs(&mut app, 2.0), 2 * hz);
        assert_eq!(counts(&app).fixed, 2 * hz, "{hz} Hz");
        let off = sim_time(&app).abs_diff(Duration::from_secs(2));
        assert!(
            off.as_nanos() <= (2 * hz) as u128,
            "{hz} Hz: {off:?} from 2 s"
        );
    }
    // A rate whose step doesn't divide a second still rounds up: 5 s at 7 ms ticks is 714.3.
    let app = headless_app_with(|app| {
        app.insert_resource(Time::<Fixed>::from_hz(1000.0 / 7.0));
    });
    assert_eq!(ticks_for(&app, 5.0), 715);
    assert_eq!(ticks_for(&app, 7.0), 1000);
}

/// Burns down one unit per fixed tick and lights at zero.
#[derive(Resource)]
struct Fuse {
    left: u32,
    lit: bool,
}

#[derive(Message, Clone, Debug, PartialEq)]
struct Bang(u32);

fn burn(mut fuse: ResMut<Fuse>, mut bangs: MessageWriter<Bang>, time: Res<Time>) {
    if fuse.left > 0 {
        fuse.left -= 1;
        if fuse.left == 0 {
            fuse.lit = true;
            bangs.write(Bang(time.elapsed().as_millis() as u32));
        }
    }
}

fn fuse_app(length: u32) -> App {
    headless_app_with(|app| {
        app.insert_resource(Fuse {
            left: length,
            lit: false,
        })
        .insert_resource(Time::<Fixed>::from_hz(50.0))
        .add_message::<Bang>()
        .add_systems(FixedUpdate, burn);
    })
}

fn lit(app: &mut App) -> bool {
    app.world().resource::<Fuse>().lit
}

#[test]
fn run_until_stops_at_the_right_tick() {
    let mut app = fuse_app(37);
    assert_eq!(run_until(&mut app, 1000, lit), Some(37));
    assert_eq!(sim_time(&app), Duration::from_millis(37 * 20));
    // Already true: no tick runs.
    assert_eq!(run_until(&mut app, 1000, lit), Some(0));
    assert_eq!(sim_time(&app), Duration::from_millis(37 * 20));
}

#[test]
fn run_until_gives_up_at_the_max() {
    let mut app = fuse_app(37);
    assert_eq!(run_until(&mut app, 36, lit), None);
    assert_eq!(
        sim_time(&app),
        Duration::from_millis(36 * 20),
        "ran exactly max_ticks"
    );
    assert_eq!(run_until(&mut app, 1, lit), Some(1));
    let mut never = 0;
    assert_eq!(
        run_until(&mut fuse_app(5), 0, |_| {
            never += 1;
            false
        }),
        None
    );
    assert_eq!(never, 1, "checked once, before any tick");
}

#[test]
fn record_catches_messages_written_in_fixed_update_once_each() {
    let mut app = fuse_app(10);
    record::<Bang>(&mut app);
    record::<Bang>(&mut app); // a second call changes nothing
    step(&mut app, 30);
    assert_eq!(
        recorded::<Bang>(&app),
        [Bang(200)],
        "fixed time at the tenth tick"
    );
    assert_eq!(take_recorded::<Bang>(&mut app), [Bang(200)]);
    step(&mut app, 5);
    assert!(recorded::<Bang>(&app).is_empty());
}

/// A walker that moves by seeded random steps each tick, to compare two runs.
#[derive(Component, Clone, Copy, PartialEq, Debug)]
struct Walker(i64, i64);

#[derive(Resource)]
struct Seed(u64);

fn wander(mut seed: ResMut<Seed>, mut q: Query<(Entity, &mut Walker)>, mut commands: Commands) {
    // A plain LCG, enough to make every run different unless the clock repeats exactly.
    let mut next = || {
        seed.0 = seed
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (seed.0 >> 33) as i64
    };
    let mut walkers: Vec<_> = q.iter_mut().collect();
    walkers.sort_by_key(|(e, _)| *e);
    for (_, w) in &mut walkers {
        w.0 += next() % 7 - 3;
        w.1 += next() % 7 - 3;
    }
    if next() % 10 == 0 {
        commands.spawn(Walker(0, 0));
    }
}

fn walk(seed: u64, ticks: u32) -> (Vec<Walker>, Duration) {
    let mut app = headless_app_with(|app| {
        app.insert_resource(Seed(seed))
            .add_systems(FixedUpdate, wander);
    });
    with_commands(&mut app, |c| {
        c.spawn(Walker(0, 0));
    });
    step(&mut app, ticks);
    let world = app.world_mut();
    let mut q = world.query::<(Entity, &Walker)>();
    let mut all: Vec<_> = q.iter(world).collect();
    all.sort_by_key(|(e, _)| *e);
    (all.into_iter().map(|(_, w)| *w).collect(), sim_time(&app))
}

#[test]
fn two_runs_from_the_same_seed_end_the_same() {
    let a = walk(2026, 500);
    let b = walk(2026, 500);
    assert!(a.0.len() > 10, "walkers were spawned: {}", a.0.len());
    assert_eq!(a, b);
    assert_ne!(a.0, walk(2027, 500).0);
}

#[test]
fn with_commands_applies_at_once_and_single_finds_it() {
    let mut app = headless_app(CountPlugin);
    let e = with_commands(&mut app, |c| c.spawn(Walker(3, 4)).id());
    assert_eq!(app.world().get::<Walker>(e), Some(&Walker(3, 4)));
    assert_eq!(single::<Walker>(&mut app), e);
}

#[test]
#[should_panic(expected = "found 2")]
fn single_panics_on_two() {
    let mut app = headless_app(CountPlugin);
    with_commands(&mut app, |c| {
        c.spawn(Walker(0, 0));
        c.spawn(Walker(1, 1));
    });
    single::<Walker>(&mut app);
}

/// Like Avian: a resource its systems need only exists after `finish`.
struct LateSetup;

#[derive(Resource)]
struct Ready(u32);

impl Plugin for LateSetup {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, |mut r: ResMut<Ready>| r.0 += 1);
    }
    fn finish(&self, app: &mut App) {
        app.insert_resource(Ready(0));
    }
    fn cleanup(&self, app: &mut App) {
        app.world_mut().resource_mut::<Ready>().0 = 100;
    }
}

#[test]
fn plugins_get_finish_and_cleanup() {
    let mut app = headless_app(LateSetup);
    step(&mut app, 3);
    assert_eq!(app.world().resource::<Ready>().0, 103);
}
