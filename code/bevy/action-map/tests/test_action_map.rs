//! Tests for code/bevy/action-map: read the Godot twin's fixture and edge cases, then drive
//! keyboard, mouse and pad input through Bevy's own InputPlugin in a headless app.
use std::time::Duration;

use action_map::*;
use bevy::input::gamepad::{
    GamepadConnection, GamepadConnectionEvent, RawGamepadAxisChangedEvent,
    RawGamepadButtonChangedEvent, RawGamepadEvent,
};
use bevy::input::keyboard::{Key, KeyboardInput, NativeKey};
use bevy::input::mouse::{MouseButtonInput, MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::input::touch::TouchPhase;
use bevy::input::{ButtonState as Press, InputPlugin};
use bevy::prelude::*;
use bevy::time::TimeUpdateStrategy;

/// The Godot twin's fixture: a byte-for-byte copy of code/godot/action-map/tests/actions.json.
const GODOT_MAP: &str = include_str!("actions.json");
/// Mouse, wheel, one-axis, look and toggle actions, and names this can't use.
const EDGE_MAP: &str = include_str!("edge_cases.json");

const LETTERS: Device = Device::Gamepad(Glyphs::Letters);
const SONY: Option<u16> = Some(0x054c);
const NINTENDO: Option<u16> = Some(0x057e);

// ------------------------------------------------------------ reading the map

#[test]
fn the_godot_fixture_reads_as_in_the_godot_twin() {
    let map = ActionMap::from_json(GODOT_MAP).unwrap();
    assert!(map.skipped.is_empty(), "{:?}", map.skipped);
    assert_eq!(map.actions.len(), 6);
    assert_eq!(map.touch_controls.len(), 4);
    assert_eq!(map.reference_px, Vec2::new(720.0, 1280.0));

    let mv = map.get("move").unwrap();
    assert_eq!(mv.kind, Kind::Axis2d);
    use KeyCode::*;
    assert_eq!(
        mv.keyboard_mouse[0],
        ("WASD".into(), Binding::Keys4([KeyW, KeyS, KeyA, KeyD]))
    );
    let arrows = Binding::Keys4([ArrowUp, ArrowDown, ArrowLeft, ArrowRight]);
    assert_eq!(mv.keyboard_mouse[1], ("Arrows".into(), arrows));
    assert_eq!(
        mv.gamepad,
        [
            ("stick_left".into(), Binding::StickLeft),
            ("dpad".into(), Binding::Dpad)
        ]
    );
    let dash = map.get("dash").unwrap();
    assert_eq!(dash.keyboard_mouse[0].1, Binding::Key(Space));
    assert_eq!(dash.gamepad[0].1, Binding::Pad(GamepadButton::South));
    let pause: Vec<_> = map
        .get("pause")
        .unwrap()
        .keyboard_mouse
        .iter()
        .map(|b| b.1)
        .collect();
    assert_eq!(pause, [Binding::Key(Escape), Binding::Key(KeyP)]);
    assert_eq!(map.get("menu_move").unwrap().press, "repeat");

    assert_eq!(map.touch_ids("dash"), ["btn_dash"]);
    assert_eq!(map.action_for_touch("btn_dash", "play"), Some("dash"));
    assert_eq!(map.action_for_touch("btn_dash", "menu"), None);
    assert_eq!(map.prompt("dash", LETTERS), "A");
    assert_eq!(map.prompt("dash", Device::Touch), "Dash");
    // The Godot twin shows the map's raw name ("Escape"); this one the action map's label.
    assert_eq!(map.prompt("pause", Device::KeyboardMouse), "Esc");
    assert_eq!(map.prompt("back", Device::Touch), "", "no touch binding");
    assert_eq!(map.prompt("nope", LETTERS), "");
}

#[test]
fn names_it_cant_use_are_skipped_with_a_reason() {
    let map = ActionMap::from_json(EDGE_MAP).unwrap();
    assert_eq!(
        map.skipped,
        [
            "jump: Hyper: unknown key name",
            "jump: LeftCtrl+B: chords (two keys at once) aren't supported",
            "jump: WASD: can't drive a button action",
            "jump: grip_left: Bevy has no button for back grips",
            "jump: face_middle: unknown controller input",
            "jump: stick_left: can't drive a button action",
            "jump: btn_missing: not in touch_layout",
            "spin: kind axis_3d isn't button, axis_1d or axis_2d",
            "slide: stick_left_x/stick_right_x: a controller pair needs two buttons or triggers",
        ]
    );
    let jump = map.get("jump").unwrap();
    assert_eq!(
        jump.keyboard_mouse,
        [("Space".into(), Binding::Key(KeyCode::Space))]
    );
    assert_eq!(
        jump.gamepad,
        [("face_south".into(), Binding::Pad(GamepadButton::South))]
    );
    assert_eq!(jump.touch, ["btn_jump"]);
    assert!(map.get("spin").is_none());
    assert!(map.get("sprint").unwrap().toggle_option);
    assert_eq!(
        map.prompt("jump", Device::Touch),
        "btn_jump",
        "a control with no label shows its id"
    );
    assert_eq!(
        map.reference_px,
        Vec2::new(1280.0, 800.0),
        "the format's default"
    );
}

#[test]
fn bad_maps_are_refused() {
    let err = ActionMap::from_json("{\"actions\": [}").unwrap_err();
    assert!(
        err.contains("doesn't parse") && err.contains("line 1"),
        "{err}"
    );
    let err = ActionMap::from_json(r#"{"schema_version": 2, "actions": []}"#).unwrap_err();
    assert!(err.contains("schema_version 2"), "{err}");
    let twice = r#"{"actions": [{"id": "a", "kind": "button"}, {"id": "a", "kind": "button"}]}"#;
    assert!(
        ActionMap::from_json(twice)
            .unwrap_err()
            .contains("action a twice")
    );
    assert!(
        ActionMap::from_json(r#"{"schema_version": 1}"#).is_err(),
        "no actions"
    );
    // The smallest map: no schema_version, devices, contexts or touch_layout, no kind.
    let map = ActionMap::from_json(
        r#"{"actions": [{"id": "go", "bindings": {"keyboard_mouse": ["G"]}}]}"#,
    )
    .unwrap();
    assert_eq!(map.get("go").unwrap().kind, Kind::Button);
    assert_eq!(map.prompt("go", Device::KeyboardMouse), "G");
}

#[test]
fn every_control_mapping_name_parses_and_has_a_label() {
    let mut keys: Vec<String> = ('A'..='Z').chain('0'..='9').map(String::from).collect();
    keys.extend((1..=12).map(|n| format!("F{n}")));
    for named in [
        "Space",
        "Enter",
        "Escape",
        "Tab",
        "Backspace",
        "LeftShift",
        "RightShift",
        "LeftCtrl",
        "RightCtrl",
        "LeftAlt",
        "RightAlt",
        "CapsLock",
        "ArrowUp",
        "ArrowDown",
        "ArrowLeft",
        "ArrowRight",
        "Comma",
        "Period",
        "Slash",
        "Backslash",
        "Semicolon",
        "Quote",
        "Backquote",
        "Minus",
        "Equal",
        "BracketLeft",
        "BracketRight",
        "WASD",
        "Arrows",
        "IJKL",
        "Mouse",
        "MouseLeft",
        "MouseRight",
        "MouseMiddle",
        "MouseWheel",
        "A/D",
        "ArrowLeft/ArrowRight",
    ] {
        keys.push(named.into());
    }
    for k in &keys {
        assert!(parse_key(k).is_ok(), "{k}");
        assert!(!key_label(k).is_empty(), "{k}");
    }
    for bad in ["F0", "F13", "Hyper", "/", "A/Hyper", "Ctrl+C", ""] {
        assert!(parse_key(bad).is_err(), "{bad}");
    }
    assert_eq!(parse_key("e"), Ok(Binding::Key(KeyCode::KeyE)));
    assert_eq!(parse_key("F12"), Ok(Binding::Key(KeyCode::F12)));
    assert_eq!(parse_key("Slash"), Ok(Binding::Key(KeyCode::Slash)));
    assert_eq!(key_label("e"), "E");
    assert_eq!(key_label("LeftShift"), "Left Shift");
    assert_eq!(key_label("ArrowLeft/ArrowRight"), "Left / Right");
    assert_eq!(key_label("Slash"), "/");
    assert_eq!(key_label("MouseLeft"), "Left click");

    let pads = [
        "face_south",
        "face_east",
        "face_west",
        "face_north",
        "shoulder_left",
        "shoulder_right",
        "trigger_left",
        "trigger_right",
        "stick_left",
        "stick_left_x",
        "stick_left_y",
        "stick_left_press",
        "stick_right",
        "stick_right_x",
        "stick_right_y",
        "stick_right_press",
        "dpad",
        "dpad_up",
        "dpad_down",
        "dpad_left",
        "dpad_right",
        "start",
        "select",
    ];
    for p in pads {
        assert!(parse_pad(p).is_ok(), "{p}");
        for g in [Glyphs::Letters, Glyphs::Shapes, Glyphs::Swapped] {
            assert!(pad_label(p, g).is_some(), "{p} {g:?}");
        }
    }
    assert_eq!(
        pad_label("face_south", Glyphs::Shapes).as_deref(),
        Some("Cross")
    );
    assert_eq!(
        pad_label("face_south", Glyphs::Swapped).as_deref(),
        Some("B")
    );
    assert_eq!(
        pad_label("trigger_right", Glyphs::Letters).as_deref(),
        Some("RT")
    );
    // Pairs drive one axis: two buttons or triggers, negative first.
    use GamepadButton::*;
    let pair = "trigger_left/trigger_right";
    assert_eq!(
        parse_pad(pair),
        Ok(Binding::PadPair(LeftTrigger2, RightTrigger2))
    );
    assert_eq!(
        parse_pad("dpad_left/dpad_right"),
        Ok(Binding::PadPair(DPadLeft, DPadRight))
    );
    assert_eq!(pad_label(pair, Glyphs::Letters).as_deref(), Some("LT / RT"));
    assert_eq!(pad_label(pair, Glyphs::Shapes).as_deref(), Some("L2 / R2"));
    assert_eq!(pad_label("trigger_left/nope", Glyphs::Letters), None);
    for bad in [
        "stick_left/stick_right",
        "trigger_left/nope",
        "grip_left/grip_right",
        "/",
    ] {
        assert!(parse_pad(bad).is_err(), "{bad}");
    }
    assert_eq!(
        parse_pad("trigger_left"),
        Ok(Binding::Pad(GamepadButton::LeftTrigger2))
    );
    assert_eq!(
        parse_pad("shoulder_left"),
        Ok(Binding::Pad(GamepadButton::LeftTrigger))
    );
}

#[test]
fn fill_puts_prompts_into_a_line() {
    let map = ActionMap::from_json(GODOT_MAP).unwrap();
    assert_eq!(
        map.fill("{dash} to dash, {nope}", LETTERS),
        "A to dash, {nope}"
    );
    assert_eq!(
        map.fill("{dash} to dash", Device::Gamepad(Glyphs::Shapes)),
        "Cross to dash"
    );
    assert_eq!(
        map.fill("{pause} pauses", Device::KeyboardMouse),
        "Esc pauses"
    );
    assert_eq!(
        map.fill("{back} to leave", Device::Touch),
        "Back to leave",
        "the label when unbound"
    );
    assert_eq!(map.fill("a { b } {", LETTERS), "a { b } {");
    assert_eq!(map.fill("{{dash}}", LETTERS), "{A}");
}

#[test]
fn pad_style_comes_from_the_vendor() {
    assert_eq!(Glyphs::for_vendor(SONY), Glyphs::Shapes);
    assert_eq!(Glyphs::for_vendor(NINTENDO), Glyphs::Swapped);
    assert_eq!(Glyphs::for_vendor(Some(0x045e)), Glyphs::Letters);
    assert_eq!(Glyphs::for_vendor(None), Glyphs::Letters);
    assert_eq!(Device::Touch.name(), "touch");
    assert_eq!(LETTERS.name(), "gamepad");
}

#[test]
fn a_wheel_gesture_is_one_press() {
    let (mut quiet, dt) = (0.0, 1.0 / 60.0);
    // A trackpad flick: scroll on most frames for half a second, with gaps.
    let presses = (0..30)
        .filter(|i| wheel_step(&mut quiet, if i % 3 == 2 { 0.0 } else { -12.0 }, dt).is_some());
    assert_eq!(presses.count(), 1);
    // Still for a moment, then a notch up: a new press, and it says up.
    for _ in 0..20 {
        wheel_step(&mut quiet, 0.0, dt);
    }
    assert_eq!(wheel_step(&mut quiet, 1.0, dt), Some(true));
}

#[test]
fn sticks_have_a_dead_zone_and_a_curve() {
    assert_eq!(shape_stick(Vec2::new(0.1, 0.05)), Vec2::ZERO);
    assert!((shape_stick(Vec2::X).x - 1.0).abs() < 1e-5);
    let half = shape_stick(Vec2::new(0.575, 0.0)).x;
    assert!(half > 0.2 && half < 0.3, "{half}");
    assert!((walk_stick(Vec2::new(0.0, 0.575)).y - 0.5).abs() < 1e-3);
    assert_eq!(
        walk_stick(Vec2::new(3.0, 4.0)).length(),
        1.0,
        "past full tilt is full tilt"
    );
}

#[test]
fn set_button_gives_edges_for_bots() {
    // With nothing reading (no ActionMapPlugin), each call counts as a frame.
    let mut a = Actions::default();
    a.set_button("dash", true);
    assert!(a.pressed("dash") && a.just_pressed("dash"));
    a.set_button("dash", true);
    assert!(a.pressed("dash") && !a.just_pressed("dash"));
    a.set_button("dash", false);
    assert!(!a.pressed("dash") && a.just_released("dash"));
    a.set_axis("move", Vec2::X);
    assert_eq!(a.axis("move"), Vec2::X);
    assert!(!a.pressed("never"));
}

// ------------------------------------------------------------ headless input

fn app(map: &str) -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, InputPlugin))
        .add_plugins(ActionMapPlugin(ActionMap::from_json(map).unwrap()))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(
            1.0 / 60.0,
        )));
    app.update();
    app
}

