use bevy::{
    app::{Plugin, Startup, Update},
    asset::AssetServer,
    ecs::{
        component::Component,
        query::With,
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res},
    },
    reflect::impl_reflect,
    text::{JustifyText, Text2d, TextFont, TextLayout},
    utils::default,
};
use bevy_world_space::{
    position::Position,
    win_info::WinInfo,
    world_unit::{WorldUnit, WorldVec2},
};

use crate::{
    color_mix_resource::{self, ColorMixRes},
    hp,
};

enum Player {
    P1,
    P2,
}
#[derive(Component)]
struct HP {
    player: Player,
}

fn setup(
    color_mix: Res<ColorMixRes>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    win_info: Res<WinInfo>,
) {
    let hp1_value = color_mix.color_mix().players().0.hp();
    let hp2_value = color_mix.color_mix().players().1.hp();

    let text_font = TextFont {
        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
        font_size: 50.0,
        ..default()
    };

    let position1 = Position::new(
        WorldVec2::new(
            WorldUnit::left(win_info.aspect_ratio()) + WorldUnit::new(5.),
            WorldUnit::ZERO,
        ),
        WorldUnit::ONE,
        25,
        0.,
    );
    let hp1 = HP { player: Player::P1 };

    let position2 = Position::new(
        WorldVec2::new(
            WorldUnit::right(win_info.aspect_ratio()) - WorldUnit::new(5.),
            WorldUnit::ZERO,
        ),
        WorldUnit::ONE,
        25,
        0.,
    );
    let hp2 = HP { player: Player::P2 };

    commands.spawn((
        hp1,
        Text2d::new(hp1_value.to_string()),
        text_font.clone(),
        TextLayout::new_with_justify(JustifyText::Center),
        position1,
    ));

    commands.spawn((
        hp2,
        Text2d::new(hp2_value.to_string()),
        text_font,
        TextLayout::new_with_justify(JustifyText::Center),
        position2,
    ));
}

fn update_hp(color_mix: Res<ColorMixRes>, mut hp_query: Query<(&mut Text2d, &HP)>) {
    let hp1 = color_mix.color_mix().players().0.hp();
    let hp2 = color_mix.color_mix().players().1.hp();

    for (mut text, hp) in &mut hp_query {
        match hp.player {
            Player::P1 => *text = Text2d::new(hp1.to_string()),
            Player::P2 => *text = Text2d::new(hp2.to_string()),
        }
    }
}

pub struct HpPlugin;
impl Plugin for HpPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, setup.after(color_mix_resource::spawn_color_mix))
            .add_systems(Update, update_hp);
    }
}
