//! the action map's action map (`controls/actions.json`, schema 1) turned into per-frame action
//! state for keyboard, mouse and gamepad, with prompt text that follows the device used last.
//! Game code asks about actions by id ("dash", "move"), never about keys or buttons: every binding
//! and every prompt comes from the map. The Bevy twin of the godot-action-map module.
//!
//! Bevy 0.20 with its `keyboard`, `mouse` and `gamepad` features; serde and serde_json.

use std::collections::{HashMap, HashSet};

use bevy::input::InputSystems;
use bevy::input::gamepad::{Gamepad, GamepadAxis, GamepadButton};
use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy::prelude::*;
use serde::Deserialize;

/// Stick travel ignored around the centre, as a fraction of full tilt.
pub const STICK_DEADZONE: f32 = 0.15;
/// A stick or d-pad pushed this far counts as one menu step, and as using the pad.
pub const MENU_PUSH: f32 = 0.5;
/// `repeat` actions fire again after this long held, then every `MENU_REPEAT_S`.
pub const MENU_DELAY_S: f32 = 0.4;
pub const MENU_REPEAT_S: f32 = 0.12;
/// One wheel gesture is one press: a trackpad or smooth wheel sends scroll for a while, with
/// empty frames between, so after a press the wheel stays quiet until it has been still this long.
pub const WHEEL_GAP_S: f32 = 0.2;
/// Mouse movement shorter than this in one frame (pixels) doesn't switch prompts to the keyboard.
pub const MOUSE_NUDGE_PX: f32 = 4.0;

// ---------------------------------------------------------------- the file

#[derive(Deserialize)]
struct MapFile {
    #[serde(default = "schema_one")]
    schema_version: u64,
    actions: Vec<MapAction>,
    #[serde(default)]
    touch_layout: TouchLayoutFile,
}

fn schema_one() -> u64 {
    1
}

#[derive(Deserialize, Default)]
struct TouchLayoutFile {
    reference_px: Option<[f32; 2]>,
    #[serde(default)]
    controls: Vec<TouchControl>,
}

#[derive(Deserialize)]
struct MapAction {
    id: String,
    #[serde(default)]
    label: String,
    #[serde(default)]
    context: String,
    #[serde(default = "button_kind")]
    kind: String,
    #[serde(default)]
    press: String,
    #[serde(default)]
    toggle_option: bool,
    #[serde(default)]
    bindings: HashMap<String, Vec<String>>,
}

fn button_kind() -> String {
    "button".into()
}

/// One on-screen control from `touch_layout.controls`: its centre as fractions of the screen, and
/// a radius (a share of the screen's height) or, for an `area`, a width and height.
#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct TouchControl {
    pub id: String,
    #[serde(default)]
    pub label: String,
    /// `button`, `stick` or `area`.
    #[serde(rename = "type", default)]
    pub kind: String,
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32,
    #[serde(default)]
    pub r: f32,
    #[serde(default)]
    pub w: f32,
    #[serde(default)]
    pub h: f32,
}

// ---------------------------------------------------------------- the map

/// What an action is: a button, one axis (steer, throttle) or two (walk, look).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Button,
    Axis1d,
    Axis2d,
}

/// One input from the map, as Bevy names it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Binding {
    Key(KeyCode),
    /// Two keys as one axis, negative then positive (`A/D`).
    KeyPair(KeyCode, KeyCode),
    /// Four keys as two axes: up, down, left, right (`WASD`, `Arrows`, `IJKL`).
    Keys4([KeyCode; 4]),
    Mouse(MouseButton),
    /// Mouse movement (`Mouse`); read it with `Actions::mouse_delta`.
    MouseMotion,
    /// The mouse wheel (`MouseWheel`), either way. One gesture is one press.
    Wheel,
    /// A button. In Bevy `LeftTrigger` is the bumper and `LeftTrigger2` the trigger.
    Pad(GamepadButton),
    /// Two pad buttons or triggers as one axis, negative then positive (`trigger_left/trigger_right`).
    PadPair(GamepadButton, GamepadButton),
    StickLeft,
    StickRight,
    /// One axis of a stick (`stick_left_x`).
    StickAxis(GamepadAxis),
    /// The d-pad as two axes.
    Dpad,
}

impl Binding {
    /// Whether an action of this kind can use the binding.
    fn fits(self, kind: Kind) -> bool {
        use Binding::*;
        match kind {
            Kind::Button => matches!(self, Key(_) | Mouse(_) | Wheel | Pad(_)),
            Kind::Axis1d => matches!(
                self,
                Key(_) | KeyPair(..) | Pad(_) | PadPair(..) | StickAxis(_)
            ),
            Kind::Axis2d => matches!(self, Keys4(_) | MouseMotion | StickLeft | StickRight | Dpad),
        }
    }
}

