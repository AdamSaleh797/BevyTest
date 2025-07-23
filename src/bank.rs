use bevy::{
    app::{Plugin, Startup},
    ecs::{
        component::Component,
        system::{Commands, Res},
    },
    log::info,
    math::primitives::CircularSector,
    render::mesh::Mesh2d,
    sprite::{ColorMaterial, MeshMaterial2d},
};
use bevy_world_space::{
    position::Position,
    win_info::WinInfo,
    world_unit::{WorldUnit, WorldVec2},
};
use strum::IntoEnumIterator;

use crate::{
    palette::{PrimaryColor, PrimaryColorTable},
    shapes::RenderingParams,
};

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
    rendering_params: &mut RenderingParams,
) -> (Mesh2d, MeshMaterial2d<ColorMaterial>) {
    let bank_icon_shape = CircularSector::from_degrees(15., 180.);
    let mesh = rendering_params.meshes.add(bank_icon_shape);
    let color = icon_color.to_bevy_color();
    let material = rendering_params.materials.add(color);

    let mesh_component = Mesh2d(mesh);
    let color_component = MeshMaterial2d(material);

    (mesh_component, color_component)
}

fn setup(mut commands: Commands, mut rendering_params: RenderingParams, win_info: Res<WinInfo>) {
    let bank = Bank::new();

    info!("setup!");
    for (row, color) in PrimaryColor::iter().enumerate() {
        for count in 0..bank.count[color] {
            let position = Position::new(
                WorldVec2::new(
                    WorldUnit::left(win_info.aspect_ratio()) + (count + 1) as f32 * WorldUnit::ONE,
                    WorldUnit::top(win_info.aspect_ratio()) - (row + 1) as f32 * WorldUnit::ONE,
                ),
                WorldUnit::ONE,
                30,
                0.,
            );
            let (mesh_component, color_component) = bank_icon(color, &mut rendering_params);
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
