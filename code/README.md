# Code library

Reusable chunks of game code for Bevy and Godot. Copy a module into your game instead of writing your own. Each one came out of a real game, was made general, and was reviewed by a second Claude session before it went in.

| Module | Engine | What it is | Use it for |
| --- | --- | --- | --- |
| [bevy-action-map](bevy/action-map/README.md) | Bevy 0.20, Rust | Reads one `controls/actions.json` into per-frame action state for keyboard, mouse, gamepad and touch, with prompts that follow the device used last | Controls from one file instead of hard-coded keys; button prompts that name the right input |
| [bevy-headless-app](bevy/headless-app/README.md) | Bevy 0.20, Rust | A windowless Bevy app where each `update()` is exactly one fixed tick, with `step`, `run_until` and message recording | Testing game rules in `cargo test` with no window or GPU; bots and self-tests |
| [godot-action-map](godot/action-map/README.md) | Godot 4.7, GDScript | Builds Godot's InputMap from the same `controls/actions.json` and gives prompt text per device | Keys, pad buttons and touch from one file |
| [godot-touch-controls](godot/touch-controls/README.md) | Godot 4.7, GDScript | On-screen sticks, buttons and tap areas drawn from the action map's `touch_layout` | A phone or browser build of a game that already has keyboard or pad controls |
| [godot-safe-save](godot/safe-save/README.md) | Godot 4.7, GDScript | Crash-safe save slots: temp file, read-back check, backup, versioned JSON that keeps floats and vectors exact | Save slots, settings and progress |

The two action-map modules are twins: they read the same file with the same action ids, so a game can move between Bevy and Godot without remapping. Their `tests/actions.json` is an example of the format.

## Layout

Each module is one folder, `code/<engine>/<name>/`:

- `README.md`: what it is, what to use it for, how to start.
- `MODULE.md`: a manifest (id, engine, version, what it depends on, how it was tested), then how to use it and its limits.
- `PROVENANCE.md`: where it came from and how Claude was used.
- `src/`: the code you copy. `tests/` and `examples/` beside it.

## Rules

1. **Code is MIT** ([LICENSE](../LICENSE)); the Markdown is CC BY-NC 4.0 ([LICENSE-CONTENT](../LICENSE-CONTENT)).
2. **Copy, don't link.** A game takes a copy of `src/` and notes the module and version it took.
3. **No secrets, no network at runtime, no paid services** in a module.
4. **Credit what you borrow.** Outside code keeps its own licence line, and `PROVENANCE.md` says where it came from.
5. **Test it.** `MODULE.md` says how its tests were last run and what they showed; "not run" is said plainly.