fn press(down: bool) -> Press {
    if down {
        Press::Pressed
    } else {
        Press::Released
    }
}

fn key(app: &mut App, key_code: KeyCode, down: bool) {
    app.world_mut().write_message(KeyboardInput {
        key_code,
        logical_key: Key::Unidentified(NativeKey::Unidentified),
        state: press(down),
        text: None,
        repeat: false,
        window: Entity::PLACEHOLDER,
    });
}

fn click(app: &mut App, button: MouseButton, down: bool) {
    let window = Entity::PLACEHOLDER;
    app.world_mut().write_message(MouseButtonInput {
        button,
        state: press(down),
        window,
    });
}

fn nudge(app: &mut App, delta: Vec2) {
    app.world_mut().write_message(MouseMotion { delta });
}

fn scroll(app: &mut App, y: f32) {
    let (unit, window, phase) = (
        MouseScrollUnit::Pixel,
        Entity::PLACEHOLDER,
        TouchPhase::Moved,
    );
    app.world_mut().write_message(MouseWheel {
        unit,
        x: 0.0,
        y,
        window,
        phase,
    });
}

/// Connect a pad as Bevy's gilrs backend would: an entity, then a connection message.
fn connect_pad(app: &mut App, vendor_id: Option<u16>) -> Entity {
    let pad = app.world_mut().spawn_empty().id();
    let connection = GamepadConnection::Connected {
        name: "Test pad".into(),
        vendor_id,
        product_id: None,
    };
    app.world_mut()
        .write_message(GamepadConnectionEvent::new(pad, connection));
    app.update();
    pad
}

