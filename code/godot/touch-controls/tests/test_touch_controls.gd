extends SceneTree
## Headless test for the godot-touch-controls module. Run it inside any Godot 4 project that holds
## copies of src/touch_controls.gd and src/input_mode.gd in res://scripts/ (MODULE.md shows how):
##   godot --headless --path <project> -s "$PWD/code/godot/touch-controls/tests/test_touch_controls.gd"
## It needs no action map file and no autoloads: FakeMap below stands in for godot-action-map's
## ActionMap with the phone template's layout. Touches go to the overlay's _input(), straight or through
## its viewport's push_input() where a check needs the handled flag. The checks start one frame in,
## because _initialize() runs before the root joins the tree (nodes added then have no _ready yet).
## Prints one line per check and quits with 0 when all pass, 1 otherwise. A script error stops only
## the test function it happens in, so each function returns true at its end and a missing true fails.

const Touch := preload("res://scripts/touch_controls.gd")
const ModeScript := preload("res://scripts/input_mode.gd")


## Stands in for ActionMap: the phone template's touch_layout, bound to test actions.
class FakeMap extends RefCounted:
	var reference_px := Vector2(720, 1280)
	var touch_controls: Array = [
		{"id": "stick_move", "label": "Move", "type": "stick", "x": 0.28, "y": 0.82, "r": 0.09},
		{"id": "btn_dash", "label": "Dash", "type": "button", "x": 0.76, "y": 0.82, "r": 0.06},
		{"id": "btn_pause", "label": "II", "type": "button", "x": 0.88, "y": 0.11, "r": 0.036},
		{"id": "tap_screen", "label": "Tap the screen", "type": "area", "x": 0.5, "y": 0.5, "w": 0.8, "h": 0.8},
	]
	var by_context := {"play": {"stick_move": "tc_move", "btn_dash": "tc_dash"},
		"always": {"btn_pause": "tc_pause"}, "menu": {"tap_screen": "tc_confirm"}}

	func action_for_touch(control_id: String, context: String) -> String:
		return by_context.get(context, {}).get(control_id, "")


## Stands in for InputMode where a test sets the mode itself.
class FakeMode extends RefCounted:
	signal changed(mode: String)
	var mode := "touch"


## The overlay, counting its _draw() calls and noting one that runs to the end while showing.
class DrawProbe extends "res://scripts/touch_controls.gd":
	var drew := false
	var draws := 0

	func _draw() -> void:
		draws += 1
		super()
		drew = drew or is_showing()


var _failed := 0
var _log := []    # what the overlay sent: "+tc_dash", "-tc_dash", "+tc_move_left 0.50"
var _taps := []   # positions passed to tapped
var _probe: DrawProbe
var _frames := 0


func _initialize() -> void:
	_run.call_deferred()


func _run() -> void:
	var tests := {"buttons": _buttons, "stick": _stick, "fingers": _two_fingers,
		"contexts and hiding": _contexts_and_hiding, "pause": _pause, "layout": _layout,
		"input mode": _input_mode, "real Input": _real_input}
	for label in tests:
		var finished = tests[label].call()
		_check(label + " ran to the end (if not, look for SCRIPT ERROR above)", finished is bool and finished)
	paused = false   # in case the pause test stopped half way
	_probe = _start_draw_probe()


## Drawing needs a frame or two, so the draw checks and the result wait for _process. Each step
## changes the probe, and the next one, two frames on, checks that it drew again.
func _process(_delta: float) -> bool:
	if _probe == null:   # _run hasn't started yet
		return false
	_frames += 1
	match _frames:
		3:
			_check("the draw probe holds the stick and the button",
				_probe.is_held("tc_dash") and _probe.is_held("tc_move_right"))
			_check("controls draw without a script error, with a stick held and a button lit", _probe.drew)
			_probe.draws = 0
			_probe.forced = false   # its input mode is gamepad, so the controls hide
			_check("turning forced off outside touch mode lets go of everything",
				not _probe.is_held("tc_dash") and not _probe.is_held("tc_move_right") and _balanced())
		5:
			_check("turning forced off redraws, so the controls go at once", _probe.draws > 0)
			_probe.drew = false
			_probe.forced = true
		7:
			_check("turning forced on draws them again", _probe.drew)
			_probe.draws = 0
			_probe.tint = Color.YELLOW
		9:
			_check("a new tint redraws", _probe.draws > 0)
			_probe.get_parent().free()
			print("all checks passed" if _failed == 0 else "%d check(s) failed" % _failed)
			quit(1 if _failed else 0)
			return true
	return false


