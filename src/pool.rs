use bevy::{
    app::{Plugin, Startup, Update},
    asset::Assets,
    ecs::{
        component::Component,
        query::With,
        system::{Commands, ResMut, Single},
    },
    math::primitives::Ellipse,
    render::mesh::Mesh2d,
    sprite::{ColorMaterial, MeshMaterial2d},
};
use bevy_world_space::{
    position::Position,
    world_unit::{WorldUnit, WorldVec2},
};

use crate::{color_mix_resource::ColorMixRes, palette::Palette, shapes::RenderingParams};

#[derive(Component)]
struct Pool;

fn setup(mut rendering_params: RenderingParams, mut commands: Commands) {
    //shape of pool
    let pool_shape = Ellipse::new(100., 60.);
    let mesh = rendering_params.meshes.add(pool_shape);

    //derive color of pool
    let material = rendering_params
        .materials
        .add(Palette::Yellow.to_bevy_color());

    let position = Position::new(
        WorldVec2::new(WorldUnit::ZERO, WorldUnit::ONE * 5.),
        WorldUnit::ONE,
        200,
        0.,
    );
    let mesh_component = Mesh2d(mesh);
    let color_component = MeshMaterial2d(material);

    commands.spawn((Pool, position, mesh_component, color_component));
}

fn update_color(
    color_mix: Single<&ColorMixRes>,
    pool_query: Single<&mut MeshMaterial2d<ColorMaterial>, With<Pool>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut color = pool_query.into_inner();

    let pool_color = color_mix.color_mix().pot().color();
    let new_color = Palette::from(pool_color).to_bevy_color();
    let material = materials.add(new_color);
    *color = MeshMaterial2d(material);
}
pub struct PoolPlugin;
impl Plugin for PoolPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_color);
    }
}