fn pad_button(app: &mut App, pad: Entity, button: GamepadButton, value: f32) {
    let raw = RawGamepadButtonChangedEvent::new(pad, button, value);
    app.world_mut().write_message(RawGamepadEvent::Button(raw));
}

fn pad_axis(app: &mut App, pad: Entity, axis: GamepadAxis, value: f32) {
    let raw = RawGamepadAxisChangedEvent::new(pad, axis, value);
    app.world_mut().write_message(RawGamepadEvent::Axis(raw));
}

fn actions(app: &App) -> &Actions {
    app.world().resource::<Actions>()
}

fn device(app: &App) -> Device {
    *app.world().resource::<Device>()
}

fn close(a: Vec2, b: Vec2) -> bool {
    (a - b).length() < 1e-4
}

#[test]
fn keys_drive_buttons_and_axes() {
    let mut app = app(GODOT_MAP);
    key(&mut app, KeyCode::Space, true);
    app.update();
    assert_eq!(
        actions(&app).button("dash"),
        ButtonState {
            pressed: true,
            just_pressed: true,
            just_released: false
        }
    );
    app.update();
    assert_eq!(
        actions(&app).button("dash"),
        ButtonState {
            pressed: true,
            just_pressed: false,
            just_released: false
        }
    );
    assert!(
        actions(&app).pressed("confirm"),
        "Space is on confirm too, in the menu context"
    );
    key(&mut app, KeyCode::Space, false);
    app.update();
    assert_eq!(
        actions(&app).button("dash"),
        ButtonState {
            pressed: false,
            just_pressed: false,
            just_released: true
        }
    );

    key(&mut app, KeyCode::KeyW, true);
    key(&mut app, KeyCode::KeyD, true);
    app.update();
    assert!(
        close(actions(&app).axis("move"), Vec2::new(1.0, 1.0).normalize()),
        "{}",
        actions(&app).axis("move")
    );
    key(&mut app, KeyCode::KeyD, false);
    key(&mut app, KeyCode::ArrowUp, true);
    app.update();
    assert_eq!(
        actions(&app).axis("move"),
        Vec2::Y,
        "two groups the same way still make full tilt"
    );
}