func _check(what: String, ok: bool) -> void:
	print(("ok   " if ok else "FAIL ") + what)
	if not ok:
		_failed += 1


## A fresh overlay in its own viewport of the given size, recording what it sends.
func _overlay(map: Object = null, view := Vector2i(720, 1280), mode: Object = null, t: Touch = null) -> Touch:
	var vp := SubViewport.new()
	vp.size = view
	root.add_child(vp)
	if t == null:
		t = Touch.new()
	t.action_map = map if map != null else FakeMap.new()
	t.input_mode = mode if mode != null else FakeMode.new()
	t.vibrate_ms = 0
	t.emit_event = func(e: InputEventAction) -> void:
		if not e.pressed:
			_log.append("-" + e.action)
		elif e.strength > 0.999:   # a full push can round to 0.99999994
			_log.append("+" + e.action)
		else:
			_log.append("+%s %.2f" % [e.action, e.strength])
	t.tapped.connect(func(at: Vector2) -> void: _taps.append(at))
	vp.add_child(t)
	_log.clear()
	_taps.clear()
	return t


func _free(t: Touch) -> void:
	t.get_parent().free()


func _c(t: Touch, id: String) -> Dictionary:
	for c in t.action_map.touch_controls:
		if c["id"] == id:
			return c
	return {}


func _spot(t: Touch, id: String) -> Vector2:
	return t.centre(_c(t, id))


func _touch(t: Touch, at: Vector2, pressed := true, index := 0) -> void:
	var e := InputEventScreenTouch.new()
	e.index = index
	e.position = at
	e.pressed = pressed
	t._input(e)


## A touch sent through the overlay's viewport, the way Godot delivers it. Returns whether it was
## marked handled: push_input() clears the flag first, so it tells about this touch alone.
func _push_touch(t: Touch, at: Vector2, pressed := true, index := 0) -> bool:
	var e := InputEventScreenTouch.new()
	e.index = index
	e.position = at
	e.pressed = pressed
	var vp := t.get_viewport()
	vp.push_input(e, true)
	return vp.is_input_handled()


func _drag(t: Touch, at: Vector2, index := 0) -> void:
	var e := InputEventScreenDrag.new()
	e.index = index
	e.position = at
	t._input(e)


## True when every action the log pressed was let go again.
func _balanced() -> bool:
	var down := {}
	for line in _log:
		down[line.substr(1).get_slice(" ", 0)] = line.begins_with("+")
	return not down.values().has(true)


func _buttons() -> bool:
	var t := _overlay()
	var dash := _spot(t, "btn_dash")
	var handled := _push_touch(t, dash)
	_check("a touch on Dash presses its action", _log == ["+tc_dash"] and t.is_held("tc_dash"))
	_check("and is marked handled, so the game doesn't see it as well", handled)
	_touch(t, dash, false)
	_check("lifting lets go", _log == ["+tc_dash", "-tc_dash"] and not t.is_held("tc_dash"))
	_log.clear()
	var r := t.radius(_c(t, "btn_dash"))
	_touch(t, dash + Vector2(r * 1.2, 0))
	_check("the hit circle is a little wider than the button", _log == ["+tc_dash"])
	_drag(t, dash + Vector2(r * 2.0, 0))
	_check("sliding well off the button lets go", _log == ["+tc_dash", "-tc_dash"])
	_drag(t, dash)
	_check("sliding back on presses again", _log == ["+tc_dash", "-tc_dash", "+tc_dash"])
	_touch(t, dash, false)
	_log.clear()
	_touch(t, _spot(t, "btn_pause"))
	_touch(t, _spot(t, "btn_pause"), false)
	_check("an 'always' control works in play", _log == ["+tc_pause", "-tc_pause"])
	_log.clear()
	handled = _push_touch(t, Vector2(360, 400))
	_touch(t, Vector2(360, 400), false)
	_check("a touch on no control sends nothing", _log.is_empty())
	_check("and is reported as a tap, left unhandled for the game", _taps == [Vector2(360, 400)] and not handled)
	_free(t)
	return true


