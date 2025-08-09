use bevy::{
    app::{Plugin, Startup, Update},
    ecs::{
        bundle::Bundle,
        component::Component,
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res},
    },
    math::primitives::CircularSector,
    render::{mesh::Mesh2d, view::Visibility},
    sprite::{ColorMaterial, MeshMaterial2d},
};
use bevy_world_space::{
    position::Position,
    win_info::WinInfo,
    world_unit::{WorldUnit, WorldVec2},
};

use crate::{
    color_mix::color::PrimaryColor,
    color_mix_resource::{self, ColorMixRes},
    palette::Palette,
    shapes::RenderingParams,
};

struct Bank;
impl Bank {
    const SECTOR_RADIUS: u32 = 15;
}

#[derive(Component)]
struct BankPiece {
    color: PrimaryColor,
    index: u32,
}

#[derive(Bundle)]
struct BankPieceEntity {
    piece: BankPiece,
    position: Position,
    mesh_component: Mesh2d,
    color_component: MeshMaterial2d<ColorMaterial>,
    visibility: Visibility,
}

fn bank_icon(
    icon_color: PrimaryColor,
    rendering_params: &mut RenderingParams,
) -> (Mesh2d, MeshMaterial2d<ColorMaterial>) {
    let bank_icon_shape = CircularSector::from_degrees(Bank::SECTOR_RADIUS as f32, 180.);
    let mesh = rendering_params.meshes.add(bank_icon_shape);
    let color = Palette::from(icon_color).to_bevy_color();
    let material = rendering_params.materials.add(color);

    let mesh_component = Mesh2d(mesh);
    let color_component = MeshMaterial2d(material);

    (mesh_component, color_component)
}

fn setup(
    mut commands: Commands,
    mut rendering_params: RenderingParams,
    color_mix: Res<ColorMixRes>,
    win_info: Res<WinInfo>,
) {
    const WIDTH: WorldUnit = WorldUnit::new(1.5);
    let bank = color_mix.color_mix().bank();

    for (row, color) in PrimaryColor::all_colors().enumerate() {
        for count in 0..bank.remaining(color) {
            let piece = BankPiece {
                color,
                index: count,
            };
            let position = Position::new(
                WorldVec2::new(
                    WorldUnit::left(win_info.aspect_ratio()) + (count + 1) as f32 * WIDTH,
                    WorldUnit::top(win_info.aspect_ratio()) - (row + 1) as f32 * WIDTH,
                ),
                WIDTH,
                2 * Bank::SECTOR_RADIUS,
                0.,
            );
            let (mesh_component, color_component) = bank_icon(color, &mut rendering_params);
            commands.spawn(BankPieceEntity {
                piece,
                position,
                mesh_component,
                color_component,
                visibility: Visibility::Visible,
            });
        }
    }
}

fn update_bank(color_mix: Res<ColorMixRes>, mut pieces: Query<(&BankPiece, &mut Visibility)>) {
    let bank = color_mix.color_mix().bank();
    for (piece, mut visibility) in &mut pieces {
        *visibility = if piece.index < bank.remaining(piece.color) {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

pub struct BankPlugin;
impl Plugin for BankPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, setup.after(color_mix_resource::spawn_color_mix))
            .add_systems(Update, update_bank);
    }
}
