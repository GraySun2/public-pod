# Module: six-axis flight

A flight model for a ship with six degrees of freedom: it pitches, rolls and yaws freely and thrusts along its nose, deck and wings. Flight Assist gives three ways to fly: arcade handling in open space, a drone-style hover in a hover field (near a landing pad), and pure momentum with it off. One tick function, `step6`, moves the ship; everything it needs comes in as plain structs, so the same file runs in Bevy, in a test or on a server.

```
id: bevy-six-axis-flight
engine: any; plain Rust, std only. The example uses Bevy 0.20
language: Rust (edition 2024)
version: 0.1
licence: MIT (LICENSE) for src/, tests/ and examples/; this Markdown CC BY-NC 4.0 (LICENSE-CONTENT)
source: rebuilt from the flight model of one of the original studio's game prototypes (a private repo; see PROVENANCE.md)
depends on: nothing (Rust std); the example needs the bevy crate, 0.20
owns: the ship's attitude (two unit vectors, turned by body rates and kept square); body rates with their top speeds and accelerations; thruster spool; Flight Assist on (grip, drag, brake, auto-coordination, auto-level) and off (momentum, slow spin decay, kill_rot, the collective lever); the hover assist and soft touch in a hover field; boost and jinks; the arm-speed thrust fade and speed caps; fuel by speed change and the small-thruster charge; level-out; the impact point on a pad's plane; the attitude as a quaternion
does not own: the game's world, gravity and hover fields (the game passes them in each tick through Ctx); collisions, landing and damage; reading the keyboard or pad (the example does a simple version); the camera; autopilots
used by: none here
copies: none
tested: cargo test in tests/ on 2026-11-10, Rust 1.97.0: 24 passed (see Tests); cargo clippy --all-targets clean; 9 deliberate breakages of the rebuild's own changes each fail a test. examples/: cargo clippy and cargo test (1 passed, the change of axes) with --ignore-rust-version, since Bevy 0.20 asks for Rust 1.97.1; not run in a window
```

## Use it in a game

1. Copy `src/flight6.rs` into the game, for example as `src/flight6.rs` with `mod flight6;`.
2. Keep one `FlyState` and one `Body` per ship. `FlyState::new(heading, &engine, &tuning)` starts level, at rest, with a full charge and Flight Assist on.
3. Each fixed tick, fill a `Controls` from the player's input and call `step6(&mut fly, &mut body, &controls, &ctx)`. `Ctx::open(dt)` is open space with no gravity; set `gravity`, `field` (a `HoverField`) and `gear` for the game's world.
4. Copy `body.pos` and `fly.att.quat()` onto the ship's transform. In the model's own axes the ship's nose is local +x, its deck up +z and its left wing +y. The model is z-up; for a y-up engine such as Bevy, turn both with the change of axes in `examples/fly.rs` (`to_bevy`), and build the ship mesh with its nose along +X and its deck up along +Y. A mesh facing Bevy's usual -Z would fly sideways.
5. Use the returned `FlyOut` for the HUD and sound: `main_frac`, `lift_frac` and `small_frac` for engine glow and volume, `boosting`, `jinking`, `hover_assist` and `grip_on` for indicators, `accel` for camera shake, `fuel_used` and `charge_used` for gauges.

To change the feel, build the context as `Ctx { engine, tuning, ..Ctx::open(dt) }`; `Ctx::open` alone uses the defaults. Turn Flight Assist on and off with `fly.fa`.

For a hover field near a pad, `field_fade(distance, full, zero)` gives a smooth fade; multiply the fade for height by the fade for distance from the pad's axis, and pass the field's full-strength pull as `accel` with that `fade`, and the height above the pad as `h`. The hover assist assumes a level pad with +z up. The prototype used a 5 m/s² pull, full strength up to 450 m above the pad and none from 600 m.

## What it gives you

