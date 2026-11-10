# Headless Bevy test app

**What it is:** a windowless Bevy app for tests: `MinimalPlugins` plus your game's own plugins, on a clock where every `app.update()` runs exactly one `FixedUpdate` tick. Your game rules run in `cargo test` with no window, no GPU and no waiting.

**Use it for:**
- Testing game rules (damage, scoring, a round timer) by stepping ticks and checking resources and components, or letting a bot play a whole match.
- Proving a seeded run repeats: two runs from the same seed end in the same state.
- Waiting for something to happen with a hard limit (`run_until`), so a broken rule fails the test instead of hanging it.

**Not for:** rendering, screenshots or asset-loading checks; those need a window or real time.

**Get started:** copy `src/headless.rs` to your game as `tests/common/mod.rs`, then in a test:

```rust
#[allow(dead_code)] mod common;
use common::*;

let mut app = headless_app(GamePlugin);            // your game's plugin
step(&mut app, 60);                                // one second at 60 Hz
let ticks = run_until(&mut app, 600, |app| app.world().resource::<Score>().0 >= 3);
assert!(ticks.is_some(), "no third goal within 600 ticks");
```

Details: [MODULE.md](MODULE.md) · Where it came from: [PROVENANCE.md](PROVENANCE.md)
