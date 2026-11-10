# Module: Bevy headless test app

A windowless Bevy app for tests, bots and self-tests: `MinimalPlugins` plus the game's own plugins, on a fixed clock where each `app.update()` is exactly one `FixedUpdate` tick. Game rules then test with no window, no GPU and no real time passing, and the same inputs give the same run every time. Six of Carter's Bevy games build this by hand (Cowboys & Aliens, Cell Block, Slapstick Chariots, Volan's Nebula, Sharp Behind, Tide-Bound); this is the one to copy from now on.

```
id: bevy-headless-app
engine: Bevy 0.20 (also passes unchanged on Bevy 0.19.1); Rust 1.97.1 or later, Bevy 0.20's minimum (Bevy 0.19.1 needs 1.95)
language: Rust
version: 0.1
licence: MIT (LICENSE) for src/, tests/ and examples/; this Markdown CC BY-NC 4.0 (LICENSE-CONTENT)
source: made general from the original studio's game code (private repos; see PROVENANCE.md)
depends on: the bevy crate, 0.20, with only its std feature; no other modules
owns: building a windowless App (MinimalPlugins, the game's plugins, finish() and cleanup(), one update for Startup); a clock that runs one FixedUpdate tick per update at whatever rate Time<Fixed> holds; step, step_secs, ticks_for, run_until and sim_time; with_commands and single for setting up and finding entities; record, recorded and take_recorded for checking messages
does not own: the game's plugins and what a test checks; windows, rendering, assets and audio (add AssetPlugin and the like in setup if a rule needs them); faking keyboard, mouse or pad input (bevy-action-map's tests show how); waiting on real time for asset loading
used by: none here
copies: none
tested: cargo +1.97.1 test in tests/ on 2026-10-27, Rust 1.97.1, Bevy 0.20.0: 11 passed (one tick per update at six rates including 2 Hz, rates set in a plugin, a Startup system and mid-run, whole seconds giving the nominal tick count at nine rates from 24 to 240 Hz, run_until's stop and give-up, messages from FixedUpdate recorded once, two seeded runs identical, finish() and cleanup() called); the same 11 pass on Bevy 0.19.1 and with Bevy's multi_threaded feature on; cargo clippy --all-targets clean; examples/fuse.rs runs; swapping the clock for ManualDuration(1/60 s) at Bevy's default 64 Hz fails 5 of the tests, and dropping ticks_for's allowance, cutting it to a quarter of a nanosecond or tripling it fails one
```

## Use it in a game

1. Copy `src/headless.rs` into the game, usually as `tests/common/mod.rs` so every integration test can share it. Your plugins must be reachable from `tests/`, so the game needs a library target (`src/lib.rs`) that exports them, as Cowboys & Aliens and Slapstick Chariots have. For a bot or a `--selftest` flag inside the game itself (Cell Block's), put it in `src/` instead.
2. In each test file:

   ```rust
   #[allow(dead_code)]   // not every test uses every helper
   mod common;
   use bevy::prelude::*;
   use common::*;
   use my_game::{GamePlugin, Score, Goal};

   #[test]
   fn a_goal_scores_once() {
       let mut app = headless_app_with(|app| {
           app.add_plugins(GamePlugin)
               .insert_resource(Time::<Fixed>::from_hz(60.0));   // or let GamePlugin set it
       });
       record::<Goal>(&mut app);
       step_secs(&mut app, 3.0);                                 // 180 ticks
       let ticks = run_until(&mut app, 600, |app| app.world().resource::<Score>().0 == 1);
       assert!(ticks.is_some(), "no goal within 10 s");
       assert_eq!(recorded::<Goal>(&app).len(), 1);
   }
   ```

3. Add what your rules need in `setup`: `TransformPlugin` for transform propagation, `StatesPlugin` for `States`, `AssetPlugin::default()` plus `MeshPlugin` when systems touch meshes, Avian's `PhysicsPlugins` for physics. The helper calls `finish()` and `cleanup()` for you, which Avian needs before the first update.

The app comes back after one update: `Startup` has run, the clock has started and no `FixedUpdate` tick has run yet. From then on, every `app.update()` (or `step(&mut app, 1)`) is one tick, and `Time` in `Update` moves on by the same step, so games that run their rules in `Update` test the same way. [examples/fuse.rs](examples/fuse.rs) is the smallest case.

## What it gives you

| Name | What it is |
| --- | --- |
| `headless_app(plugins)` | A headless app with these plugins added after `MinimalPlugins` |
| `headless_app_with(setup)` | The same, with a closure that adds plugins, resources and systems before `finish()` |
| `step(app, ticks)` | Run that many fixed ticks, one update each |
| `step_secs(app, secs)` | Run the whole number of ticks that covers `secs` of game time (rounded up, so 1 s at 120 Hz is 120 ticks); returns the count |
| `ticks_for(app, secs)` | That count, without running |
| `run_until(app, max_ticks, done)` | Tick until `done(app)` holds: `Some(ticks)` (0 if it already held), or `None` after `max_ticks` |
| `sim_time(app)` | Game time on the fixed clock, as a `Duration` |
| `allow_long_steps(app)` | Lets a clock slower than 4 Hz tick every update (already called once by the builders) |
| `with_commands(app, f)` | Run `f` with `Commands` and apply them at once, to spawn with the game's own helpers |
| `single::<C>(app)` | The one entity with component `C`; panics with the count otherwise |
| `record::<M>(app)` | Keep every `M` message written from now on, `FixedUpdate` ones included |
| `recorded::<M>(app)`, `take_recorded::<M>(app)` | The messages kept so far; the second also empties the record |

## Limits

- The clock uses `TimeUpdateStrategy::FixedTimesteps(1)`, which the builders insert after your `setup`; a `TimeUpdateStrategy` you insert in `setup` is replaced. Pausing or slowing `Time<Virtual>` still pauses or slows the fixed ticks, as in the game.
- Same inputs, same run, only when the game's own systems are deterministic: seed every random number, and order systems that touch the same data (`.chain()`, `.before()`), because Bevy may run unordered systems in either order.
- `run_until` counts ticks, not wall time. To wait for assets that load on other threads, loop on `app.update()` with a wall-clock limit instead' `update_until` does.
- `record::<M>` also keeps `M` messages still buffered from the update or two before it was called.
- `Time<Fixed>` holds its step in whole nanoseconds, so at rates such as 30, 120 or 144 Hz a tick is a fraction of a nanosecond short. `ticks_for` and `step_secs` allow up to 1 ns a tick for that, so whole seconds give the nominal count, and `sim_time` after `step_secs(secs)` can be that much under `secs`.
- No window, so anything that needs one (`Window` queries, cameras that render, screenshots) needs a different harness.

## Tests

`tests/` is a small Cargo package that builds `src/headless.rs` as a library. From `code/bevy/headless-app/tests`:

```
cargo test
cargo run --example fuse
```

Bevy 0.20 needs Rust 1.97.1; if your default toolchain is older, use `cargo +1.97.1 test`. The first build compiles Bevy (about a minute with only the `std` feature); set `CARGO_TARGET_DIR` to a folder shared with other Bevy modules to build it once.

The tests check one fixed tick per update at 64 (the default), 60, 30, 144, about 142.9 and 2 Hz, with `Update`'s delta equal to the step; a rate set in a plugin, in a `Startup` system and changed mid-run; `step_secs` rounding, and whole seconds giving exactly seconds times rate ticks at 24, 30, 60, 64, 90, 120, 144, 165 and 240 Hz (from 1 s to an hour, with half a tick more still rounding up); `run_until` stopping at the exact tick, returning 0 when already done and giving up after exactly `max_ticks`; a message written in `FixedUpdate` recorded once; two seeded runs of 500 ticks that spawn entities ending identical (and a different seed differing); `with_commands` and `single`; and a plugin whose `finish()` and `cleanup()` must run before its systems can.
