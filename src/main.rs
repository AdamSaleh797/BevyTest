mod circle;
mod framerate_counter;
mod inertia;
mod mouse_drag;
mod pool;
mod palette;

use bevy::{DefaultPlugins, app::App};
use circle::CirclePlugin;
use framerate_counter::FrameratePlugin;
use inertia::InertiaPlugin;
use mouse_drag::MouseDragPlugin;
use pool::PoolPlugin;

fn main() {
    println!("Git Test");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameratePlugin)
        .add_plugins(CirclePlugin)
        .add_plugins(MouseDragPlugin)
        .add_plugins(InertiaPlugin)
        .add_plugins(PoolPlugin)
        .run();
}
