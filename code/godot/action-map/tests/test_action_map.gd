extends SceneTree
## Headless test for the godot-action-map module.
##
## Run from the repo root, against the Godot phone template (it holds a byte-for-byte
## copy of src/action_map.gd as its ActionMap autoload, and tests/actions.json is the
## same file as its controls/actions.json):
##   godot --headless --path path/to/your/godot/project \
##     -s "$PWD/code/godot/action-map/tests/test_action_map.gd"
## Prints one line per check and quits with 0 when all pass, 1 otherwise.

var _failed := 0
var _started := false


func _process(_delta: float) -> bool:
	if _started:
		return false
	_started = true
	var am: Node = root.get_node_or_null("ActionMap")
	if am == null:
		am = load("res://scripts/action_map.gd").new()
		am.name = "ActionMap"
		root.add_child(am)
	_run(am)
	print("all checks passed" if _failed == 0 else "%d check(s) failed" % _failed)
	quit(1 if _failed else 0)
	return true


func _check(what: String, ok: bool) -> void:
	print(("ok   " if ok else "FAIL ") + what)
	if not ok:
		_failed += 1


func _has_key(action: String, code: Key) -> bool:
	for ev in InputMap.action_get_events(action):
		if ev is InputEventKey and ev.physical_keycode == code:
			return true
	return false


func _has_pad_button(action: String, button: JoyButton) -> bool:
	for ev in InputMap.action_get_events(action):
		if ev is InputEventJoypadButton and ev.button_index == button:
			return true
	return false


func _run(am: Node) -> void:
	_check("read 6 actions", am.actions.size() == 6)
	_check("read 4 touch controls", am.touch_controls.size() == 4)
	_check("reference size 720x1280", am.reference_px == Vector2(720, 1280))
	for d in ["left", "right", "up", "down"]:
		_check("axis_2d move -> move_%s" % d, InputMap.has_action("move_" + d))
	_check("no plain 'move' action", not InputMap.has_action("move"))
	_check("WASD: move_left has A", _has_key("move_left", KEY_A))
	_check("Arrows: move_up has Up", _has_key("move_up", KEY_UP))
	_check("dpad: move_down has D-pad down", _has_pad_button("move_down", JOY_BUTTON_DPAD_DOWN))
	_check("dash has Space", _has_key("dash", KEY_SPACE))
	_check("dash has face_south (A)", _has_pad_button("dash", JOY_BUTTON_A))
	_check("pause has Escape and P", _has_key("pause", KEY_ESCAPE) and _has_key("pause", KEY_P))
	_check("touch_ids(dash) == [btn_dash]", am.touch_ids("dash") == ["btn_dash"])
	_check("action_for_touch(btn_dash, play) == dash", am.action_for_touch("btn_dash", "play") == "dash")
	_check("action_for_touch(btn_dash, menu) == ''", am.action_for_touch("btn_dash", "menu") == "")
	_check("prompt(dash, gamepad) == A", am.prompt("dash", "gamepad") == "A")
	_check("prompt(dash, touch) == Dash", am.prompt("dash", "touch") == "Dash")
	_check("prompt(pause, keyboard_mouse) == Escape", am.prompt("pause", "keyboard_mouse") == "Escape")
	_check("prompt(back, touch) == '' (no touch binding)", am.prompt("back", "touch") == "")
