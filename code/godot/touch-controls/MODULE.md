# Module: Godot touch controls

On-screen touch controls for a Godot 4 game: thumb sticks, buttons and tap areas, drawn from the action map's `touch_layout` (the action map's format, read by [godot-action-map](../action-map/MODULE.md)). Each finger is tracked by its touch index, and each control sends the game's own actions as `InputEventAction`s through `Input.parse_input_event()`, so `Input.get_vector()`, `Input.is_action_pressed()` and `_unhandled_input()` all see a touch the same way they see a key or a pad button. `input_mode.gd` keeps track of the device in use, so the controls show on touch and step aside when a pad or the keyboard is used.

```
id: godot-touch-controls
engine: Godot 4.7
language: GDScript
version: 0.1
licence: MIT (LICENSE) for src/, tests/ and examples/; this Markdown CC BY-NC 4.0 (LICENSE-CONTENT)
source: made general from the original studio's game code (private repos; see PROVENANCE.md)
depends on: godot-action-map (its ActionMap autoload gives the layout and the action each control drives; any object with touch_controls, reference_px and action_for_touch() will do); the action map format, schema_version 1 (touch_layout controls of type stick, button and area)
owns: drawing sticks and buttons with their labels from touch_layout, inside the safe area and scaled to the screen; tracking each finger; sending the bound actions as InputEventActions (a stick as <action>_left, _right, _up and _down, with strength); the tapped signal for a touch on no control; the input mode (touch, gamepad or keyboard_mouse, from the last real input) and showing the controls only on touch; letting go of everything when the controls hide, the context changes or the app loses focus
does not own: the layout, bindings and labels (the action map's controls/actions.json); building the InputMap and prompt text (godot-action-map); sizes in millimetres, reach, edges and phone tests (your own design); what a tap on a menu row does (the game); button art beyond plain circles (art-design)
used by: none here
copies: none
tested: not run (no Godot binary in the container). tests/test_touch_controls.gd is written but unrun on 2026-10-27. Its scenarios were run against a Python model of touch_controls.gd's logic with float32 vectors, and every expectation held (the model isn't kept). After a review on 2026-10-27 (letting go when the tree pauses, redrawing when forced or tint change, a second finger on a held stick, checks of the handled flag), the new checks were run against a Python model of the fixed script and of Godot 4.5's pause, push_input and redraw rules: all pass, and each new check fails on the old behaviour it covers. The template's earlier touch_controls.gd and this input_mode.gd passed the phone template's headless Chromium phone test on 2026-10-25 (see that package's PACKAGE.md); the combined touch_controls.gd has not run anywhere yet. A second review found the test ran its checks before the root joined the tree; they now start one frame in
```

## Use it in a game

1. Set up [godot-action-map](../action-map/MODULE.md): its `ActionMap` autoload and a `controls/actions.json` whose `touch_layout` places the controls and whose actions bind them (`"touch": ["stick_move"]`).
2. Copy `src/touch_controls.gd` and `src/input_mode.gd` to the project, for example `res://scripts/`.
3. Add `input_mode.gd` as an autoload named `InputMode`, after `ActionMap`:

   ```
   [autoload]
   ActionMap="*res://scripts/action_map.gd"
   InputMode="*res://scripts/input_mode.gd"
   ```

4. Add the overlay under a `CanvasLayer`, and switch its context when menus open and close:

   ```gdscript
   var touch := preload("res://scripts/touch_controls.gd").new()
   $UI.add_child(touch)          # $UI is a CanvasLayer
   touch.context = "menu"        # a menu opened: the stick and play buttons go, menu controls come
   touch.context = "play"        # back to the game
   ```

   If the game pauses the tree for its menus (`get_tree().paused = true`), the overlay stops getting input like any other node, and lets go of everything as the tree pauses. For touch in a menu shown while paused, also set `touch.process_mode = Node.PROCESS_MODE_ALWAYS` (or set it on the `CanvasLayer`).

5. Read input the usual Godot way. Nothing in the game needs to know about touch:

   ```gdscript
   var dir := Input.get_vector("move_left", "move_right", "move_up", "move_down")
   if Input.is_action_just_pressed("dash"):
       ...
   ```

6. For desk testing, turn on **Project Settings > Input Devices > Pointing > Emulate Touch From Mouse**, so a mouse click is a touch.

[examples/touch_play.gd](examples/touch_play.gd) is the smallest case. Without an `InputMode` autoload the controls always show; set `visible = false` to hide them.

## What it gives you

