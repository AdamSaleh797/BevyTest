use bevy::{
    app::{FixedUpdate, Plugin},
    ecs::{component::Component, system::Query},
    math::Vec2,
    transform::components::Transform,
};

#[derive(Component)]
pub struct Inertia {
    pub target_position: Vec2,
}

fn apply_inertia(mut inertia_query: Query<(&Inertia, &mut Transform)>) {
    for (inertia, mut transform) in &mut inertia_query {
        transform.translation.x = inertia.target_position.x;
        transform.translation.y = inertia.target_position.y;
    }
}

pub struct InertiaPlugin;
impl Plugin for InertiaPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(FixedUpdate, apply_inertia);
    }
}