/// One action as read from the map. Bindings keep their map names, for prompts.
#[derive(Clone, Debug)]
pub struct ActionDef {
    pub id: String,
    pub label: String,
    pub context: String,
    pub kind: Kind,
    /// `tap`, `hold`, `double_tap`, `analog` or `repeat`. Only `repeat` changes how it reads.
    pub press: String,
    /// The map offers a toggle for this held action; whether to use it is the game's call.
    pub toggle_option: bool,
    pub keyboard_mouse: Vec<(String, Binding)>,
    pub gamepad: Vec<(String, Binding)>,
    /// On-screen control ids from `touch_layout`.
    pub touch: Vec<String>,
}

/// The action map, read once. `ActionMapPlugin` inserts it as a resource.
#[derive(Resource, Clone, Debug)]
pub struct ActionMap {
    /// Every action, in file order.
    pub actions: Vec<ActionDef>,
    /// `touch_layout.controls`, as read.
    pub touch_controls: Vec<TouchControl>,
    /// `touch_layout.reference_px` (the format's default, 1280 x 800, when it isn't set).
    pub reference_px: Vec2,
    /// Bindings and actions left out, each with why ("dash: Hyper: unknown key name").
    pub skipped: Vec<String>,
    by_id: HashMap<String, usize>,
}

impl ActionMap {
    /// Read a map. `Err` when the text isn't a schema 1 action map or repeats an action id. A
    /// binding this can't use is left out and listed in `skipped`, so one odd key doesn't stop the
    /// game; a test can check that `skipped` is empty.
    pub fn from_json(text: &str) -> Result<ActionMap, String> {
        let file: MapFile =
            serde_json::from_str(text).map_err(|e| format!("action map doesn't parse: {e}"))?;
        if file.schema_version != 1 {
            return Err(format!(
                "action map has schema_version {}; this reads 1",
                file.schema_version
            ));
        }
        let controls = file.touch_layout.controls;
        let mut skipped = Vec::new();
        let mut actions: Vec<ActionDef> = Vec::new();
        let mut by_id = HashMap::new();
        for a in file.actions {
            if by_id.contains_key(&a.id) {
                return Err(format!("action map has the action {} twice", a.id));
            }
            let kind = match a.kind.as_str() {
                "button" => Kind::Button,
                "axis_1d" => Kind::Axis1d,
                "axis_2d" => Kind::Axis2d,
                other => {
                    skipped.push(format!(
                        "{}: kind {other} isn't button, axis_1d or axis_2d",
                        a.id
                    ));
                    continue;
                }
            };
            let mut read = |device: &str, parse: fn(&str) -> Result<Binding, String>| {
                let mut out = Vec::new();
                for name in a.bindings.get(device).into_iter().flatten() {
                    match parse(name) {
                        Ok(b) if b.fits(kind) => out.push((name.clone(), b)),
                        Ok(_) => skipped
                            .push(format!("{}: {name}: can't drive a {} action", a.id, a.kind)),
                        Err(why) => skipped.push(format!("{}: {name}: {why}", a.id)),
                    }
                }
                out
            };
            let keyboard_mouse = read("keyboard_mouse", parse_key);
            let gamepad = read("gamepad", parse_pad);
            let mut touch = Vec::new();
            for t in a.bindings.get("touch").into_iter().flatten() {
                if controls.iter().any(|c| &c.id == t) {
                    touch.push(t.clone());
                } else {
                    skipped.push(format!("{}: {t}: not in touch_layout", a.id));
                }
            }
            by_id.insert(a.id.clone(), actions.len());
            actions.push(ActionDef {
                id: a.id,
                label: a.label,
                context: a.context,
                kind,
                press: a.press,
                toggle_option: a.toggle_option,
                keyboard_mouse,
                gamepad,
                touch,
            });
        }
        Ok(ActionMap {
            actions,
            touch_controls: controls,
            reference_px: Vec2::from(file.touch_layout.reference_px.unwrap_or([1280.0, 800.0])),
            skipped,
            by_id,
        })
    }

    pub fn get(&self, id: &str) -> Option<&ActionDef> {
        self.by_id.get(id).map(|&i| &self.actions[i])
    }

    /// The touch control ids bound to an action, e.g. "dash" -> ["btn_dash"].
    pub fn touch_ids(&self, id: &str) -> &[String] {
        self.get(id).map_or(&[], |a| a.touch.as_slice())
    }