func _stick() -> bool:
	var t := _overlay()
	var c := _c(t, "stick_move")
	var mid := t.centre(c)
	var r := t.radius(c)
	_touch(t, mid + Vector2(r, 0))
	_check("pushing the stick right presses move_right fully", _log == ["+tc_move_right"])
	_drag(t, mid + Vector2(-r * 0.5, 0))
	_check("half left: move_left at half strength", "+tc_move_left 0.50" in _log and t.is_held("tc_move_left"))
	_check("and move_right lets go", _log.back() == "-tc_move_right" and not t.is_held("tc_move_right"))
	_drag(t, mid + Vector2(0, r * 3.0))
	_check("sliding past the edge keeps steering, at full strength", _log.back() == "+tc_move_down"
		and not t.is_held("tc_move_left"))
	_touch(t, mid, false)
	_check("lifting centres the stick and lets go of everything", not t.is_held("tc_move_down") and _balanced())
	_log.clear()
	_touch(t, mid)
	_check("a touch on the dead centre sends nothing", _log.is_empty())
	var handled := _push_touch(t, mid + Vector2(r, 0), true, 1)
	_check("a second finger on a held stick doesn't move it", _log.is_empty())
	_check("and isn't a tap: it is marked handled", _taps.is_empty() and handled)
	_touch(t, mid + Vector2(r, 0), false, 1)
	_drag(t, mid + Vector2(0, -r))
	_check("the first finger still steers", _log == ["+tc_move_up"])
	_touch(t, mid + Vector2(r, 0), true, 1)
	_touch(t, mid, false)
	_log.clear()
	_touch(t, mid + Vector2(-r, 0), true, 2)
	_check("once the steering finger lifts, a new one takes the stick (a resting finger doesn't hold it)",
		_log == ["+tc_move_left"])
	_touch(t, mid, false, 2)
	_touch(t, mid, false, 1)
	_check("and everything is let go", _balanced() and _taps.is_empty())
	_free(t)
	return true


func _two_fingers() -> bool:
	var t := _overlay()
	var stick := _spot(t, "stick_move")
	var r := t.radius(_c(t, "stick_move"))
	var dash := _spot(t, "btn_dash")
	_touch(t, stick + Vector2(0, -r), true, 0)
	_touch(t, dash, true, 1)
	_check("walk and dash at once", t.is_held("tc_move_up") and t.is_held("tc_dash"))
	_touch(t, dash, true, 2)
	_touch(t, dash, false, 1)
	_check("two thumbs on one button: lifting one keeps it held", t.is_held("tc_dash"))
	_touch(t, dash, false, 2)
	_check("lifting the other lets go", not t.is_held("tc_dash") and t.is_held("tc_move_up"))
	_touch(t, dash, true, 1)
	_touch(t, dash, true, 1)
	_check("a second press on a finger that never lifted lets the first go",
		_log.count("-tc_dash") == 2 and _log.count("+tc_dash") == 3 and t.is_held("tc_dash"))
	t.release_all()
	_check("release_all lets go of everything", not t.is_held("tc_dash") and not t.is_held("tc_move_up") and _balanced())
	var before := _log.size()
	_drag(t, stick, 0)
	_touch(t, dash, false, 1)
	_touch(t, stick, false, 0)
	_check("fingers lifted after release_all send nothing", _log.size() == before)
	_free(t)
	return true


