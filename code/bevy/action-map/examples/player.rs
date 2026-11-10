//! The smallest use: move with the "move" action, dash with "dash", and keep a hint that names the
//! dash button on the device used last. In a game, add `ActionMapPlugin` and `PlayerPlugin` next to
//! `DefaultPlugins`; here `main` runs them with no window and presses Space once, so it runs anywhere.
//! cargo run --example player (from ../tests)
use action_map::{ActionMap, ActionMapPlugin, Actions, Device};
use bevy::input::keyboard::{Key, KeyboardInput, NativeKey};
use bevy::input::{ButtonState, InputPlugin};
use bevy::prelude::*;

/// In a game: include_str!("../controls/actions.json").
const ACTIONS_JSON: &str = include_str!("../tests/actions.json");

#[derive(Resource, Default)]
struct Player {
    pos: Vec2,
    hint: String,
}

struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Player>()
            .add_systems(Update, (walk, show_hint));
    }
}

fn walk(actions: Res<Actions>, time: Res<Time>, mut player: ResMut<Player>) {
    let speed = if actions.pressed("dash") {
        900.0
    } else {
        300.0
    };
    player.pos += actions.axis("move") * speed * time.delta_secs();
    if actions.just_pressed("dash") {
        println!("dash!");
    }
}

/// Redraw the hint only when the device changes (in a game, set a Text here).
fn show_hint(map: Res<ActionMap>, device: Res<Device>, mut player: ResMut<Player>) {
    if device.is_changed() {
        player.hint = map.fill("{dash} to dash", *device);
        println!("hint: {}", player.hint);
    }
}

fn main() {
    let map = ActionMap::from_json(ACTIONS_JSON).expect("controls/actions.json reads");
    for s in &map.skipped {
        println!("action map: skipped {s}");
    }
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        InputPlugin,
        ActionMapPlugin(map),
        PlayerPlugin,
    ));
    app.update();
    app.world_mut().write_message(KeyboardInput {
        key_code: KeyCode::Space,
        logical_key: Key::Unidentified(NativeKey::Unidentified),
        state: ButtonState::Pressed,
        text: None,
        repeat: false,
        window: Entity::PLACEHOLDER,
    });
    app.update();
}
