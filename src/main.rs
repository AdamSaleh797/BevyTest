mod circle;
mod framerate_counter;

use bevy::{DefaultPlugins, app::App};
use circle::CirclePlugin;
use framerate_counter::FrameratePlugin;

fn main() {
    println!("Git Test");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameratePlugin)
        .add_plugins(CirclePlugin)
        .run();
}