func _contexts_and_hiding() -> bool:
	var t := _overlay()
	var dash := _spot(t, "btn_dash")
	_touch(t, dash)
	t.context = "menu"
	_check("changing context lets go of what was held", _log == ["+tc_dash", "-tc_dash"])
	_touch(t, dash, false)
	_log.clear()
	_touch(t, dash)
	_touch(t, dash, false)
	_check("in menus Dash isn't live: the touch lands on the tap area", _log == ["+tc_confirm", "-tc_confirm"])
	_log.clear()
	_touch(t, Vector2(30, 30))
	_touch(t, Vector2(30, 30), false)
	_check("outside the tap area: a tap, nothing sent", _log.is_empty() and _taps.back() == Vector2(30, 30))
	t.context = "play"
	_touch(t, dash)
	t.visible = false
	_check("hiding the controls lets go", _log.back() == "-tc_dash" and not t.is_showing())
	_touch(t, dash, false)
	_log.clear()
	_taps.clear()
	_touch(t, dash)
	_check("hidden controls take no touches and report no taps", _log.is_empty() and _taps.is_empty())
	t.visible = true
	_touch(t, dash, false)
	_touch(t, dash)
	var vp := t.get_parent()
	vp.remove_child(t)
	_check("leaving the tree lets go", _log.back() == "-tc_dash" and not t.is_held("tc_dash"))
	t.free()
	vp.free()
	return true


## Pausing the tree stops a pausable overlay's _input, so it lets go of what was held, since the
## release would never reach it. With process_mode ALWAYS it keeps working in a paused menu.
func _pause() -> bool:
	var t := _overlay()
	var dash := _spot(t, "btn_dash")
	_touch(t, dash)
	paused = true
	_check("pausing the tree lets go of what was held", _log == ["+tc_dash", "-tc_dash"] and not t.is_held("tc_dash"))
	_push_touch(t, dash, false)
	_push_touch(t, dash)
	_check("a paused overlay takes no touches", _log.size() == 2 and not t.is_held("tc_dash"))
	paused = false
	_push_touch(t, dash, false)
	_free(t)
	t = _overlay()
	t.process_mode = Node.PROCESS_MODE_ALWAYS
	t.context = "menu"
	paused = true
	var mid := Vector2(360, 640)   # on the menu's tap area
	var handled := _push_touch(t, mid)
	_check("with process_mode ALWAYS, touch works in a menu while paused", _log == ["+tc_confirm"] and handled)
	_push_touch(t, mid, false)
	_check("and lets go", _log == ["+tc_confirm", "-tc_confirm"])
	paused = false
	_free(t)
	return true


func _layout() -> bool:
	var t := _overlay()
	var stick := _c(t, "stick_move")
	_check("radius at the reference size: r x reference height", is_equal_approx(t.radius(stick), 0.09 * 1280.0))
	_check("centre from the fractions of the screen", t.centre(stick).is_equal_approx(Vector2(0.28 * 720.0, 0.82 * 1280.0)))
	var safe := t.safe_rect().grow(0.01)
	for c in t.action_map.touch_controls:
		if c["type"] != "area":
			var r := t.radius(c)
			_check("%s sits inside the safe area" % c["id"], safe.encloses(Rect2(t.centre(c) - Vector2(r, r), Vector2(r, r) * 2.0)))
	_free(t)
	t = _overlay(null, Vector2i(1440, 2560))
	_check("a portrait layout on a canvas twice the size: twice the radius", is_equal_approx(t.radius(stick), 0.09 * 1280.0 * 2.0))
	_free(t)
	var wide := FakeMap.new()
	wide.reference_px = Vector2(1280, 800)
	t = _overlay(wide, Vector2i(2560, 1600))
	_check("a landscape layout on a canvas twice the size: twice the radius too", is_equal_approx(t.radius(stick), 0.09 * 800.0 * 2.0))
	_free(t)
	var edge := FakeMap.new()
	edge.touch_controls = [{"id": "btn_dash", "label": "Dash", "type": "button", "x": 1.0, "y": 0.0, "r": 0.06}]
	t = _overlay(edge)
	var corner := _c(t, "btn_dash")
	var cr := t.radius(corner)
	_check("a control placed on the corner is pulled inside the safe area",
		t.safe_rect().grow(0.01).encloses(Rect2(t.centre(corner) - Vector2(cr, cr), Vector2(cr, cr) * 2.0)))
	_free(t)
	return true