    /// The action a touch control drives in a context ("play", "menu", "always" ...).
    pub fn action_for_touch(&self, control_id: &str, context: &str) -> Option<&str> {
        self.actions
            .iter()
            .find(|a| a.context == context && a.touch.iter().any(|t| t == control_id))
            .map(|a| a.id.as_str())
    }

    /// Prompt text for an action on a device: the label of its first usable binding there ("Space",
    /// "A", "Cross", a touch control's label), or "" when it has none there.
    pub fn prompt(&self, id: &str, device: Device) -> String {
        let Some(a) = self.get(id) else {
            return String::new();
        };
        match device {
            Device::KeyboardMouse => a.keyboard_mouse.first().map(|(n, _)| key_label(n)),
            Device::Gamepad(g) => a.gamepad.first().and_then(|(n, _)| pad_label(n, g)),
            Device::Touch => a.touch.first().map(|t| {
                let c = self.touch_controls.iter().find(|c| &c.id == t);
                c.filter(|c| !c.label.is_empty())
                    .map_or(t.clone(), |c| c.label.clone())
            }),
        }
        .unwrap_or_default()
    }

    /// Replace each `{action_id}` in a line with that action's prompt on `device`, or with its
    /// label when it has no binding there. Braces that don't name an action stay as they are.
    pub fn fill(&self, template: &str, device: Device) -> String {
        let mut out = String::with_capacity(template.len());
        let mut rest = template;
        while let Some(open) = rest.find('{') {
            out.push_str(&rest[..open]);
            let after = &rest[open + 1..];
            match after
                .find('}')
                .and_then(|close| Some((close, self.get(&after[..close])?)))
            {
                Some((close, a)) => {
                    let p = self.prompt(&a.id, device);
                    out.push_str(if p.is_empty() { &a.label } else { &p });
                    rest = &after[close + 1..];
                }
                None => {
                    out.push('{');
                    rest = after;
                }
            }
        }
        out.push_str(rest);
        out
    }
}

// ---------------------------------------------------------------- names

/// the action map's named keys: the map's name, Bevy's key and what players read (arrows as
/// words, since Bevy's default font has no arrow glyphs). Letters, digits and F1-F12 come below.
const NAMED_KEYS: &[(&str, KeyCode, &str)] = &[
    ("Space", KeyCode::Space, "Space"),
    ("Enter", KeyCode::Enter, "Enter"),
    ("Escape", KeyCode::Escape, "Esc"),
    ("Tab", KeyCode::Tab, "Tab"),
    ("Backspace", KeyCode::Backspace, "Backspace"),
    ("LeftShift", KeyCode::ShiftLeft, "Left Shift"),
    ("RightShift", KeyCode::ShiftRight, "Right Shift"),
    ("LeftCtrl", KeyCode::ControlLeft, "Left Ctrl"),
    ("RightCtrl", KeyCode::ControlRight, "Right Ctrl"),
    ("LeftAlt", KeyCode::AltLeft, "Left Alt"),
    ("RightAlt", KeyCode::AltRight, "Right Alt"),
    ("CapsLock", KeyCode::CapsLock, "Caps Lock"),
    ("ArrowUp", KeyCode::ArrowUp, "Up"),
    ("ArrowDown", KeyCode::ArrowDown, "Down"),
    ("ArrowLeft", KeyCode::ArrowLeft, "Left"),
    ("ArrowRight", KeyCode::ArrowRight, "Right"),
    ("Comma", KeyCode::Comma, ","),
    ("Period", KeyCode::Period, "."),
    ("Slash", KeyCode::Slash, "/"),
    ("Backslash", KeyCode::Backslash, "\\"),
    ("Semicolon", KeyCode::Semicolon, ";"),
    ("Quote", KeyCode::Quote, "'"),
    ("Backquote", KeyCode::Backquote, "`"),
    ("Minus", KeyCode::Minus, "-"),
    ("Equal", KeyCode::Equal, "="),
    ("BracketLeft", KeyCode::BracketLeft, "["),
    ("BracketRight", KeyCode::BracketRight, "]"),
];

