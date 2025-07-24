use bevy::{
    app::{Plugin, Update},
    asset::Assets,
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        query::{With, Without},
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res, ResMut, Resource},
    },
    input::{ButtonInput, keyboard::KeyCode},
    math::primitives::Rectangle,
    render::mesh::{Mesh, Mesh2d},
    sprite::{ColorMaterial, MeshMaterial2d},
};
use bevy_world_space::{position::Position, win_info::WinInfo, world_unit::WorldVec2};

use crate::{bounding_box::BoundingBox, shapes::RenderingParams};

#[derive(Resource, Default)]
struct VisualizationState {
    visualization_on: bool,
}

#[derive(Component)]
struct BoxSkeleton {
    parent_id: Entity,
    offset: WorldVec2,
}

fn toggle(
    keys: Res<ButtonInput<KeyCode>>,
    bounding_box_query: Query<(&BoundingBox, Entity)>,
    box_skeleton_query: Query<Entity, With<BoxSkeleton>>,
    rendering_params: RenderingParams,
    commands: Commands,
    mut visualization_state: ResMut<VisualizationState>,
    win_info: Res<WinInfo>,
) {
    if !keys.just_pressed(KeyCode::KeyB) {
        return;
    }
    if visualization_state.visualization_on {
        disable_visualization(box_skeleton_query, commands);
        visualization_state.visualization_on = false;
    } else {
        enable_visualization(bounding_box_query, rendering_params, commands, &win_info);
        visualization_state.visualization_on = true;
    }
}

fn enable_visualization(
    bounding_box_query: Query<(&BoundingBox, Entity)>,
    mut rendering_params: RenderingParams,
    mut commands: Commands,
    win_info: &WinInfo,
) {
    for (bounding_box, id) in &bounding_box_query {
        let rectangle = bounding_box.bounding_box().to_rect(win_info);
        let mesh = rendering_params
            .meshes
            .add(Rectangle::new(rectangle.width(), rectangle.height()));
        let color = Color::srgb(0., 1., 0.);
        let material = rendering_params.materials.add(color);

        let position = Position::new(
            bounding_box.bounding_box().center(),
            bounding_box.bounding_box().width(),
            rectangle.width() as u32,
            -1.,
        );
        let mesh_component = Mesh2d(mesh);
        let color_component = MeshMaterial2d(material);

        commands.spawn((
            BoxSkeleton {
                parent_id: id,
                offset: bounding_box.bounding_box().center(),
            },
            position,
            mesh_component,
            color_component,
        ));
    }
}

fn disable_visualization(
    box_skeleton_query: Query<Entity, With<BoxSkeleton>>,
    mut commands: Commands,
) {
    for box_skeleton in &box_skeleton_query {
        commands.entity(box_skeleton).despawn();
    }
}

fn update_position(
    mut box_skeleton_query: Query<(&BoxSkeleton, &mut Position)>,
    parent_query: Query<&Position, Without<BoxSkeleton>>,
) {
    for (box_skeleton, mut position) in &mut box_skeleton_query {
        if let Ok(pos) = parent_query.get(box_skeleton.parent_id) {
            position.pos = pos.pos + box_skeleton.offset;
        }
    }
}

#[derive(Default)]
pub struct BoundingBoxVisualizationPlugin;
impl Plugin for BoundingBoxVisualizationPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<VisualizationState>()
            .add_systems(Update, toggle)
            .add_systems(Update, update_position.after(toggle));
    }
}