## With the real input_mode.gd (not in the tree, so its _ready doesn't look at the pads).
func _input_mode() -> bool:
	var im: ModeScript = ModeScript.new()
	var seen := []
	im.changed.connect(func(m: String) -> void: seen.append(m))
	var key := InputEventKey.new()
	key.pressed = true
	key.physical_keycode = KEY_E
	im._input(key)
	var nudge := InputEventJoypadMotion.new()
	nudge.axis = JOY_AXIS_LEFT_X
	nudge.axis_value = 0.2
	im._input(nudge)
	_check("a key press switches to keyboard_mouse; a small stick nudge doesn't switch", seen == ["keyboard_mouse"])
	var pad := InputEventJoypadButton.new()
	pad.pressed = true
	pad.button_index = JOY_BUTTON_A
	im._input(pad)
	_check("a pad button switches to gamepad", im.mode == "gamepad")
	var t := _overlay(null, Vector2i(720, 1280), im)
	_check("the controls hide while a pad is in use", not t.is_showing())
	var dash := _spot(t, "btn_dash")
	var handled := _push_touch(t, dash)
	_check("the first touch works while they are still hidden", t.is_held("tc_dash"))
	_check("and is left unhandled, so InputMode (an autoload, so its _input runs after the scene's) sees it",
		not handled)
	var touch := InputEventScreenTouch.new()
	touch.pressed = true
	im._input(touch)
	_check("InputMode switches to touch and the controls show", im.mode == "touch" and t.is_showing())
	_check("switching to touch keeps the press", t.is_held("tc_dash"))
	im._input(pad)
	_check("a pad button hides the controls and lets go", im.mode == "gamepad" and not t.is_showing() and not t.is_held("tc_dash"))
	t.forced = true
	_check("forced shows them anyway", t.is_showing())
	_check("mode changes seen in order", seen == ["keyboard_mouse", "gamepad", "touch", "gamepad"])
	_free(t)
	im.free()
	return true


## The default emit_event, Input.parse_input_event(), reaches Input's action state.
func _real_input() -> bool:
	var names := ["tc_dash", "tc_move_left", "tc_move_right", "tc_move_up", "tc_move_down"]
	for a in names:
		InputMap.add_action(a, 0.2)
	var t := _overlay()
	t.emit_event = Input.parse_input_event
	var dash := _spot(t, "btn_dash")
	_touch(t, dash)
	Input.flush_buffered_events()
	_check("Input sees the Dash press", Input.is_action_pressed("tc_dash"))
	_touch(t, dash, false)
	Input.flush_buffered_events()
	_check("and the release", not Input.is_action_pressed("tc_dash"))
	var c := _c(t, "stick_move")
	_touch(t, t.centre(c) + Vector2(t.radius(c) * 0.5, 0))
	Input.flush_buffered_events()
	_check("a half push reaches Input at half strength", is_equal_approx(Input.get_action_strength("tc_move_right"), 0.5))
	_check("and Input.get_vector() points right", Input.get_vector("tc_move_left", "tc_move_right", "tc_move_up", "tc_move_down").x > 0.0)
	_touch(t, t.centre(c), false)
	Input.flush_buffered_events()
	_check("lifting lets go in Input too", Input.get_action_strength("tc_move_right") == 0.0)
	_free(t)
	for a in names:
		InputMap.erase_action(a)
	return true


## A forced overlay with a stick held and a button lit, left for _process to check after it draws.
## Its input mode is gamepad, so it shows only while forced.
func _start_draw_probe() -> DrawProbe:
	var probe := DrawProbe.new()
	probe.forced = true
	var pad := FakeMode.new()
	pad.mode = "gamepad"
	_overlay(null, Vector2i(720, 1280), pad, probe)
	var c := _c(probe, "stick_move")
	_touch(probe, probe.centre(c) + Vector2(probe.radius(c) * 0.5, 0), true, 0)
	_touch(probe, _spot(probe, "btn_dash"), true, 1)
	probe.queue_redraw()
	return probe