fn key_code(name: &str) -> Option<KeyCode> {
    use KeyCode::*;
    const LETTERS: [KeyCode; 26] = [
        KeyA, KeyB, KeyC, KeyD, KeyE, KeyF, KeyG, KeyH, KeyI, KeyJ, KeyK, KeyL, KeyM, KeyN, KeyO,
        KeyP, KeyQ, KeyR, KeyS, KeyT, KeyU, KeyV, KeyW, KeyX, KeyY, KeyZ,
    ];
    const DIGITS: [KeyCode; 10] = [
        Digit0, Digit1, Digit2, Digit3, Digit4, Digit5, Digit6, Digit7, Digit8, Digit9,
    ];
    const F_KEYS: [KeyCode; 12] = [F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12];
    if let Some(&(_, code, _)) = NAMED_KEYS.iter().find(|k| k.0 == name) {
        return Some(code);
    }
    Some(match name.as_bytes() {
        &[c] if c.is_ascii_alphabetic() => LETTERS[(c.to_ascii_uppercase() - b'A') as usize],
        &[c] if c.is_ascii_digit() => DIGITS[(c - b'0') as usize],
        [b'F', n @ ..] => {
            let n: usize = std::str::from_utf8(n).ok()?.parse().ok()?;
            *F_KEYS.get(n.checked_sub(1)?)?
        }
        _ => return None,
    })
}

/// One keyboard or mouse name from the map, as a Bevy binding. `Err` says why it can't be used.
pub fn parse_key(name: &str) -> Result<Binding, String> {
    use KeyCode::*;
    Ok(match name {
        "WASD" => Binding::Keys4([KeyW, KeyS, KeyA, KeyD]),
        "Arrows" => Binding::Keys4([ArrowUp, ArrowDown, ArrowLeft, ArrowRight]),
        "IJKL" => Binding::Keys4([KeyI, KeyK, KeyJ, KeyL]),
        "Mouse" => Binding::MouseMotion,
        "MouseLeft" => Binding::Mouse(MouseButton::Left),
        "MouseRight" => Binding::Mouse(MouseButton::Right),
        "MouseMiddle" => Binding::Mouse(MouseButton::Middle),
        "MouseWheel" => Binding::Wheel,
        _ if name.len() > 1 && name.contains('+') => {
            return Err("chords (two keys at once) aren't supported".into());
        }
        _ if name.len() > 1 && name.contains('/') => {
            let (neg, pos) = name.split_once('/').unwrap_or_default();
            match (key_code(neg), key_code(pos)) {
                (Some(n), Some(p)) => Binding::KeyPair(n, p),
                _ => return Err("unknown key name in the pair".into()),
            }
        }
        _ => Binding::Key(key_code(name).ok_or("unknown key name")?),
    })
}

/// One controller name from the map (named by position), as a Bevy binding.
pub fn parse_pad(name: &str) -> Result<Binding, String> {
    use GamepadButton::*;
    if let Some((neg, pos)) = name.split_once('/').filter(|_| name.len() > 1) {
        return match (parse_pad(neg)?, parse_pad(pos)?) {
            (Binding::Pad(n), Binding::Pad(p)) => Ok(Binding::PadPair(n, p)),
            _ => Err("a controller pair needs two buttons or triggers".into()),
        };
    }
    Ok(match name {
        "stick_left" => Binding::StickLeft,
        "stick_right" => Binding::StickRight,
        "dpad" => Binding::Dpad,
        "stick_left_x" => Binding::StickAxis(GamepadAxis::LeftStickX),
        "stick_left_y" => Binding::StickAxis(GamepadAxis::LeftStickY),
        "stick_right_x" => Binding::StickAxis(GamepadAxis::RightStickX),
        "stick_right_y" => Binding::StickAxis(GamepadAxis::RightStickY),
        _ => Binding::Pad(match name {
            "face_south" => South,
            "face_east" => East,
            "face_west" => West,
            "face_north" => North,
            "shoulder_left" => LeftTrigger,
            "shoulder_right" => RightTrigger,
            "trigger_left" => LeftTrigger2,
            "trigger_right" => RightTrigger2,
            "select" => Select,
            "start" => Start,
            "stick_left_press" => LeftThumb,
            "stick_right_press" => RightThumb,
            "dpad_up" => DPadUp,
            "dpad_down" => DPadDown,
            "dpad_left" => DPadLeft,
            "dpad_right" => DPadRight,
            "grip_left" | "grip_right" => return Err("Bevy has no button for back grips".into()),
            _ => return Err("unknown controller input".into()),
        }),
    })
}