| Name | What it is |
| --- | --- |
| `touch_controls.gd` `new()` | The overlay, a full-screen `Control` that ignores the mouse. Add it under a `CanvasLayer` |
| `context` | `"play"` or `"menu"`: which controls are live. Controls in the `always` context work in play. Setting it lets go of everything held |
| `forced` | Show the controls whatever the input mode (a touch-only build, screenshots). Can change at any time: the controls redraw, and turning it off outside touch mode lets go of everything held |
| `action_map`, `input_mode` | Where the layout and the mode come from. Found at `/root/ActionMap` and `/root/InputMode` when left empty; set them before adding the overlay to use something else |
| `emit_event` | `Callable(InputEventAction)` that sends each event; `Input.parse_input_event` by default. Tests swap it to record what fires |
| `vibrate_ms`, `tint` | Buzz on a button press, in ms (0 for none; Android needs the `VIBRATE` permission); colour of the drawn controls (a new tint redraws them) |
| `tapped(pos)` | Signal: a touch that started on no control. The touch is left unhandled, so the game sees it too |
| `is_showing()` | True while the controls are drawn |
| `is_held(action)` | True while a touch holds the action (a stick direction at any strength) |
| `release_all()` | Lets go of everything. Also runs when the controls hide or leave the tree, the context changes, the tree pauses, or the app loses focus |
| `safe_rect()` | The screen's safe area in canvas units, less a 16 px margin; lay the HUD out inside it |
| `centre(c)`, `radius(c)` | Where a `touch_layout` control sits and how big it is, in canvas units |
| `input_mode.gd` `mode`, `changed(mode)` | `"touch"`, `"gamepad"` or `"keyboard_mouse"`, from the last real input, and a signal when it changes. A pad connecting switches to `gamepad`; one disconnecting while in use falls back to `touch` |

How each control type behaves:

- **stick**: takes a touch up to 1.6 radii from its centre and follows that finger until it lifts, even off the stick. It drives `<action>_left`, `_right`, `_up` and `_down` (the four actions godot-action-map makes for an `axis_2d` action), each as strong as the stick leans that way. A second finger on a held stick rests there: it doesn't move the stick, isn't reported as a tap and is marked handled. Once the steering finger lifts, the next new finger takes the stick.
- **button**: takes a touch up to 1.25 radii out, lets go when the finger slides past 1.6 radii and presses again if it slides back. Two fingers on one button keep it held until both lift.
- **area**: a rectangle (`x`, `y`, `w`, `h` as fractions of the screen) that holds its action while a finger that started inside stays down. Not drawn. Sticks and buttons win over an area under them.

## How it fits with the rest

- **[godot-action-map](../action-map/MODULE.md)** builds the InputMap from `controls/actions.json` and answers which action a touch control drives; this module draws those controls and presses those actions. `ActionMap.prompt(action, "touch")` returns the same label the overlay draws.

## Limits

- Plain circles with text labels; no images or per-control colours.
- the action map's three control types only. No d-pad, swipe, pinch or long press, and no repeat while a direction is held (for menus, tap rows with `tapped` or an area).
- Stick directions are pressed at any strength above zero. Read movement with `Input.get_vector()`, whose dead zone applies, rather than `is_action_pressed()` on one direction.
- Only `always` joins another context, and only `play` (the action map's usual set-up); other pairings in the map's `contexts` aren't read.
- Events sent with `Input.parse_input_event()` reach `Input` at its next flush, normally the next frame, not at once.
- The overlay keeps the default `process_mode`, so it takes no touches while the tree is paused, and lets go of everything as it pauses. A pause menu that is used by touch needs `process_mode = Node.PROCESS_MODE_ALWAYS` on the overlay or its `CanvasLayer` (step 4). A finger already down when the tree unpauses does nothing until it lifts and touches again.
- The safe-area conversion has only run in headless Chromium inside the template. Check it on a real phone with a notch. iOS Safari's multitouch bugs apply; a lost release is handled by letting the old finger go when its index is pressed again.

## Tests

`tests/test_touch_controls.gd` checks buttons (press, release, the wider hit circle, sliding off and back), the always context, taps on no control, the stick (full and half strength, sliding past the edge, the dead centre, a second finger resting on it, a new finger taking it after the first lifts), two fingers at once and two on one button, a lost release, `release_all()`, contexts, hiding, leaving the tree, pausing the tree (it lets go; with `PROCESS_MODE_ALWAYS` a paused menu still works), which touches are marked handled (a touch on a control while showing, but not a tap or the first touch while hidden, sent through the viewport's `push_input()`), the layout (radius and centre at the reference size, controls inside the safe area, scaling on a bigger canvas in portrait and landscape, a control on the corner pulled in), the real `input_mode.gd` (key, pad, a small stick nudge, the first touch while hidden, `forced`), the default `Input.parse_input_event()` path into `Input`, that `_draw()` runs to its end, and that changing `forced` or `tint` redraws (turning `forced` off outside touch mode also lets go). It needs no action map and no autoloads; a stand-in map has the phone template's layout. Run it in a scratch project holding the two scripts:

```
mkdir -p /tmp/touch-test/scripts && cp code/godot/touch-controls/src/*.gd /tmp/touch-test/scripts/
printf 'config_version=5\n\n[application]\n\nconfig/name="touch test"\n' > /tmp/touch-test/project.godot
godot --headless --path /tmp/touch-test -s "$PWD/code/godot/touch-controls/tests/test_touch_controls.gd"
```

Each test function has to reach its end, so a script error counts as a failure; still treat any `SCRIPT ERROR` line in the output as one.
