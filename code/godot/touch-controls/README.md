# Godot touch controls

**What it is:** an on-screen thumb stick, buttons and tap areas for Godot 4 phone and tablet games, drawn from the action map's `touch_layout`. Each control presses the game's own input actions, so the game reads touch exactly like keys or a pad. A small `input_mode.gd` shows the controls on touch and hides them when a pad or keyboard is used.

**Use it for:**

- A phone or browser build of a game that already has keyboard or pad controls, with no new input code in the game.
- Walking with a stick while pressing a button with the other thumb (each finger is tracked).
- Menus on touch: switch `context` to `"menu"` and a tap area or the `tapped` signal picks things (if the menu pauses the tree, set the overlay's `process_mode` to `PROCESS_MODE_ALWAYS`).
- Steam Deck or laptop touchscreens next to a pad: the controls step aside when the pad is used.

**Not for:** gestures (swipe, pinch), a d-pad, or styled button art; deciding where controls go (your design and the action map do that).

**Get started:** set up [godot-action-map](../action-map/README.md) first. Copy `src/touch_controls.gd` and `src/input_mode.gd` to `res://scripts/`, add `input_mode.gd` as an autoload named `InputMode`, then:

```gdscript
var touch := preload("res://scripts/touch_controls.gd").new()
$UI.add_child(touch)   # any CanvasLayer
```

Its tests are written but have not been run in Godot yet.

Details: [MODULE.md](MODULE.md) · Where it came from: [PROVENANCE.md](PROVENANCE.md)