/// What players read for a keyboard or mouse name (the action map's labels).
pub fn key_label(name: &str) -> String {
    if let Some((a, b)) = name.split_once('/').filter(|_| name.len() > 1) {
        return format!("{} / {}", key_label(a), key_label(b));
    }
    let label = match name {
        "Arrows" => "Arrow keys",
        "WASD" => "W A S D",
        "IJKL" => "I J K L",
        "MouseLeft" => "Left click",
        "MouseRight" => "Right click",
        "MouseMiddle" => "Middle click",
        "MouseWheel" => "Mouse wheel",
        n => NAMED_KEYS.iter().find(|k| k.0 == n).map_or(n, |k| k.2),
    };
    // Single letters as printed on the key, so "e" shows as "E".
    if label.len() == 1 {
        label.to_ascii_uppercase()
    } else {
        label.to_string()
    }
}

/// What players read for a controller name in a button style (the action map's pad table); a
/// pair reads like "LT / RT".
pub fn pad_label(name: &str, glyphs: Glyphs) -> Option<String> {
    if let Some((a, b)) = name.split_once('/').filter(|_| name.len() > 1) {
        return Some(format!(
            "{} / {}",
            pad_label(a, glyphs)?,
            pad_label(b, glyphs)?
        ));
    }
    let (letters, shapes, swapped) = match name {
        "face_south" => ("A", "Cross", "B"),
        "face_east" => ("B", "Circle", "A"),
        "face_west" => ("X", "Square", "Y"),
        "face_north" => ("Y", "Triangle", "X"),
        "shoulder_left" => ("LB", "L1", "L"),
        "shoulder_right" => ("RB", "R1", "R"),
        "trigger_left" => ("LT", "L2", "ZL"),
        "trigger_right" => ("RT", "R2", "ZR"),
        "stick_left_press" => ("LS (press)", "L3", "Left stick (press)"),
        "stick_right_press" => ("RS (press)", "R3", "Right stick (press)"),
        "start" => ("Menu", "Options", "+"),
        "select" => ("View", "Create", "-"),
        // The same in every style.
        _ => {
            let same = match name {
                "stick_left" => "Left stick",
                "stick_left_x" => "Left stick left/right",
                "stick_left_y" => "Left stick up/down",
                "stick_right" => "Right stick",
                "stick_right_x" => "Right stick left/right",
                "stick_right_y" => "Right stick up/down",
                "dpad" => "D-pad",
                "dpad_up" => "D-pad up",
                "dpad_down" => "D-pad down",
                "dpad_left" => "D-pad left",
                "dpad_right" => "D-pad right",
                _ => return None,
            };
            (same, same, same)
        }
    };
    let label = match glyphs {
        Glyphs::Letters => letters,
        Glyphs::Shapes => shapes,
        Glyphs::Swapped => swapped,
    };
    Some(label.to_string())
}

// ---------------------------------------------------------------- devices

/// Which names a controller's buttons go by (the action map's three button styles).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Glyphs {
    /// A B X Y, LB, Menu: Xbox, Steam Deck and most other pads.
    #[default]
    Letters,
    /// Cross Circle Square Triangle, L1, Options: PlayStation.
    Shapes,
    /// B A Y X, L, +: Nintendo, confirm on the right.
    Swapped,
}

impl Glyphs {
    /// From the pad's USB vendor: Sony shapes, Nintendo swapped, anything else letters.
    pub fn for_vendor(vendor: Option<u16>) -> Glyphs {
        match vendor {
            Some(0x054c) => Glyphs::Shapes,
            Some(0x057e) => Glyphs::Swapped,
            _ => Glyphs::Letters,
        }
    }
}

/// The device used last; prompts show its keys or buttons. Changes only when the device does, so
/// `Res<Device>::is_changed()` says when to redraw prompts.
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Device {
    #[default]
    KeyboardMouse,
    Gamepad(Glyphs),
    Touch,
}

impl Device {
    /// The map's name for the device: `keyboard_mouse`, `gamepad` or `touch`.
    pub fn name(self) -> &'static str {
        match self {
            Device::KeyboardMouse => "keyboard_mouse",
            Device::Gamepad(_) => "gamepad",
            Device::Touch => "touch",
        }
    }
}

/// What on-screen controls report each frame, by control id from `touch_layout`. This module
/// draws nothing and reads no fingers: a touch-controls system fills this before `ActionMapSystems`.
#[derive(Resource, Clone, Debug, Default)]
pub struct TouchState {
    /// Controls a finger is on.
    pub held: HashSet<String>,
    /// Controls a finger came down on this frame (and `area` controls tapped).
    pub just_pressed: HashSet<String>,
    /// Stick controls, -1 to 1, y up.
    pub sticks: HashMap<String, Vec2>,
    /// A finger is on the screen (or just lifted), so mouse input this frame came from it: browsers
    /// turn taps into clicks, which mustn't switch prompts to the keyboard.
    pub active: bool,
}

// ---------------------------------------------------------------- per frame

