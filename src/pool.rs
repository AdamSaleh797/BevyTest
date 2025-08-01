use bevy::{
    app::{Plugin, Startup, Update},
    asset::Assets,
    ecs::{
        component::Component,
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

use crate::{
    palette::{Palette, PrimaryColor},
    shapes::RenderingParams,
};

#[derive(Component)]
struct Pool {
    colors: (PrimaryColor, PrimaryColor),
}
impl Pool {
    fn new() -> Self {
        Self {
            colors: (PrimaryColor::Yellow, PrimaryColor::Yellow),
        }
    }
    fn color(&self) -> Palette {
        let (c1, c2) = self.colors;
        c1.blend(c2)
    }
}

fn setup(mut rendering_params: RenderingParams, mut commands: Commands) {
    let pool = Pool::new();

    //shape of pool
    let pool_shape = Ellipse::new(100., 60.);
    let mesh = rendering_params.meshes.add(pool_shape);

    //derive color of pool
    let color = pool.color().to_bevy_color();
    let material = rendering_params.materials.add(color);

    let position = Position::new(
        WorldVec2::new(WorldUnit::ZERO, WorldUnit::ONE * 5.),
        WorldUnit::ONE,
        200,
        0.,
    );
    let mesh_component = Mesh2d(mesh);
    let color_component = MeshMaterial2d(material);

    commands.spawn((pool, position, mesh_component, color_component));
}

fn update_color(
    pool_query: Single<(&Pool, &mut MeshMaterial2d<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (pool, mut color) = pool_query.into_inner();

    let new_color = pool.color().to_bevy_color();
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