#[test]
fn a_tap_inside_one_frame_still_counts() {
    let mut app = app(GODOT_MAP);
    key(&mut app, KeyCode::Space, true);
    key(&mut app, KeyCode::Space, false);
    app.update();
    assert_eq!(
        actions(&app).button("dash"),
        ButtonState {
            pressed: false,
            just_pressed: true,
            just_released: true
        }
    );
    app.update();
    assert_eq!(actions(&app).button("dash"), ButtonState::default());
}

#[test]
fn pads_drive_actions_and_prompts_follow_the_pad() {
    let mut app = app(GODOT_MAP);
    let map = app.world().resource::<ActionMap>().clone();
    assert_eq!(device(&app), Device::KeyboardMouse);
    let pad = connect_pad(&mut app, SONY);
    assert_eq!(
        device(&app),
        Device::KeyboardMouse,
        "connecting isn't using"
    );

    pad_button(&mut app, pad, GamepadButton::South, 1.0);
    app.update();
    assert!(actions(&app).just_pressed("dash"));
    assert_eq!(device(&app), Device::Gamepad(Glyphs::Shapes));
    assert_eq!(map.prompt("dash", device(&app)), "Cross");
    pad_button(&mut app, pad, GamepadButton::South, 0.0);

    pad_axis(&mut app, pad, GamepadAxis::LeftStickX, 0.575);
    app.update();
    assert!(
        close(actions(&app).axis("move"), Vec2::new(0.5, 0.0)),
        "{}",
        actions(&app).axis("move")
    );
    pad_axis(&mut app, pad, GamepadAxis::LeftStickX, 0.1);
    app.update();
    assert_eq!(
        actions(&app).axis("move"),
        Vec2::ZERO,
        "inside the dead zone"
    );
    pad_button(&mut app, pad, GamepadButton::DPadUp, 1.0);
    app.update();
    assert_eq!(actions(&app).axis("move"), Vec2::Y, "the d-pad moves too");

    key(&mut app, KeyCode::KeyP, true);
    app.update();
    assert_eq!(device(&app), Device::KeyboardMouse);
    assert_eq!(map.prompt("dash", device(&app)), "Space");

    let switch = connect_pad(&mut app, NINTENDO);
    pad_button(&mut app, switch, GamepadButton::East, 1.0);
    app.update();
    assert_eq!(device(&app), Device::Gamepad(Glyphs::Swapped));
    assert_eq!(
        map.prompt("dash", device(&app)),
        "B",
        "confirm on the right"
    );
    assert!(actions(&app).pressed("back"));
}

