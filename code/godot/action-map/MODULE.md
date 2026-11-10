# Module: Godot action map

Builds Godot's InputMap from the action map (`controls/actions.json`), so a Godot game's keys, pad buttons and touch controls all come from the one file the action map owns. It also answers "what do I show the player for this action on this device?" for prompts. It is the first module in the [code library](../../README.md), taken out of the Godot phone template.

```
id: godot-action-map
engine: Godot 4.7
language: GDScript
version: 0.1
licence: MIT (LICENSE) for src/, tests/ and examples/; this Markdown CC BY-NC 4.0 (LICENSE-CONTENT)
source: made general from the original studio's game code (private repos; see PROVENANCE.md)
depends on: the action map format, schema_version 1 (actions, bindings per device, touch_layout); no other modules
owns: reading controls/actions.json at start-up; turning each axis_2d action into four InputMap actions (<id>_left, _right, _up, _down) for Input.get_vector(); turning each button action into one InputMap action with its keys, pad buttons and triggers; touch_ids(), action_for_touch() and prompt() lookups
does not own: the action map format, bindings and prompt wording (control-mapping); drawing touch controls, sizes and safe area (touch-controls, and the template's touch_controls.gd); switching prompts when the device changes (the template's input_mode.gd); remapping at runtime
used by: none here
copies: none
tested: not run yet. No Godot binary in the build container on 2026-10-27; tests/test_action_map.gd is written but unrun. The code itself was exported and phone-tested inside the template on 2026-10-25 (see the template's PROVENANCE.md)
```

## Use it in a game

1. Copy `src/action_map.gd` to the game's Godot project, for example `scripts/action_map.gd`.
2. Put the game's action map at `res://controls/actions.json` (the action map writes and checks it). To keep it elsewhere, change `MAP_PATH` at the top of the script.
3. In **Project Settings > Autoload**, add the script with the name `ActionMap`, or add this to `project.godot`:

   ```
   [autoload]
   ActionMap="*res://scripts/action_map.gd"
   ```

4. Read input the usual Godot way. [examples/player.gd](examples/player.gd) is the smallest case:

   ```gdscript
   var dir := Input.get_vector("move_left", "move_right", "move_up", "move_down")
   if Input.is_action_just_pressed("dash"):
       ...
   hint.text = "%s to dash" % ActionMap.prompt("dash", "gamepad")   # "A to dash"
   ```

Don't add the same actions in Project Settings > Input Map as well; the action map is the one source of truth.

## What it gives you

| Name | What it is |
| --- | --- |
| `actions` | The map's actions, as read |
| `touch_controls` | `touch_layout.controls`, as read |
| `reference_px` | `touch_layout.reference_px` as a `Vector2` (default 720 x 1280) |
| `touch_ids(action_id)` | Touch control ids bound to an action, e.g. `"dash"` -> `["btn_dash"]` |
| `action_for_touch(control_id, context)` | The action a touch control drives in a context (`play`, `menu` or `always`), or `""` |
| `prompt(action_id, device)` | Prompt text for an action on `touch`, `gamepad` or `keyboard_mouse`: the touch control's label, the pad's letters (A, B, LB, RT, Menu ...) or the key name; `""` when the action has no binding there |

Pad inputs use the action map's position names (`face_south`, `shoulder_left`, `trigger_right`, `stick_left`, `dpad` ...). Godot's joypad buttons are positional too, so `face_south` is the bottom face button on every pad. Key groups for `axis_2d` actions are `WASD`, `Arrows` and `IJKL`; other key names go through `OS.find_keycode_from_string()`, plus `LeftShift`.

## Limits

- Reads the map once, in `_ready()`. No runtime remapping or saving of player bindings yet.
- Letters glyphs only for pad prompts (the action map's `letters` set); no PlayStation or Nintendo names, no icons.
- Mouse buttons and the right stick as a button aren't mapped. Unknown key or pad names give a warning and are skipped.
- Tested inside the template on a headless Chromium phone, not on a real phone or a real pad.

## Tests

`tests/test_action_map.gd` checks the actions read, the four `move_*` actions, sample keys and pad buttons, and each lookup, against `tests/actions.json`. Run it from the repo root against the template, which holds the same two files:

```
godot --headless --path path/to/your/godot/project \
  -s "$PWD/code/godot/action-map/tests/test_action_map.gd"
```

