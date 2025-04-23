mod circle;
mod framerate_counter;
mod mouse_drag;

use bevy::{DefaultPlugins, app::App};
use circle::CirclePlugin;
use framerate_counter::FrameratePlugin;
use mouse_drag::MouseDragPlugin;

fn main() {
    println!("Git Test");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameratePlugin)
        .add_plugins(CirclePlugin)
        .add_plugins(MouseDragPlugin)
        .run();
}