#[test]
fn device_changes_only_when_the_device_does() {
    #[derive(Resource, Default)]
    struct Changes(u32);
    let mut app = app(GODOT_MAP);
    app.init_resource::<Changes>()
        .add_systems(Update, |d: Res<Device>, mut c: ResMut<Changes>| {
            c.0 += d.is_changed() as u32;
        });
    let pad = connect_pad(&mut app, None);
    let changes = |app: &App| app.world().resource::<Changes>().0;
    let first = changes(&app); // the first run sees the resource as new
    key(&mut app, KeyCode::KeyP, true);
    app.update();
    assert_eq!(changes(&app), first, "already keyboard");
    pad_button(&mut app, pad, GamepadButton::South, 1.0);
    app.update();
    pad_button(&mut app, pad, GamepadButton::North, 1.0);
    app.update();
    assert_eq!(changes(&app), first + 1);
    key(&mut app, KeyCode::KeyP, false);
    key(&mut app, KeyCode::Escape, true);
    app.update();
    assert_eq!(changes(&app), first + 2);
}

#[test]
fn menu_steps_repeat_while_held() {
    let mut app = app(GODOT_MAP);
    key(&mut app, KeyCode::ArrowDown, true);
    let mut at = Vec::new();
    for frame in 0..60 {
        app.update();
        let step = actions(&app).step("menu_move");
        if step != IVec2::ZERO {
            assert_eq!(step, IVec2::new(0, -1));
            at.push(frame);
        }
    }
    assert_eq!(at[0], 0, "a step on the push");
    assert!((24..=25).contains(&at[1]), "again after 0.4 s: {at:?}");
    assert!(
        at.windows(2)
            .skip(1)
            .all(|w| (7..=8).contains(&(w[1] - w[0]))),
        "then every 0.12 s: {at:?}"
    );
    key(&mut app, KeyCode::ArrowDown, false);
    key(&mut app, KeyCode::ArrowRight, true);
    app.update();
    assert_eq!(
        actions(&app).step("menu_move"),
        IVec2::new(1, 0),
        "a new direction steps at once"
    );
}

#[test]
fn mouse_buttons_motion_and_wheel() {
    let mut app = app(EDGE_MAP);
    let pad = connect_pad(&mut app, None);
    pad_button(&mut app, pad, GamepadButton::RightTrigger2, 1.0);
    app.update();
    assert!(actions(&app).pressed("fire"), "the trigger fires");
    assert_eq!(device(&app), LETTERS);
    pad_button(&mut app, pad, GamepadButton::RightTrigger2, 0.0);

    nudge(&mut app, Vec2::new(2.0, 1.0));
    app.update();
    assert_eq!(
        device(&app),
        LETTERS,
        "a nudge of the desk doesn't switch prompts"
    );
    assert_eq!(actions(&app).mouse_delta("look"), Vec2::new(2.0, 1.0));
    assert_eq!(
        actions(&app).mouse_delta("move"),
        Vec2::ZERO,
        "move has no Mouse binding"
    );
    nudge(&mut app, Vec2::new(10.0, -3.0));
    app.update();
    assert_eq!(device(&app), Device::KeyboardMouse);
    assert_eq!(actions(&app).mouse_delta("look"), Vec2::new(10.0, -3.0));

    click(&mut app, MouseButton::Left, true);
    app.update();
    assert!(actions(&app).just_pressed("fire"));
    click(&mut app, MouseButton::Left, false);
    app.update();
    assert!(actions(&app).just_released("fire"));

    let mut presses = 0;
    for i in 0..30 {
        if i % 3 != 2 {
            scroll(&mut app, -12.0);
        }
        app.update();
        presses += actions(&app).just_pressed("next_tool") as u32;
    }
    assert_eq!(presses, 1, "one gesture, one press");
    assert!(!actions(&app).pressed("next_tool"));
}

