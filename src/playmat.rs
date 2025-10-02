use bevy::{
    app::{Plugin, Startup, Update},
    ecs::{
        component::Component,
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res},
    },
    math::primitives::Rectangle,
    render::mesh::Mesh2d,
    sprite::{ColorMaterial, MeshMaterial2d},
};
use bevy_world_space::{
    position::Position,
    win_info::WinInfo,
    world_unit::{WorldUnit, WorldVec2},
};

use crate::{
    color_mix_resource::{self, ColorMixRes},
    palette::Palette,
    shapes::RenderingParams,
};

enum Player {
    P1,
    P2,
}
#[derive(Component)]
struct Playmat {
    player: Player,
}

fn setup(
    mut rendering_params: RenderingParams,
    color_mix: Res<ColorMixRes>,
    mut commands: Commands,
    win_info: Res<WinInfo>,
) {
    //shape of playmats
    let playmat_shape = Rectangle::new(16., 8.);
    let mesh = rendering_params.meshes.add(playmat_shape);

    // PLAYMAT 1 SETUP //

    //color of playmat1
    let color1 = color_mix.color_mix().pot().colors()[0];
    let material1 = rendering_params
        .materials
        .add(Palette::from(color1).to_bevy_color());

    //position of playmat 1
    let position1 = Position::new(
        WorldVec2::new(
            WorldUnit::left(win_info.aspect_ratio()) + WorldUnit::new(8.),
            WorldUnit::bottom(win_info.aspect_ratio()) + WorldUnit::new(4.),
        ),
        WorldUnit::new(16.),
        16,
        0.,
    );
    let playmat1 = Playmat { player: Player::P1 };
    let mesh_component1 = Mesh2d(mesh.clone());
    let color_component1 = MeshMaterial2d(material1);

    // PLAYMAT 2 SETUP //

    //color of playmat 2
    let color2 = color_mix.color_mix().pot().colors()[1];
    let material2 = rendering_params
        .materials
        .add(Palette::from(color2).to_bevy_color());

    //position of playmat 2
    let position2 = Position::new(
        WorldVec2::new(
            WorldUnit::right(win_info.aspect_ratio()) - WorldUnit::new(8.),
            WorldUnit::bottom(win_info.aspect_ratio()) + WorldUnit::new(4.),
        ),
        WorldUnit::new(16.),
        16,
        0.,
    );
    let playmat2 = Playmat { player: Player::P2 };
    let mesh_component2 = Mesh2d(mesh);
    let color_component2 = MeshMaterial2d(material2);

    //spawn playmat 1
    commands.spawn((playmat1, mesh_component1, color_component1, position1));

    //spawn playmat 2
    commands.spawn((playmat2, mesh_component2, color_component2, position2));
}

fn update_playmat(
    color_mix: Res<ColorMixRes>,
    mut rendering_params: RenderingParams,
    mut hp_query: Query<(&mut MeshMaterial2d<ColorMaterial>, &Playmat)>,
) {
    for (mut color, playmat) in &mut hp_query {
        //color of playmat1
        let color1 = color_mix.color_mix().pot().colors()[0];
        let material1 = rendering_params
            .materials
            .add(Palette::from(color1).to_bevy_color());

        //color of playmat 2
        let color2 = color_mix.color_mix().pot().colors()[1];
        let material2 = rendering_params
            .materials
            .add(Palette::from(color2).to_bevy_color());

        match playmat.player {
            Player::P1 => *color = MeshMaterial2d(material1),
            Player::P2 => *color = MeshMaterial2d(material2),
        }
    }
}

pub struct PlaymatPlugin;
impl Plugin for PlaymatPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, setup.after(color_mix_resource::spawn_color_mix))
            .add_systems(Update, update_playmat);
    }
}