- **Axes.** World z is up. Body rates are `w = V3 { x: roll, y: yaw, z: pitch }` in rad/s; positive is right wing down, nose right, nose up. The ship's own axes are `f` (nose), `u` (deck up) and `r = f × u` (right wing).
- **Flight Assist on, open space.** Main thrust along the nose, retro as a brake, lift and strafe on the small thrusters. The grip swings the velocity onto the nose (so the ship carves), drag slows it gently, a held bank yaws the ship toward the low wing, and the wings level themselves soon after the roll stick is let go. `no_grip` lets it slide, for example while aiming.
- **Flight Assist on, in a hover field.** The stick sets a speed over the ground (up to `hover_vh`), lift sets a climb or sink, and the ship leans into the push up to `hover_tilt`, holding still hands off. The hover assist is on above a field fade of `hover_fade`, at or under `hover_speed`, or at any speed with the gear down. Main thrust from `punch_min_h` up leaves the hover, with the gear up only. Under `soft_h` with the gear down, the descent is held to `max(soft_min, h / 5)`. With Flight Assist on, the gear down caps the speed at `hover_speed` everywhere, open space included.
- **Flight Assist off.** Every thruster pushes as it is. A spin keeps going when the stick is let go, decaying slowly; `kill_rot` stops it at full authority. In a hover field, the collective lever (`fly.lever`, 0 to `lever_max`) holds the ship against the field at `hover_mark`.
- **Speed.** Thrust fades along each axis as the speed that way nears `arm_speed` and never pushes past it; only the boost and jinks go on toward `speed_cap`.
- **Boost and jink.** Boost alone pushes along the nose. Boost with lift, strafe or (Flight Assist off) retro held doubles that thruster with no fade: a jink.
- **Fuel.** The tank pays per m/s of speed change (`fuel_per_dv`), plus a burn for the boost and for going faster than `arm_speed`. The small thrusters pay from the charge (counted in m/s of speed change), which refills while there is fuel; above `small_tank_speed` their push along the velocity pays the tank. Turning is free. At 0 fuel the ship still turns but cannot thrust.
- **Helpers.** `level_out_step` turns the ship level (roll 0, pitch within `level_max_pitch`) while it coasts, snapping level after `level_snap_s`. `impact_point` says where a ship would meet a pad's plane under a constant pull, for a landing ring. `Attitude::quat` returns `[x, y, z, w]`.

Every number is in `Engine` (thrusters and charge) or `Tuning` (feel). The defaults are one prototype's first guesses; tune them for your game.

## Limits

- One ship per call, no collisions: the game moves the ship back out of anything it hits and changes `body.vel`.
- `Body.vel` is relative to whatever frame the game flies in. Near a moving pad, pass the velocity relative to the pad and carry the pad's own motion yourself.
- It assumes a fixed tick. It works at any `dt`, but the tests and tuning use 1/60 s.
- `f32` throughout. Far from the origin (tens of kilometres) positions get coarse; keep the game's origin near the player.
- The hover assist takes over the main engine as well as the small thrusters, and pays all its thrust from the charge first.

## Tests

`cargo test` in `tests/` (no Bevy, a few seconds). 24 tests, all passing on 2026-11-10 with Rust 1.97.0:

- the attitude stays square and unit length over 100,000 random ticks, with speed, heading and charge always finite and in range;
- the heading is held exactly while the nose is in the vertical cone of a pitch loop, and only flips by the half turn the nose makes; level-out from near vertical comes back down on the heading the ship was flying;
- `quat()` turns the local axes onto the nose, deck and left wing at 500 random attitudes;
- the thrust table (each thruster, and each jink at double) within 1%, and the main spool at 63% after 0.15 s;
- a bang-bang half-turn flip takes about 1.44 s; Flight Assist on stops each axis within 0.25 s; off, a released spin coasts about 1.19 s; `kill_rot` stops it within 12 ticks;
- 60 s of full thrust stays at the arm speed with Flight Assist on or off; boost reaches the top speed; a strafe jink passes the arm speed and a plain strafe doesn't; the gear caps speed;
- stopping from 224 m/s takes 4 s and about 448 m with the brake, and about 7.5 s on the retro thruster;
- fuel: turning is free; 1 s of main costs about 5.4; a sideways pulse at speed is paid from the charge; fast retro pays the tank; a held lift drains the charge to 0 and never below; the charge refills only with fuel; the boost's burn; the burn above the arm speed;
- the hover assist holds within 0.5 m for 60 s hands off; the stick gives the hover ground speed; the lean stops at `hover_tilt`; soft touch holds the descent; main thrust punches out above `punch_min_h` and not below; taking over in a field loses at most 1 m (Flight Assist on) or 3 m (off, lever at the mark) in 2 s;
- the field pulls by `accel × fade`, the lever holds at half strength, the hover starts only above `hover_fade`, and under it Flight Assist on cancels the pull;
- a dry tank turns but doesn't thrust, and no field acts;
- Flight Assist on carves at 112 m/s with the course on the nose, slides with `no_grip`, lets go of the grip for `grip_hold_s` after a strafe, coordinates a bank and levels itself;
- level-out from upside down and steep finishes within 1.5 s with the ship coasting, and snaps level past its time limit;
- the arm fade, over-burn and field-fade helpers, a zero `dt` changing nothing, and the impact point on the pad's plane.

`examples/` has one test of its own: `to_bevy` puts the mesh's +X, +Y and +Z on the nose, deck and right wing at four attitudes. Run it with `cargo test` in `examples/` (a long first Bevy build, with Rust 1.97.1 or `--ignore-rust-version`).