#[test]
fn look_sticks_curve_and_walk_sticks_do_not() {
    let mut app = app(EDGE_MAP);
    let pad = connect_pad(&mut app, None);
    pad_axis(&mut app, pad, GamepadAxis::RightStickX, 0.575);
    pad_axis(&mut app, pad, GamepadAxis::LeftStickY, 0.575);
    app.update();
    let look = actions(&app).axis("look").x;
    assert!(look > 0.2 && look < 0.3, "squared for fine aim: {look}");
    assert!(
        close(actions(&app).axis("move"), Vec2::new(0.0, 0.5)),
        "{}",
        actions(&app).axis("move")
    );
}

#[test]
fn one_axis_actions_read_pairs_sticks_and_triggers() {
    let mut app = app(EDGE_MAP);
    key(&mut app, KeyCode::KeyD, true);
    app.update();
    assert_eq!(actions(&app).value("steer"), 1.0);
    key(&mut app, KeyCode::KeyA, true);
    app.update();
    assert_eq!(actions(&app).value("steer"), 0.0, "both ways at once");
    key(&mut app, KeyCode::KeyA, false);
    key(&mut app, KeyCode::KeyD, false);
    key(&mut app, KeyCode::ArrowLeft, true);
    key(&mut app, KeyCode::KeyW, true);
    app.update();
    assert_eq!(
        (
            actions(&app).value("steer"),
            actions(&app).value("throttle")
        ),
        (-1.0, 1.0)
    );
    key(&mut app, KeyCode::ArrowLeft, false);
    key(&mut app, KeyCode::KeyW, false);

    let pad = connect_pad(&mut app, None);
    pad_axis(&mut app, pad, GamepadAxis::LeftStickX, -0.575);
    pad_button(&mut app, pad, GamepadButton::LeftTrigger2, 0.6);
    app.update();
    assert!(
        (actions(&app).value("steer") + 0.5).abs() < 1e-4,
        "{}",
        actions(&app).value("steer")
    );
    assert!(
        (actions(&app).value("throttle") - 0.6).abs() < 1e-4,
        "a half-pulled trigger"
    );
}

#[test]
fn touch_state_drives_actions_and_prompts() {
    let mut app = app(EDGE_MAP);
    {
        let mut t = app.world_mut().resource_mut::<TouchState>();
        t.held.insert("btn_fire".into());
        t.just_pressed.insert("btn_fire".into());
        t.sticks.insert("stick_move".into(), Vec2::new(0.0, -0.8));
        t.active = true;
    }
    click(&mut app, MouseButton::Left, true); // the browser's click from the same tap
    app.update();
    assert!(actions(&app).just_pressed("fire"));
    assert_eq!(actions(&app).axis("move"), Vec2::new(0.0, -0.8));
    assert_eq!(
        device(&app),
        Device::Touch,
        "the tap's click doesn't count as the mouse"
    );
    let map = app.world().resource::<ActionMap>().clone();
    assert_eq!(map.prompt("fire", device(&app)), "SHOOT");
}

#[test]
fn a_bot_holding_a_button_presses_it_once() {
    /// What the bot does each frame: `Some(down)` calls set_button, `None` leaves it alone.
    #[derive(Resource, Default)]
    struct Bot(Option<bool>);
    #[derive(Resource, Default)]
    struct Seen {
        pressed: u32,
        just_pressed: u32,
        just_released: u32,
    }
    let mut app = app(GODOT_MAP);
    app.init_resource::<Bot>()
        .init_resource::<Seen>()
        .add_systems(
            Update,
            (
                |bot: Res<Bot>, mut a: ResMut<Actions>| {
                    if let Some(down) = bot.0 {
                        a.set_button("dash", down);
                    }
                },
                |a: Res<Actions>, mut seen: ResMut<Seen>| {
                    seen.pressed += a.pressed("dash") as u32;
                    seen.just_pressed += a.just_pressed("dash") as u32;
                    seen.just_released += a.just_released("dash") as u32;
                },
            )
                .chain(),
        );
    let run = |app: &mut App, bot: Option<bool>, frames: u32| {
        app.world_mut().resource_mut::<Bot>().0 = bot;
        for _ in 0..frames {
            app.update();
        }
        let seen = app.world().resource::<Seen>();
        (seen.pressed, seen.just_pressed, seen.just_released)
    };
    assert_eq!(run(&mut app, Some(true), 30), (30, 1, 0), "held 30 frames");
    assert_eq!(
        run(&mut app, None, 5),
        (30, 1, 1),
        "the bot stops setting it"
    );
    assert_eq!(run(&mut app, Some(true), 10), (40, 2, 1), "held again");
    assert_eq!(run(&mut app, Some(false), 5), (40, 2, 2), "the bot lets go");
}