/// One button action this frame.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ButtonState {
    pub pressed: bool,
    pub just_pressed: bool,
    pub just_released: bool,
}

#[derive(Clone, Copy, Debug, Default)]
struct Repeat {
    held: IVec2,
    wait: f32,
}

/// Every action's state this frame, by id. `ActionMapPlugin` updates it in `PreUpdate`.
#[derive(Resource, Clone, Debug, Default)]
pub struct Actions {
    buttons: HashMap<String, ButtonState>,
    axes: HashMap<String, Vec2>,
    steps: HashMap<String, IVec2>,
    mouse: HashMap<String, Vec2>,
    repeat: HashMap<String, Repeat>,
    wheel_quiet: f32,
    /// Each button's `pressed` as the last frame ended, bot writes included; the edges are worked
    /// out from it. `None` until `read_actions` first runs.
    last: Option<HashMap<String, bool>>,
}

impl Actions {
    pub fn button(&self, id: &str) -> ButtonState {
        self.buttons.get(id).copied().unwrap_or_default()
    }
    pub fn pressed(&self, id: &str) -> bool {
        self.button(id).pressed
    }
    /// True on the frame it goes down. A press and release inside one frame (a quick tap on a
    /// slow frame) gives `just_pressed` and `just_released` together, with `pressed` false. A
    /// `repeat` button is also just pressed again while held, as menu steps are.
    pub fn just_pressed(&self, id: &str) -> bool {
        self.button(id).just_pressed
    }
    pub fn just_released(&self, id: &str) -> bool {
        self.button(id).just_released
    }
    /// An `axis_2d` action: x right, y up, length at most 1 (sticks after the dead zone).
    pub fn axis(&self, id: &str) -> Vec2 {
        self.axes.get(id).copied().unwrap_or_default()
    }
    /// An `axis_1d` action, -1 to 1 (a trigger gives 0 to 1).
    pub fn value(&self, id: &str) -> f32 {
        self.axis(id).x
    }
    /// A `repeat` axis (menus): one step per push, then repeating while held. x right, y up; an
    /// `axis_1d` steps in x.
    pub fn step(&self, id: &str) -> IVec2 {
        self.steps.get(id).copied().unwrap_or_default()
    }
    /// Mouse movement this frame, in pixels, for an axis bound to `Mouse` (a look); zero otherwise.
    pub fn mouse_delta(&self, id: &str) -> Vec2 {
        self.mouse.get(id).copied().unwrap_or_default()
    }
    /// Set a button by hand, for bots and tests. Run after `ActionMapSystems`, every frame the bot
    /// holds it, or the next read overwrites it. The edges are worked out from the last frame, so a
    /// button held this way is just pressed once. With nothing reading (no `ActionMapPlugin`),
    /// each call counts as a frame.
    pub fn set_button(&mut self, id: &str, pressed: bool) {
        let was = match &self.last {
            Some(last) => last.get(id).copied().unwrap_or(false),
            None => self.pressed(id),
        };
        let state = ButtonState {
            pressed,
            just_pressed: pressed && !was,
            just_released: !pressed && was,
        };
        self.buttons.insert(id.to_string(), state);
    }
    /// Set an axis by hand (an `axis_1d` value goes in x). Same timing as `set_button`.
    pub fn set_axis(&mut self, id: &str, v: Vec2) {
        self.axes.insert(id.to_string(), v);
    }
}

/// The wheel this frame: `Some(true)` a press up, `Some(false)` down, `None` nothing. `quiet` is
/// the time left before the wheel can press again (`WHEEL_GAP_S` after any scroll).
pub fn wheel_step(quiet: &mut f32, delta_y: f32, dt: f32) -> Option<bool> {
    if delta_y == 0.0 {
        *quiet = (*quiet - dt).max(0.0);
        return None;
    }
    let press = (*quiet == 0.0).then_some(delta_y > 0.0);
    *quiet = WHEEL_GAP_S;
    press
}

/// A look stick: radial dead zone, then a squared response, so aim is fine near the centre and
/// full speed at the edge.
pub fn shape_stick(v: Vec2) -> Vec2 {
    let len = v.length().min(1.0);
    if len <= STICK_DEADZONE {
        return Vec2::ZERO;
    }
    let t = (len - STICK_DEADZONE) / (1.0 - STICK_DEADZONE);
    v.normalize() * t * t
}

/// A walking stick: dead zone, then linear, so a half push walks at half speed.
pub fn walk_stick(v: Vec2) -> Vec2 {
    let len = v.length().min(1.0);
    if len <= STICK_DEADZONE {
        return Vec2::ZERO;
    }
    v.normalize() * (len - STICK_DEADZONE) / (1.0 - STICK_DEADZONE)
}

