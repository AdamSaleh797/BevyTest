mod bank;
mod bounding_box;
mod circle;
mod color_mix_resource;
mod debug;
mod framerate_counter;
mod hp;
mod inertia;
mod mouse_drag;
mod palette;
mod pool;
mod shapes;
mod ui_components;

use bank::BankPlugin;
use bevy::{
    DefaultPlugins,
    app::App,
    asset::{AssetMetaCheck, AssetMode, AssetPlugin},
    prelude::PluginGroup,
    utils::default,
};
use bevy_world_space::{WorldSpacePlugins, world_init::WorldInitPlugin};
use circle::CirclePlugin;
use framerate_counter::FrameratePlugin;
use hp::HpPlugin;
use inertia::InertiaPlugin;
use mouse_drag::MouseDragPlugin;
use pool::PoolPlugin;

use crate::{color_mix_resource::ColorMixPlugin, debug::DebugPlugins};

fn main() {
    println!("Git Test");
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            mode: AssetMode::Unprocessed,
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .add_plugins(WorldSpacePlugins.set(WorldInitPlugin {
            screen_width: 1280.,
            screen_height: 720.,
        }))
        .add_plugins(FrameratePlugin)
        .add_plugins(ColorMixPlugin)
        .add_plugins(CirclePlugin)
        .add_plugins(MouseDragPlugin)
        .add_plugins(InertiaPlugin)
        .add_plugins(PoolPlugin)
        .add_plugins(BankPlugin)
        .add_plugins(DebugPlugins)
        .add_plugins(HpPlugin)
        .run();
}
