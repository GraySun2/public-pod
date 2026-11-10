# Module: Bevy action map

Turns the action map (`controls/actions.json`) into per-frame action state for a Bevy game: buttons, two-axis and one-axis actions, menu steps and mouse look, from keyboard, mouse, gamepad and on-screen touch controls. It also tracks the device used last, so prompts show the right key or button, in the pad's own style (A B X Y, Cross Circle, or Nintendo's swapped letters). Game code asks about actions by id, never about keys. It is the Bevy twin of [godot-action-map](../../godot/action-map/MODULE.md) and reads the same file. Copy it instead of writing a per-game action enum.

```
id: bevy-action-map
engine: Bevy 0.20 (also passes unchanged on Bevy 0.19.1) with its keyboard, mouse and gamepad features; Rust 1.97.1 or later, Bevy 0.20's minimum; builds in edition 2021 or 2024 crates
language: Rust
version: 0.1
licence: MIT (LICENSE) for src/, tests/ and examples/; this Markdown CC BY-NC 4.0 (LICENSE-CONTENT)
source: made general from the original studio's game code (private repos; see PROVENANCE.md)
depends on: the action map format, schema_version 1 (actions, bindings per device, touch_layout); the bevy crate 0.20 (keyboard, mouse and gamepad features), serde with derive, and serde_json; no other modules
owns: reading the map (from text the game embeds with include_str!); turning each the action map key, mouse and pad name (pairs like A/D and trigger_left/trigger_right included) into Bevy inputs, and listing the ones it can't use in skipped; per-frame Actions (pressed, just pressed, just released, axis_2d, axis_1d, repeat steps and repeating buttons for menus, mouse movement for look actions) from keyboard, mouse, every connected pad and TouchState, timed on real time so menus work while the game is paused; set_button and set_axis for bots; the Device used last, with the pad's button style from its USB vendor; prompt(), fill(), touch_ids() and action_for_touch()
does not own: the action map format, its bindings and prompt wording (control-mapping); drawing on-screen controls and reading fingers (touch-controls; that code fills TouchState); remapping and saving player bindings; turning a toggle_option action on and off (the flag is read, the toggle is the game's); contexts (the game reads the actions its current screen needs); presets
used by: none here
copies: none
tested: cargo +1.97.1 test in tests/ on 2026-10-27, Rust 1.97.1, Bevy 0.20.0: 28 passed (the Godot twin's fixture read with the same lookups, every action map name parsed and labelled, pad pairs, names it can't use skipped with reasons, bad maps refused, and keyboard, mouse, wheel and pad input sent through Bevy's own InputPlugin in a headless app: buttons and their edges, a tap inside one frame from a key and from a pad, a bot holding a button over many frames, WASD and stick axes with the dead zone, the look curve, one-axis key pairs, pad pairs, sticks and triggers and their clamp, two pads, menu repeat timing for axes, buttons and axis_1d, menus and the wheel while Time<Virtual> is paused, prompts following the device, a stick push and the pad vendor, touch state); the same 28 pass on Bevy 0.19.1; src builds as edition 2021; cargo clippy --all-targets clean; examples/player.rs runs; 32 deliberate breakages each fail a test (each of the review fixes undone, an old press-edge bug, the menu delay, the look curve, the Nintendo style, the wheel gesture, the mouse nudge, taps counted as clicks, device change detection, the stronger pad, the stick device switch, a pad tap, the raw menu stick, the d-pad and axis_1d clamps, device priority and more)
```

## Use it in a game

1. Copy `src/action_map.rs` into the game's `src/`, and add `#[allow(dead_code)] mod action_map;` (the allow keeps the build quiet about helpers you don't use).
2. In `Cargo.toml`: `serde = { version = "1", features = ["derive"] }` and `serde_json = "1"`. Bevy's default features already bring in keyboard, mouse and gamepad input and `bevy_gilrs` for real pads (keyboard and mouse come with Bevy's window). With `default-features = false`, make sure `keyboard`, `mouse`, `gamepad` and `bevy_gilrs` are on.
3. Keep the action map at `controls/actions.json` (the action map writes and checks it) and embed it, so web builds need no file loading:

   ```rust
   use action_map::{ActionMap, ActionMapPlugin, Actions, Device};

   const ACTIONS_JSON: &str = include_str!("../controls/actions.json");

   let map = ActionMap::from_json(ACTIONS_JSON).expect("controls/actions.json reads");
   for s in &map.skipped {
       warn!("action map: skipped {s}");
   }
   app.add_plugins((DefaultPlugins, ActionMapPlugin(map)));
   ```

