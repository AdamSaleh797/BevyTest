use bevy::{
    app::{FixedUpdate, Plugin, Update}, ecs::system::{Res, ResMut, Resource}, log::info, time::{Time, Timer, TimerMode}
};
use std::time::Duration;

#[derive(Resource)]
struct Framerate {
    frames: u32,
    timer: Timer,
}
impl Framerate {
    fn new() -> Framerate {
        Framerate {
            frames: 0,
            timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
        }
    }
}

fn print_framerate(mut framerate: ResMut<Framerate>, time: Res<Time>) {
    framerate.frames += 1;
    if framerate.timer.tick(time.delta()).just_finished() {
        info!("{} fps", framerate.frames);
        framerate.frames = 0;
    }
}

pub struct FrameratePlugin;
impl Plugin for FrameratePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(FixedUpdate, print_framerate)
            .insert_resource(Framerate::new());
    }
}
