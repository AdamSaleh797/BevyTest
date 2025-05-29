use bevy::{
    app::{FixedUpdate, Plugin},
    ecs::{component::Component, system::Query},
    math::{Vec2, Vec3Swizzles},
    transform::components::Transform,
};

pub struct InertiaParams {
    jiggle: f32,
    snap: f32,
}
impl InertiaParams {
    pub fn plunging() -> Self {
        Self {
            jiggle: 0.7,
            snap: 0.1,
        }
    }
}

#[derive(Component)]
pub struct Inertia {
    pub target_position: Vec2,
    velocity: Vec2,
    params: InertiaParams,
}
impl Inertia {
    pub fn new(target_position: Vec2, params: InertiaParams) -> Self {
        Self {
            target_position,
            velocity: Vec2::ZERO,
            params,
        }
    }
}

fn apply_inertia(mut inertia_query: Query<(&mut Inertia, &mut Transform)>) {
    for (mut inertia, mut transform) in &mut inertia_query {
        let direction = inertia.target_position - transform.translation.xy();
        inertia.velocity =
            inertia.params.jiggle * inertia.velocity + inertia.params.snap * direction;
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
