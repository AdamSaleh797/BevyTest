use bevy::{
    app::{Plugin, Startup},
    asset::Assets,
    ecs::{
        component::Component,
        system::{Commands, Res, ResMut},
    },
    log::info,
    math::primitives::CircularSector,
    render::mesh::{Mesh, Mesh2d},
    sprite::{ColorMaterial, MeshMaterial2d},
};
use bevy_world_space::{
    position::Position,
    world_unit::{AspectRatio, WorldUnit, WorldVec2},
};
use strum::IntoEnumIterator;

use crate::palette::{Palette, PrimaryColor, PrimaryColorTable};

#[derive(Component)]
struct Bank {
    count: PrimaryColorTable<u32>,
}
impl Bank {
    fn new() -> Self {
        Self {
            count: PrimaryColorTable::filled(5),
        }
    }
}

fn bank_icon(
    icon_color: PrimaryColor,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) -> (Mesh2d, MeshMaterial2d<ColorMaterial>) {
    let bank_icon_shape = CircularSector::from_degrees(15., 180.);
    let mesh = meshes.add(bank_icon_shape);
    let color = icon_color.to_bevy_color();
    let material = materials.add(color);

    let mesh_component = Mesh2d(mesh);
    let color_component = MeshMaterial2d(material);

    (mesh_component, color_component)
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    aspect_ratio: Res<AspectRatio>,
) {
    let bank = Bank::new();

    info!("setup!");
    for (row, color) in PrimaryColor::iter().enumerate() {
        for count in 0..bank.count[color] {
            let position = Position::new(
                WorldVec2::new(
                    WorldUnit::left(&aspect_ratio) + (count + 1) as f32 * WorldUnit::ONE,
                    WorldUnit::top(&aspect_ratio) - (row + 1) as f32 * WorldUnit::ONE,
                ),
                WorldUnit::ONE,
                30,
                0.,
            );
            let (mesh_component, color_component) = bank_icon(color, &mut meshes, &mut materials);
            commands.spawn((position, mesh_component, color_component));
        }
    }
}

pub struct BankPlugin;
impl Plugin for BankPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, setup);
    }
}
