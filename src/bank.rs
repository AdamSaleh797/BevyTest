use bevy::{
    app::{FixedUpdate, Plugin, Startup, Update},
    ecs::{
        bundle::Bundle,
        component::Component,
        event::{Event, EventReader, EventWriter},
        system::{Commands, Query, Res, ResMut, Resource},
    },
    input::{ButtonInput, keyboard::KeyCode},
    math::primitives::CircularSector,
    render::{mesh::Mesh2d, view::Visibility},
    sprite::{ColorMaterial, MeshMaterial2d},
};
use bevy_world_space::{
    position::Position,
    win_info::WinInfo,
    world_unit::{WorldUnit, WorldVec2},
};
use strum::IntoEnumIterator;

use crate::{
    palette::{Palette, PrimaryColor, PrimaryColorTable},
    shapes::RenderingParams,
};

#[derive(Event)]
struct TakeBankColorEvent(PrimaryColor);

#[derive(Event)]
struct PlayPieceColorEvent(Palette);

#[derive(Resource)]
struct Bank {
    count: PrimaryColorTable<u32>,
}
impl Bank {
    const COLOR_SLICE_COUNT: u32 = 5;
    const SECTOR_RADIUS: u32 = 15;
}
impl Default for Bank {
    fn default() -> Self {
        Self {
            count: PrimaryColorTable::filled(Self::COLOR_SLICE_COUNT),
        }
    }
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
    let color = icon_color.to_bevy_color();
    let material = rendering_params.materials.add(color);

    let mesh_component = Mesh2d(mesh);
    let color_component = MeshMaterial2d(material);

    (mesh_component, color_component)
}

fn setup(mut commands: Commands, mut rendering_params: RenderingParams, win_info: Res<WinInfo>) {
    const WIDTH: WorldUnit = WorldUnit::new(1.5);

    for (row, color) in PrimaryColor::iter().enumerate() {
        for count in 0..Bank::COLOR_SLICE_COUNT {
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

fn update_bank(
    mut take_color: EventReader<TakeBankColorEvent>,
    mut play_piece: EventReader<PlayPieceColorEvent>,
    mut bank: ResMut<Bank>,
    mut pieces: Query<(&BankPiece, &mut Visibility)>,
) {
    let mut updated = false;
    for &TakeBankColorEvent(color) in take_color.read() {
        // TODO: Add these asserts back once game logic is implemented.
        // debug_assert!(bank.count[color] >= 2);
        bank.count[color] = bank.count[color].saturating_sub(2);
        updated = true;
    }
    for &PlayPieceColorEvent(color) in play_piece.read() {
        let (c1, c2) = color.constituents();
        // debug_assert!(bank.count[c1] < Bank::COLOR_SLICE_COUNT);
        // debug_assert!(bank.count[c2] < Bank::COLOR_SLICE_COUNT);
        bank.count[c1] = (bank.count[c1] + 1).min(Bank::COLOR_SLICE_COUNT);
        bank.count[c2] = (bank.count[c2] + 1).min(Bank::COLOR_SLICE_COUNT);
        updated = true;
    }

    if updated {
        for (piece, mut visibility) in &mut pieces {
            *visibility = if piece.index < bank.count[piece.color] {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}

fn test(
    input: Res<ButtonInput<KeyCode>>,
    mut take_color: EventWriter<TakeBankColorEvent>,
    mut play_piece: EventWriter<PlayPieceColorEvent>,
) {
    if !input.pressed(KeyCode::ShiftLeft) {
        if input.just_pressed(KeyCode::KeyR) {
            take_color.send(TakeBankColorEvent(PrimaryColor::Red));
        }
        if input.just_pressed(KeyCode::KeyY) {
            take_color.send(TakeBankColorEvent(PrimaryColor::Yellow));
        }
        if input.just_pressed(KeyCode::KeyB) {
            take_color.send(TakeBankColorEvent(PrimaryColor::Blue));
        }
    } else {
        if input.just_pressed(KeyCode::KeyR) {
            play_piece.send(PlayPieceColorEvent(Palette::Red));
        }
        if input.just_pressed(KeyCode::KeyY) {
            play_piece.send(PlayPieceColorEvent(Palette::Yellow));
        }
        if input.just_pressed(KeyCode::KeyB) {
            play_piece.send(PlayPieceColorEvent(Palette::Blue));
        }
        if input.just_pressed(KeyCode::KeyG) {
            play_piece.send(PlayPieceColorEvent(Palette::Green));
        }
        if input.just_pressed(KeyCode::KeyO) {
            play_piece.send(PlayPieceColorEvent(Palette::Orange));
        }
        if input.just_pressed(KeyCode::KeyP) {
            play_piece.send(PlayPieceColorEvent(Palette::Purple));
        }
    }
}

pub struct BankPlugin;
impl Plugin for BankPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<TakeBankColorEvent>()
            .add_event::<PlayPieceColorEvent>()
            .init_resource::<Bank>()
            .add_systems(Startup, setup)
            .add_systems(FixedUpdate, update_bank)
            .add_systems(Update, test);
    }
}
