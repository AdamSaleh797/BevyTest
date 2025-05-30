use bevy::{
    app::{FixedUpdate, Plugin, Startup},
    asset::Assets,
    color::Color,
    ecs::{
        component::Component,
        system::{Commands, ResMut, Single},
    },
    math::primitives::Ellipse,
    render::mesh::{Mesh, Mesh2d},
    sprite::{ColorMaterial, MeshMaterial2d},
    transform::components::Transform,
};

use crate::palette::Palette;

enum PrimaryColor {
    Red,
    Yellow,
    Blue,
}

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
        match self.colors {
            (PrimaryColor::Yellow, PrimaryColor::Yellow) => Palette::Yellow,
            (PrimaryColor::Red, PrimaryColor::Red) => Palette::Red,
            (PrimaryColor::Blue, PrimaryColor::Blue) => Palette::Blue,
            (PrimaryColor::Yellow, PrimaryColor::Red)
            | (PrimaryColor::Red, PrimaryColor::Yellow) => Palette::Orange,
            (PrimaryColor::Yellow, PrimaryColor::Blue)
            | (PrimaryColor::Blue, PrimaryColor::Yellow) => Palette::Green,
            (PrimaryColor::Blue, PrimaryColor::Red) | (PrimaryColor::Red, PrimaryColor::Blue) => {
                Palette::Purple
            }
        }
    }
}

pub fn setup(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let pool = Pool::new();

    //shape of pool
    let pool_shape = Ellipse::new(100., 60.);
    let mesh = meshes.add(pool_shape);

    //derive color of pool
    let color = pool.color().to_bevy_color();
    let material = materials.add(color);

    let transform = Transform::from_xyz(0., 150., 0.);
    let mesh_component = Mesh2d(mesh);
    let color_component = MeshMaterial2d(material);

    commands.spawn((pool, transform, mesh_component, color_component));
}

fn change_color(
    pool_query: Single<(&mut Pool, &mut MeshMaterial2d<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (mut pool, mut color) = pool_query.into_inner();

    pool.colors = (PrimaryColor::Blue, PrimaryColor::Yellow);

    let new_color = pool.color().to_bevy_color();
    let material = materials.add(new_color);
    *color = MeshMaterial2d(material);
}
pub struct PoolPlugin;
impl Plugin for PoolPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, setup)
            .add_systems(FixedUpdate, change_color);
    }
}
