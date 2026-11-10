# Six-axis flight

**What it is:** a flight model for a ship that pitches, rolls and yaws freely and thrusts along all three of its axes, with Flight Assist. One plain Rust file, `src/flight6.rs`, with no dependencies, so it runs inside Bevy or any other engine.

**Use it for:**
- A space or drone game where the ship flies in every direction: arcade handling with Flight Assist on, pure momentum with it off.
- Hovering over a landing pad: with Flight Assist on in a hover field, the stick sets a speed over the ground and the ship holds still when it is let go.
- Boost, jinks, fuel, a rechargeable charge for the small thrusters, a level-out helper and a landing-ring impact point, all from one tick function.

**Not for:** collisions, physics bodies or networking. It moves one ship's position, velocity and attitude; the game does the rest.

**Get started:** copy `src/flight6.rs` into your game, then once a fixed tick:

```rust
use flight6::*;

let (engine, tuning) = (Engine::default(), Tuning::default());
let mut fly = FlyState::new(0.0, &engine, &tuning);
let mut body = Body { pos: V3::ZERO, vel: V3::ZERO, fuel: 100.0 };

let controls = Controls { main: 1.0, yaw: 0.3, ..Default::default() };
let ctx = Ctx { engine, tuning, ..Ctx::open(1.0 / 60.0) }; // open space, no gravity
let out = step6(&mut fly, &mut body, &controls, &ctx);
// body.pos, body.vel and fly.att.quat() now hold the new state; out says what the engines did.
```

`examples/fly.rs` is a small Bevy game that flies it with the keyboard, including the change from the model's z-up axes to Bevy's y-up.

Details: [MODULE.md](MODULE.md) · Where it came from: [PROVENANCE.md](PROVENANCE.md)
