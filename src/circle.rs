use bevy::{
    app::{FixedUpdate, Plugin, Startup},
    asset::Assets,
    color::Color,
    core_pipeline::core_2d::Camera2d,
    ecs::{
        component::Component,
        query::With,
        system::{Commands, Query, ResMut},
    },
    math::primitives::Circle,
    render::mesh::{Mesh, Mesh2d},
    sprite::{ColorMaterial, MeshMaterial2d},
    transform::components::Transform,
};

#[derive(Component)]
struct CircleId;

fn setup(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    commands.spawn(Camera2d);

    let circle = Circle::new(50.);
    let mesh = meshes.add(circle);
    let color = Color::srgb(1., 0., 0.);
    let material = materials.add(color);

    let transform = Transform::from_xyz(0., 0., 0.);
    let mesh_component = Mesh2d(mesh);
    let color_component = MeshMaterial2d(material);

    commands.spawn((CircleId, transform, mesh_component, color_component));
}

fn move_entities(mut query: Query<&mut Transform, With<CircleId>>) {
    for mut shape in &mut query {
        shape.translation.x += 1.;
    }
}

pub struct CirclePlugin;
impl Plugin for CirclePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, setup)
            .add_systems(FixedUpdate, move_entities);
    }
}
