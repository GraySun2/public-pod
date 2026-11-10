# Bevy action map

**What it is:** reads your game's `controls/actions.json` (the action map's action map) and gives you every action's state each frame (pressed, just pressed, stick and key axes) from keyboard, mouse and gamepad. It also gives prompt text ("Space", "A", "Cross") for whichever device the player used last. It is the Bevy twin of the Godot action map.

**Use it for:**
- Reading input by action ("jump", "move") instead of by key, so changing a binding means editing the map, not the code.
- Button prompts and hints that switch between keyboard, Xbox-style, PlayStation and Nintendo names by themselves.
- Menus that step once per push and repeat while held, and bots or tests that press actions directly (`set_button`).

**Not for:** drawing touch controls or reading fingers (your touch code fills `TouchState`), or letting players remap in the game (not yet).

**Get started:** copy `src/action_map.rs` into your game, add `serde` (with `derive`) and `serde_json` to `Cargo.toml`, then:

```rust
#[allow(dead_code)] mod action_map;
use action_map::{ActionMap, ActionMapPlugin, Actions};

const MAP: &str = include_str!("../controls/actions.json");   // embedded, so web builds load nothing
app.add_plugins(ActionMapPlugin(ActionMap::from_json(MAP).expect("controls/actions.json")));

fn jump(actions: Res<Actions>) { if actions.just_pressed("jump") { /* ... */ } }
```

Details: [MODULE.md](MODULE.md) · Where it came from: [PROVENANCE.md](PROVENANCE.md)