/// One step per push of a `repeat` action: on the push, again after `MENU_DELAY_S` held, then every
/// `MENU_REPEAT_S`.
fn repeat_step(r: &mut Repeat, v: Vec2, dt: f32) -> IVec2 {
    let dir = |x: f32| (x > MENU_PUSH) as i32 - (x < -MENU_PUSH) as i32;
    let held = IVec2::new(dir(v.x), dir(v.y));
    let step = if held == IVec2::ZERO {
        IVec2::ZERO
    } else if held != r.held {
        r.wait = MENU_DELAY_S;
        held
    } else {
        r.wait -= dt;
        if r.wait <= 0.0 {
            r.wait = MENU_REPEAT_S;
            held
        } else {
            IVec2::ZERO
        }
    };
    r.held = held;
    step
}

/// The value with the largest size across every connected pad (two pads don't add up).
fn strongest<T: Copy + Default>(
    pads: &Query<&Gamepad>,
    f: impl Fn(&Gamepad) -> T,
    size: impl Fn(T) -> f32,
) -> T {
    pads.iter().map(f).fold(
        T::default(),
        |best, v| if size(v) > size(best) { v } else { best },
    )
}

/// Reads every action from the keyboard, mouse, pads and `TouchState` into `Actions`, and sets
/// `Device` to the device used last. Runs in `ActionMapSystems`.
#[allow(clippy::too_many_arguments)]
pub fn read_actions(
    map: Res<ActionMap>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    scroll: Res<AccumulatedMouseScroll>,
    pads: Query<&Gamepad>,
    touch: Res<TouchState>,
    // Real time, so menus repeat and the wheel settles while the game pauses or slows Time<Virtual>.
    time: Res<Time<Real>>,
    mut device: ResMut<Device>,
    mut out: ResMut<Actions>,
) {
    let dt = time.delta_secs();
    // Each button as the last frame ended, a bot's set_button included: the edges come from it.
    let last: HashMap<String, bool> = out
        .buttons
        .iter()
        .map(|(id, b)| (id.clone(), b.pressed))
        .collect();
    let mut next = Actions {
        wheel_quiet: out.wheel_quiet,
        repeat: std::mem::take(&mut out.repeat),
        ..default()
    };
    let wheel = wheel_step(&mut next.wheel_quiet, scroll.delta.y, dt).is_some();

    // The device used last decides the prompts: keyboard and mouse first, then touch, then pads.
    let pad_used = pads
        .iter()
        .filter(|g| {
            g.get_just_pressed().next().is_some()
                || g.left_stick().length() > MENU_PUSH
                || g.right_stick().length() > MENU_PUSH
        })
        .last();
    let kbm_used = keys.get_just_pressed().next().is_some()
        || (!touch.active
            && (mouse.get_just_pressed().next().is_some()
                || motion.delta.length() > MOUSE_NUDGE_PX
                || scroll.delta != Vec2::ZERO));
    let touch_used =
        !touch.just_pressed.is_empty() || touch.sticks.values().any(|v| v.length() > MENU_PUSH);
    let now = if kbm_used {
        Some(Device::KeyboardMouse)
    } else if touch_used {
        Some(Device::Touch)
    } else {
        pad_used.map(|g| Device::Gamepad(Glyphs::for_vendor(g.vendor_id())))
    };
    if let Some(d) = now {
        device.set_if_neq(d);
    }

    let key = |k: KeyCode| keys.pressed(k) as i32 as f32;
    for a in &map.actions {
        match a.kind {
            Kind::Button => {
                let (mut down, mut tapped) = (false, false);
                for (_, b) in &a.keyboard_mouse {
                    match *b {
                        Binding::Key(k) => {
                            down |= keys.pressed(k);
                            tapped |= keys.just_pressed(k);
                        }
                        Binding::Mouse(m) => {
                            down |= mouse.pressed(m);
                            tapped |= mouse.just_pressed(m);
                        }
                        // One wheel gesture is one press: down this frame, up the next.
                        Binding::Wheel => down |= wheel,
                        _ => {}
                    }
                }
                for (_, b) in &a.gamepad {
                    if let Binding::Pad(p) = *b {
                        down |= pads.iter().any(|g| g.pressed(p));
                        tapped |= pads.iter().any(|g| g.just_pressed(p));
                    }
                }
                for t in &a.touch {
                    down |= touch.held.contains(t);
                    tapped |= touch.just_pressed.contains(t);
                }
                let was = last.get(&a.id).copied().unwrap_or(false);
                let mut state = ButtonState {
                    pressed: down,
                    just_pressed: !was && (down || tapped),
                    just_released: !down && (was || tapped),
                };
                // Held, a `repeat` button presses again on the menu's beat.
                if a.press == "repeat" {
                    let r = next.repeat.entry(a.id.clone()).or_default();
                    let held = Vec2::new(down as i32 as f32, 0.0);
                    state.just_pressed |= repeat_step(r, held, dt) != IVec2::ZERO;
                }
                next.buttons.insert(a.id.clone(), state);
            }
            Kind::Axis1d => {
                let repeat = a.press == "repeat";
                let mut v = 0.0;
                for (_, b) in &a.keyboard_mouse {
                    match *b {
                        Binding::Key(k) => v += key(k),
                        Binding::KeyPair(neg, pos) => v += key(pos) - key(neg),
                        _ => {}
                    }
                }
                for (_, b) in &a.gamepad {
                    match *b {
                        Binding::StickAxis(ax) => {
                            let s = strongest(&pads, |g| g.get(ax).unwrap_or(0.0), f32::abs);
                            // A menu reads the raw stick, as `axis_2d` menus do.
                            v += if repeat {
                                s
                            } else {
                                walk_stick(Vec2::new(s, 0.0)).x
                            };
                        }
                        Binding::Pad(p) => {
                            v += strongest(&pads, |g| g.get(p).unwrap_or(0.0), f32::abs)
                        }
                        Binding::PadPair(neg, pos) => {
                            let pair =
                                |g: &Gamepad| g.get(pos).unwrap_or(0.0) - g.get(neg).unwrap_or(0.0);
                            v += strongest(&pads, pair, f32::abs);
                        }
                        _ => {}
                    }
                }
                let v = Vec2::new(v.clamp(-1.0, 1.0), 0.0);
                if repeat {
                    let r = next.repeat.entry(a.id.clone()).or_default();
                    next.steps.insert(a.id.clone(), repeat_step(r, v, dt));
                }
                next.axes.insert(a.id.clone(), v);
            }
            Kind::Axis2d => {
                let look = a
                    .keyboard_mouse
                    .iter()
                    .any(|(_, b)| *b == Binding::MouseMotion);
                let repeat = a.press == "repeat";
                let mut v = Vec2::ZERO;
                for (_, b) in &a.keyboard_mouse {
                    if let Binding::Keys4([up, down, left, right]) = *b {
                        v += Vec2::new(key(right) - key(left), key(up) - key(down));
                    }
                }
                for (_, b) in &a.gamepad {
                    let s = match *b {
                        Binding::StickLeft => strongest(&pads, Gamepad::left_stick, Vec2::length),
                        Binding::StickRight => strongest(&pads, Gamepad::right_stick, Vec2::length),
                        Binding::Dpad => pads
                            .iter()
                            .map(Gamepad::dpad)
                            .sum::<Vec2>()
                            .clamp(Vec2::NEG_ONE, Vec2::ONE),
                        _ => continue,
                    };
                    v += if repeat {
                        s
                    } else if look {
                        shape_stick(s)
                    } else {
                        walk_stick(s)
                    };
                }
                for t in &a.touch {
                    v += touch.sticks.get(t).copied().unwrap_or_default();
                }
                let v = v.clamp_length_max(1.0);
                if repeat {
                    let r = next.repeat.entry(a.id.clone()).or_default();
                    let step = repeat_step(r, v, dt);
                    next.steps.insert(a.id.clone(), step);
                }
                if look {
                    next.mouse.insert(a.id.clone(), motion.delta);
                }
                next.axes.insert(a.id.clone(), v);
            }
        }
    }
    next.last = Some(last);
    *out = next;
}

/// The system set that fills `Actions` and `Device`, in `PreUpdate` after Bevy's input systems.
/// Put systems that fill `TouchState` before it, and bots that set actions after it.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ActionMapSystems;

/// Inserts the map, `Actions`, `Device` and `TouchState`, and reads the actions every frame.
/// Needs Bevy's `InputPlugin`, which `DefaultPlugins` has.
pub struct ActionMapPlugin(pub ActionMap);

impl Plugin for ActionMapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.0.clone())
            .init_resource::<Actions>()
            .init_resource::<Device>()
            .init_resource::<TouchState>()
            .configure_sets(PreUpdate, ActionMapSystems.after(InputSystems))
            .add_systems(PreUpdate, read_actions.in_set(ActionMapSystems));
    }
}
