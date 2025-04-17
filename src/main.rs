mod framerate_counter;

use bevy::{DefaultPlugins, app::App};
use framerate_counter::FrameratePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameratePlugin)
        .run();
}
