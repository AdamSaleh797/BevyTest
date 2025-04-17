mod framerate_counter;

use bevy::{DefaultPlugins, app::App};
use framerate_counter::FrameratePlugin;

fn main() {
    println!("Git Test");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameratePlugin)
        .run();
}