/// The frames, out of 60 with the arrow held, on which `menu_move` steps.
fn menu_steps(app: &mut App) -> Vec<u32> {
    key(app, KeyCode::ArrowDown, true);
    let mut at = Vec::new();
    for frame in 0..60 {
        app.update();
        if actions(app).step("menu_move") != IVec2::ZERO {
            at.push(frame);
        }
    }
    key(app, KeyCode::ArrowDown, false);
    app.update();
    at
}

#[test]
fn menus_and_the_wheel_keep_time_while_the_game_is_paused() {
    let running = menu_steps(&mut app(GODOT_MAP));
    assert!(running.len() >= 5, "{running:?}");
    // A pause or settings screen usually pauses virtual time; the menu on it still repeats.
    let mut paused = app(GODOT_MAP);
    paused.world_mut().resource_mut::<Time<Virtual>>().pause();
    assert_eq!(menu_steps(&mut paused), running);

    let mut app = app(EDGE_MAP);
    app.world_mut().resource_mut::<Time<Virtual>>().pause();
    let mut presses = 0;
    for _ in 0..3 {
        scroll(&mut app, -12.0);
        for _ in 0..60 {
            app.update();
            presses += actions(&app).just_pressed("next_tool") as u32;
        }
    }
    assert_eq!(presses, 3, "three notches a second apart are three presses");
}

#[test]
fn a_pad_pair_drives_one_axis() {
    let map = ActionMap::from_json(EDGE_MAP).unwrap();
    use GamepadButton::*;
    let slide = map.get("slide").unwrap();
    let pads: Vec<_> = slide.gamepad.iter().map(|b| b.1).collect();
    assert_eq!(
        pads,
        [
            Binding::PadPair(LeftTrigger2, RightTrigger2),
            Binding::PadPair(DPadLeft, DPadRight)
        ]
    );
    assert_eq!(
        map.prompt("slide", Device::Gamepad(Glyphs::Swapped)),
        "ZL / ZR"
    );
    assert_eq!(map.fill("{slide} to slide", LETTERS), "LT / RT to slide");

    let mut app = app(EDGE_MAP);
    let pad = connect_pad(&mut app, None);
    pad_button(&mut app, pad, RightTrigger2, 0.6);
    app.update();
    assert!((actions(&app).value("slide") - 0.6).abs() < 1e-4);
    pad_button(&mut app, pad, LeftTrigger2, 1.0);
    app.update();
    let v = actions(&app).value("slide");
    assert!((v + 0.4).abs() < 1e-4, "both pulled: {v}");
    pad_button(&mut app, pad, LeftTrigger2, 0.0);
    pad_button(&mut app, pad, RightTrigger2, 0.0);
    pad_button(&mut app, pad, DPadLeft, 1.0);
    app.update();
    assert_eq!(actions(&app).value("slide"), -1.0, "the d-pad pair");
}

#[test]
fn repeat_buttons_and_one_axis_repeat_like_menus() {
    let menu = menu_steps(&mut app(GODOT_MAP));
    let mut app = app(EDGE_MAP);
    key(&mut app, KeyCode::KeyR, true);
    key(&mut app, KeyCode::KeyX, true);
    let (mut presses, mut steps) = (Vec::new(), Vec::new());
    for frame in 0..60 {
        app.update();
        let a = actions(&app);
        assert!(a.pressed("next_page"), "held all along");
        if a.just_pressed("next_page") {
            presses.push(frame);
        }
        match a.step("zoom") {
            IVec2::ZERO => {}
            s => {
                assert_eq!(s, IVec2::X);
                steps.push(frame);
            }
        }
    }
    assert_eq!(
        presses, menu,
        "a repeat button presses again on the menu's beat"
    );
    assert_eq!(steps, menu, "a repeat axis_1d steps on the same beat");
    assert_eq!(actions(&app).value("zoom"), 1.0);
    assert!(!actions(&app).just_released("next_page"));
    key(&mut app, KeyCode::KeyR, false);
    key(&mut app, KeyCode::KeyX, false);
    app.update();
    assert!(actions(&app).just_released("next_page"));
    assert_eq!(actions(&app).step("zoom"), IVec2::ZERO);

    // A menu reads the raw stick: past half way steps, even though the dead zone leaves 0.47.
    let pad = connect_pad(&mut app, None);
    pad_axis(&mut app, pad, GamepadAxis::RightStickY, -0.55);
    app.update();
    assert_eq!(actions(&app).step("zoom"), IVec2::new(-1, 0));
    // A tap button (fire) held is still pressed only once.
    pad_button(&mut app, pad, GamepadButton::RightTrigger2, 1.0);
    let mut fired = 0;
    for _ in 0..60 {
        app.update();
        fired += actions(&app).just_pressed("fire") as u32;
    }
    assert_eq!(fired, 1);
}

