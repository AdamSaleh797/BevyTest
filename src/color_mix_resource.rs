use bevy::{
    app::{Plugin, Startup},
    ecs::system::{Commands, Resource},
};

use crate::color_mix::game::ColorMix;

#[derive(Resource)]
pub struct ColorMixRes {
    color_mix: ColorMix,
}

impl ColorMixRes {
    pub fn color_mix(&self) -> &ColorMix {
        &self.color_mix
    }
}

pub fn spawn_color_mix(mut commands: Commands) {
    commands.insert_resource(ColorMixRes {
        color_mix: ColorMix::new(5),
    });
}

pub struct ColorMixPlugin;

impl Plugin for ColorMixPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, spawn_color_mix);
    }
}
