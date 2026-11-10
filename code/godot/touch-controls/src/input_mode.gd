extends Node
## Which device the player is using right now: "touch", "gamepad" or "keyboard_mouse".
##
## Touch is the default on a phone. The mode follows the last real input, so prompts
## switch the moment a pad button is pressed or the screen is touched. A pad that
## connects switches to "gamepad" at once, which hides the on-screen controls; a pad
## that disconnects while in use falls back to touch.

signal changed(mode: String)

const STICK_DEADZONE := 0.35

var mode := "touch"


func _ready() -> void:
	Input.joy_connection_changed.connect(_on_joy_connection_changed)
	if not Input.get_connected_joypads().is_empty():
		_set_mode("gamepad")
	elif not DisplayServer.is_touchscreen_available() and not OS.has_feature("web_android") and not OS.has_feature("web_ios"):
		_set_mode("keyboard_mouse")


func _input(event: InputEvent) -> void:
	if event is InputEventScreenTouch or event is InputEventScreenDrag:
		_set_mode("touch")
	elif event is InputEventJoypadButton and event.pressed:
		_set_mode("gamepad")
	elif event is InputEventJoypadMotion and absf(event.axis_value) > STICK_DEADZONE:
		_set_mode("gamepad")
	elif event is InputEventKey and event.pressed:
		_set_mode("keyboard_mouse")


func _on_joy_connection_changed(_device: int, connected: bool) -> void:
	if connected:
		_set_mode("gamepad")
	elif mode == "gamepad" and Input.get_connected_joypads().is_empty():
		_set_mode("touch")


func _set_mode(m: String) -> void:
	if m != mode:
		mode = m
		changed.emit(mode)