#[test]
fn two_pads_dont_add_up_the_stronger_wins() {
    for strong_first in [true, false] {
        let mut app = app(EDGE_MAP);
        let first = connect_pad(&mut app, None);
        let second = connect_pad(&mut app, None);
        let (strong, weak) = if strong_first {
            (first, second)
        } else {
            (second, first)
        };
        pad_axis(&mut app, strong, GamepadAxis::LeftStickX, 0.9);
        pad_axis(&mut app, weak, GamepadAxis::LeftStickX, 0.575);
        app.update();
        let walk = (0.9 - STICK_DEADZONE) / (1.0 - STICK_DEADZONE);
        let a = actions(&app);
        assert!(
            close(a.axis("move"), Vec2::new(walk, 0.0)),
            "{}",
            a.axis("move")
        );
        assert!(
            (a.value("steer") - walk).abs() < 1e-4,
            "{}",
            a.value("steer")
        );
    }
    // The d-pads are read per direction: up on both and right on one is up and right.
    let mut app = app(GODOT_MAP);
    let (a, b) = (connect_pad(&mut app, None), connect_pad(&mut app, None));
    pad_button(&mut app, a, GamepadButton::DPadUp, 1.0);
    pad_button(&mut app, a, GamepadButton::DPadRight, 1.0);
    pad_button(&mut app, b, GamepadButton::DPadUp, 1.0);
    app.update();
    let mv = actions(&app).axis("move");
    assert!(close(mv, Vec2::ONE.normalize()), "{mv}");
}

#[test]
fn a_stick_pushed_past_half_way_is_using_the_pad() {
    let mut app = app(GODOT_MAP);
    let pad = connect_pad(&mut app, SONY);
    pad_axis(&mut app, pad, GamepadAxis::LeftStickX, 0.4);
    app.update();
    assert_eq!(device(&app), Device::KeyboardMouse, "a resting thumb");
    pad_axis(&mut app, pad, GamepadAxis::LeftStickX, 0.8);
    app.update();
    assert_eq!(device(&app), Device::Gamepad(Glyphs::Shapes));
    pad_axis(&mut app, pad, GamepadAxis::LeftStickX, 0.0);
    key(&mut app, KeyCode::KeyP, true);
    app.update();
    assert_eq!(device(&app), Device::KeyboardMouse);
    pad_axis(&mut app, pad, GamepadAxis::RightStickY, -0.8);
    app.update();
    assert_eq!(
        device(&app),
        Device::Gamepad(Glyphs::Shapes),
        "the right stick too"
    );
}

#[test]
fn a_pad_tap_inside_one_frame_still_counts() {
    let mut app = app(GODOT_MAP);
    let pad = connect_pad(&mut app, None);
    pad_button(&mut app, pad, GamepadButton::South, 1.0);
    pad_button(&mut app, pad, GamepadButton::South, 0.0);
    app.update();
    assert_eq!(
        actions(&app).button("dash"),
        ButtonState {
            pressed: false,
            just_pressed: true,
            just_released: true
        }
    );
}

#[test]
fn menus_step_from_a_stick_past_half_way() {
    let mut app = app(GODOT_MAP);
    let pad = connect_pad(&mut app, None);
    // 0.55 is past MENU_PUSH; after the walking dead zone it would be 0.47 and not step.
    pad_axis(&mut app, pad, GamepadAxis::LeftStickY, 0.55);
    app.update();
    assert_eq!(actions(&app).step("menu_move"), IVec2::new(0, 1));
}

#[test]
fn a_key_and_a_trigger_on_one_axis_stop_at_one() {
    let mut app = app(EDGE_MAP);
    let pad = connect_pad(&mut app, None);
    key(&mut app, KeyCode::KeyW, true);
    pad_button(&mut app, pad, GamepadButton::LeftTrigger2, 1.0);
    app.update();
    assert_eq!(actions(&app).value("throttle"), 1.0);
}

#[test]
fn a_key_and_a_touch_in_one_frame_count_as_the_keyboard() {
    let mut app = app(EDGE_MAP);
    app.world_mut()
        .resource_mut::<TouchState>()
        .just_pressed
        .insert("btn_fire".into());
    key(&mut app, KeyCode::Tab, true);
    app.update();
    assert_eq!(device(&app), Device::KeyboardMouse);
}
