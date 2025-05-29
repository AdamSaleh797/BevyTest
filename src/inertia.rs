use bevy::{
    app::{FixedUpdate, Plugin},
    ecs::{component::Component, system::Query},
    math::{Vec2, Vec3Swizzles},
    transform::components::Transform,
};

#[derive(Component)]
pub struct Inertia {
    pub target_position: Vec2,
    velocity: Vec2,
}
impl Inertia {
    pub fn new(target_position: Vec2) -> Self {
        Self {
            target_position,
            velocity: Vec2::ZERO,
        }
    }
}

fn apply_inertia(mut inertia_query: Query<(&mut Inertia, &mut Transform)>) {
    for (mut inertia, mut transform) in &mut inertia_query {
        let direction = inertia.target_position - transform.translation.xy();
        inertia.velocity = 0.7 * inertia.velocity + 0.1 * direction;
        transform.translation.x += inertia.velocity.x;
        transform.translation.y += inertia.velocity.y;
    }
}

pub struct InertiaPlugin;
impl Plugin for InertiaPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(FixedUpdate, apply_inertia);
    }
}
