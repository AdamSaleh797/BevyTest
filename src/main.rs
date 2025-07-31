mod bank;
mod bounding_box;
mod circle;
mod debug;
mod framerate_counter;
mod inertia;
mod mouse_drag;
mod palette;
mod pool;
mod shapes;
mod ui_components;

use bank::BankPlugin;
use bevy::{DefaultPlugins, app::App, prelude::PluginGroup};
use bevy_world_space::{WorldSpacePlugins, world_init::WorldInitPlugin};
use circle::CirclePlugin;
use framerate_counter::FrameratePlugin;
use inertia::InertiaPlugin;
use mouse_drag::MouseDragPlugin;
use pool::PoolPlugin;

use crate::{debug::DebugPlugins, ui_components::UIPlugins};

fn main() {
    println!("Git Test");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldSpacePlugins.set(WorldInitPlugin {
            screen_width: 1280.,
            screen_height: 720.,
        }))
        .add_plugins(FrameratePlugin)
        .add_plugins(CirclePlugin)
        .add_plugins(MouseDragPlugin)
        .add_plugins(InertiaPlugin)
        .add_plugins(PoolPlugin)
        .add_plugins(BankPlugin)
        .add_plugins(DebugPlugins)
        .add_plugins(UIPlugins)
        .run();
}
