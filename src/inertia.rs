use bevy::{
    app::{FixedUpdate, Plugin},
    ecs::{component::Component, system::Query},
};
use bevy_world_space::{position::Position, world_unit::WorldVec2};

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
    pub target_position: WorldVec2,
    velocity: WorldVec2,
    params: InertiaParams,
}
impl Inertia {
    pub fn new(target_position: WorldVec2, params: InertiaParams) -> Self {
        Self {
            target_position,
            velocity: WorldVec2::ZERO,
            params,
        }
    }
}

fn apply_inertia(mut inertia_query: Query<(&mut Inertia, &mut Position)>) {
    for (mut inertia, mut position) in &mut inertia_query {
        let direction = inertia.target_position - position.pos;
        inertia.velocity =
            inertia.params.jiggle * inertia.velocity + inertia.params.snap * direction;
        position.pos.x += inertia.velocity.x;
        position.pos.y += inertia.velocity.y;
    }
}

pub struct InertiaPlugin;
impl Plugin for InertiaPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(FixedUpdate, apply_inertia);
    }
}