4. Read actions in any system after `PreUpdate`:

   ```rust
   fn player(actions: Res<Actions>, time: Res<Time>, mut q: Query<&mut Transform, With<Player>>) {
       let speed = if actions.pressed("dash") { 900.0 } else { 300.0 };
       for mut t in &mut q {
           t.translation += (actions.axis("move") * speed * time.delta_secs()).extend(0.0);
       }
   }

   fn hint(map: Res<ActionMap>, device: Res<Device>, mut text: Single<&mut Text, With<DashHint>>) {
       if device.is_changed() {
           text.0 = map.fill("{dash} to dash", *device);   // "Space to dash", "A to dash", "Cross to dash"
       }
   }
   ```

[examples/player.rs](examples/player.rs) is the smallest case. Add a test like `assert!(ActionMap::from_json(ACTIONS_JSON).unwrap().skipped.is_empty())` so a binding this can't read fails the build, not the playtest.

## What it gives you

| Name | What it is |
| --- | --- |
| `ActionMap::from_json(text)` | The map, or `Err` saying why it doesn't parse, isn't schema 1, or repeats an action id |
| `map.actions`, `map.get(id)` | Every action in file order (`id`, `label`, `context`, `kind`, `press`, `toggle_option`, bindings per device), and one by id |
| `map.skipped` | Each binding or action left out, with why (`"jump: LeftCtrl+B: chords (two keys at once) aren't supported"`) |
| `map.touch_controls`, `map.reference_px` | `touch_layout.controls` as read, and `reference_px` (1280 x 800 when unset, the format's default) |
| `map.touch_ids(id)`, `map.action_for_touch(control, context)` | The touch controls on an action, and the action a control drives in a context |
| `map.prompt(id, device)` | The label of the action's first usable binding on that device ("Space", "Left click", "A", "Cross", a touch control's label), or `""` when it has none there |
| `map.fill(line, device)` | Each `{action_id}` in a line replaced by its prompt, or its label when it has no binding there |
| `ActionMapPlugin(map)` | Inserts the map, `Actions`, `Device` and `TouchState` and reads input every frame in `ActionMapSystems` (`PreUpdate`, after Bevy's input) |
| `Actions` | `pressed`, `just_pressed`, `just_released` and `button` (all three) for buttons; `axis` (x right, y up, length at most 1) for `axis_2d`; `value` (-1 to 1, a trigger 0 to 1) for `axis_1d`; `step` (one per push, repeating) for `press: "repeat"` axes; `mouse_delta` (pixels) for axes bound to `Mouse`; `set_button` and `set_axis` for bots (call them after `ActionMapSystems` every frame; a held button is just pressed once) |
| `Device` | `KeyboardMouse`, `Gamepad(Glyphs)` or `Touch`; changes only when the device does. `name()` gives the map's device name |
| `Glyphs` | `Letters`, `Shapes` or `Swapped`; `for_vendor(id)` picks Shapes for Sony, Swapped for Nintendo, Letters otherwise |
| `TouchState` | What on-screen controls report: `held`, `just_pressed`, `sticks` by control id, and `active` while a finger is down |
| `parse_key`, `parse_pad`, `key_label`, `pad_label` | One map name as a Bevy binding, and as prompt text (a pair reads "A / D", "LT / RT") |
| `shape_stick`, `walk_stick`, `wheel_step` | The look curve, the walking dead zone, and one-press-per-wheel-gesture, for games that read raw input too |

Names follow the action map's format: letters, digits, `Space`, `Enter`, `Escape`, `Tab`, `Backspace`, `LeftShift` ... `F1` to `F12`, the groups `WASD`, `Arrows` and `IJKL` for `axis_2d`, pairs like `A/D` for `axis_1d`, `Mouse`, `MouseLeft`, `MouseRight`, `MouseMiddle`, `MouseWheel`; pad positions `face_south` ... `start`, `select`, `stick_left`, `stick_right`, `stick_left_x` ..., `dpad` and `dpad_up` ..., and pairs of buttons or triggers like `trigger_left/trigger_right` for `axis_1d`. Bevy's pad buttons are positional too, so `face_south` is the bottom face button on every pad (in Bevy, `LeftTrigger` is the bumper and `LeftTrigger2` the trigger).

How each kind reads: a button is down while any of its bindings is; an `axis_2d` adds its key groups, sticks (dead zone 0.15, then linear; squared for a look, an axis bound to `Mouse`), d-pad and touch sticks, capped at length 1; an `axis_1d` adds held keys (+1), key pairs (-1 or +1), a stick axis, an analog trigger and pad pairs (the second minus the first, so `trigger_left/trigger_right` is right minus left), clamped to -1 to 1. Two pads don't add up: the stronger stick or trigger wins (the d-pads are read per direction). With `press: "repeat"`, an `axis_2d` or `axis_1d` gives a `step` once per push past half way, reading the raw stick, then again after 0.4 s held and every 0.12 s after; a `repeat` button is just pressed again on the same beat. That beat and the wheel's quiet gap run on real time, so menus still repeat when the game pauses or slows `Time<Virtual>`. The device used last is the keyboard or mouse after a key, click, scroll or a mouse move over 4 px (unless a finger is down), touch after a control is touched, and a pad after a button or a stick pushed past half way.

## How it differs from the Godot twin

Same file, same action ids, same `touch_ids`, `action_for_touch` and `prompt(action, device)`. Different where Bevy is: there is no InputMap, so `axis_2d` actions are read with `axis("move")` instead of four `move_left` ... actions; prompts use the action map labels ("Esc", "Left Shift") where the Godot twin shows the map's raw name ("Escape"); pad prompts follow the pad's style instead of letters only; the device used last is tracked here (the Godot template leaves it to its `input_mode.gd`); mouse buttons, the wheel, mouse look and `axis_1d` actions are read; `reference_px` defaults to the format's 1280 x 800, not the phone template's 720 x 1280; and the map is passed in as text instead of read from a path at start-up.

## Limits

- Reads the map once. No remapping at runtime and no saving of player bindings yet; a game can save remaps by action name in its settings file, a pattern to follow when this adds it.
- No chords (`LeftCtrl+B`), no `_hold` pad names, no back grips (Bevy has no button for them) and no pad pairs of sticks (`stick_left_x/stick_right_x`); each is listed in `skipped`.
- `press` other than `repeat` changes nothing: a `hold` action is held, a `tap` or `double_tap` is read with `just_pressed`, and toggles for `toggle_option` actions are left to the game.
- A bot holds a button by calling `set_button(id, true)` after `ActionMapSystems` every frame; the next read lets go of anything it doesn't set again.
- Touch input comes only through `TouchState`; nothing here reads Bevy's `Touches`. A look drag on a touch `area` isn't turned into an axis.
- Tested with Bevy's input messages in a headless app, not with a real pad, a phone or a browser.

## Tests

`tests/` is a small Cargo package that builds `src/action_map.rs` as a library. From `code/bevy/action-map/tests`:

```
cargo test
cargo run --example player
```

Bevy 0.20 needs Rust 1.97.1; if your default toolchain is older, use `cargo +1.97.1 test`. The first build compiles Bevy (about a minute with these features); set `CARGO_TARGET_DIR` to a folder shared with other Bevy modules to build it once.

`tests/actions.json` is the Godot twin's fixture, byte for byte, so both twins are tested on one file; `tests/edge_cases.json` adds mouse, wheel, one-axis, look and toggle actions, pad pairs, a repeat button and a repeat one-axis action, and names this can't use. The headless tests send `KeyboardInput`, `MouseButtonInput`, `MouseMotion`, `MouseWheel` and raw gamepad messages through Bevy's `InputPlugin`, so they check the same path a real device takes after the OS. `python3 -m unittest discover tests/code_library` (no Rust needed) checks this manifest and that both copies of `tests/actions.json` match.
