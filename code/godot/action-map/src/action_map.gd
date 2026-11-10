extends Node
## Builds Godot's InputMap from the action map's action map (controls/actions.json).
##
## The action map is the one source of truth for bindings. This script only reads it:
## an axis_2d action "move" becomes four InputMap actions (move_left, move_right,
## move_up, move_down) for Input.get_vector(); a button action keeps its own id.
## Touch bindings name controls in the map's touch_layout; TouchControls draws them.

const MAP_PATH := "res://controls/actions.json"

## the action map names pad inputs by position. Godot's joypad buttons are positional
## too (SDL layout), so face_south is JOY_BUTTON_A on every pad.
const PAD_BUTTONS := {
	"face_south": JOY_BUTTON_A, "face_east": JOY_BUTTON_B,
	"face_west": JOY_BUTTON_X, "face_north": JOY_BUTTON_Y,
	"shoulder_left": JOY_BUTTON_LEFT_SHOULDER, "shoulder_right": JOY_BUTTON_RIGHT_SHOULDER,
	"start": JOY_BUTTON_START, "select": JOY_BUTTON_BACK,
	"stick_left_press": JOY_BUTTON_LEFT_STICK, "stick_right_press": JOY_BUTTON_RIGHT_STICK,
	"dpad_up": JOY_BUTTON_DPAD_UP, "dpad_down": JOY_BUTTON_DPAD_DOWN,
	"dpad_left": JOY_BUTTON_DPAD_LEFT, "dpad_right": JOY_BUTTON_DPAD_RIGHT,
}
const PAD_TRIGGERS := {"trigger_left": JOY_AXIS_TRIGGER_LEFT, "trigger_right": JOY_AXIS_TRIGGER_RIGHT}
## Prompt text per pad position, letters style (the action map's "letters" glyph set).
const PAD_LETTERS := {
	"face_south": "A", "face_east": "B", "face_west": "X", "face_north": "Y",
	"shoulder_left": "LB", "shoulder_right": "RB", "trigger_left": "LT", "trigger_right": "RT",
	"start": "Menu", "select": "View", "stick_left": "Left stick", "dpad": "D-pad",
}
## Key groups used by axis_2d actions: [left, right, up, down].
const KEY_GROUPS := {
	"WASD": [KEY_A, KEY_D, KEY_W, KEY_S],
	"Arrows": [KEY_LEFT, KEY_RIGHT, KEY_UP, KEY_DOWN],
	"IJKL": [KEY_J, KEY_L, KEY_I, KEY_K],
}
const DIRS := ["left", "right", "up", "down"]

var actions: Array = []          ## The map's actions, as read.
var touch_controls: Array = []   ## touch_layout.controls, as read.
var reference_px := Vector2(720, 1280)
var _by_id := {}


func _ready() -> void:
	var text := FileAccess.get_file_as_string(MAP_PATH)
	var data = JSON.parse_string(text)
	if typeof(data) != TYPE_DICTIONARY:
		push_error("ActionMap: can't read %s" % MAP_PATH)
		return
	actions = data.get("actions", [])
	var layout: Dictionary = data.get("touch_layout", {})
	touch_controls = layout.get("controls", [])
	var ref: Array = layout.get("reference_px", [720, 1280])
	reference_px = Vector2(ref[0], ref[1])
	for a in actions:
		_by_id[a["id"]] = a
		_add_action(a)


func _add_action(a: Dictionary) -> void:
	var id: String = a["id"]
	var b: Dictionary = a.get("bindings", {})
	if a.get("kind", "button") == "axis_2d":
		for i in DIRS.size():
			var act := "%s_%s" % [id, DIRS[i]]
			InputMap.add_action(act, 0.2)
			for key_name in b.get("keyboard_mouse", []):
				if KEY_GROUPS.has(key_name):
					InputMap.action_add_event(act, _key(KEY_GROUPS[key_name][i]))
			for pad in b.get("gamepad", []):
				for ev in _pad_dir_events(pad, i):
					InputMap.action_add_event(act, ev)
	else:
		InputMap.add_action(id, 0.5)
		for key_name in b.get("keyboard_mouse", []):
			var ev := _key_from_name(key_name)
			if ev:
				InputMap.action_add_event(id, ev)
		for pad in b.get("gamepad", []):
			var ev := _pad_button(pad)
			if ev:
				InputMap.action_add_event(id, ev)


func _key(code: Key) -> InputEventKey:
	var ev := InputEventKey.new()
	ev.physical_keycode = code
	return ev


func _key_from_name(key_name: String) -> InputEventKey:
	if key_name == "LeftShift":
		return _key(KEY_SHIFT)
	var code := OS.find_keycode_from_string(key_name)
	if code == KEY_NONE:
		push_warning("ActionMap: no key named %s" % key_name)
		return null
	return _key(code)


func _pad_button(pad: String) -> InputEvent:
	if PAD_BUTTONS.has(pad):
		var ev := InputEventJoypadButton.new()
		ev.button_index = PAD_BUTTONS[pad]
		return ev
	if PAD_TRIGGERS.has(pad):
		var ev := InputEventJoypadMotion.new()
		ev.axis = PAD_TRIGGERS[pad]
		ev.axis_value = 1.0
		return ev
	push_warning("ActionMap: no pad input named %s" % pad)
	return null


func _pad_dir_events(pad: String, dir: int) -> Array:
	if pad == "stick_left" or pad == "stick_right":
		var ev := InputEventJoypadMotion.new()
		var left := pad == "stick_left"
		ev.axis = (JOY_AXIS_LEFT_X if left else JOY_AXIS_RIGHT_X) if dir < 2 else (JOY_AXIS_LEFT_Y if left else JOY_AXIS_RIGHT_Y)
		ev.axis_value = -1.0 if dir % 2 == 0 else 1.0
		return [ev]
	if pad == "dpad":
		var ev := InputEventJoypadButton.new()
		ev.button_index = [JOY_BUTTON_DPAD_LEFT, JOY_BUTTON_DPAD_RIGHT, JOY_BUTTON_DPAD_UP, JOY_BUTTON_DPAD_DOWN][dir]
		return [ev]
	return []


## The touch control ids bound to an action, e.g. "dash" -> ["btn_dash"].
func touch_ids(action_id: String) -> Array:
	return _by_id.get(action_id, {}).get("bindings", {}).get("touch", [])


## The action a touch control drives in a context ("play", "menu" or "always").
func action_for_touch(control_id: String, context: String) -> String:
	for a in actions:
		if a.get("context") == context and control_id in a.get("bindings", {}).get("touch", []):
			return a["id"]
	return ""


## Prompt text for an action on a device: "touch", "gamepad" or "keyboard_mouse".
func prompt(action_id: String, device: String) -> String:
	var names: Array = _by_id.get(action_id, {}).get("bindings", {}).get(device, [])
	if names.is_empty():
		return ""
	var first: String = names[0]
	match device:
		"gamepad":
			return PAD_LETTERS.get(first, first)
		"touch":
			for c in touch_controls:
				if c["id"] == first:
					return c.get("label", first)
			return first
		_:
			return first
