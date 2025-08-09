use bevy::{
    app::{Plugin, Startup},
    asset::AssetServer,
    color::Color,
    ecs::system::{Commands, Res},
    hierarchy::{BuildChildren, ChildBuild},
    text::{JustifyText, TextColor, TextFont, TextLayout},
    ui::{
        BackgroundColor, BorderColor, JustifyContent, Node, PositionType, UiRect, Val, widget::Text,
    },
    utils::default,
};

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let container = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        ..default()
    };

    let button = (
        // BorderColor(Color::BLACK),
        // BackgroundColor(Color::srgb(1.0, 0.65, 0.65)),
        Node {
            width: Val::Px(200.),
            height: Val::Px(200.),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
    );

    // commands.spawn(container).with_children(|parent| {
    //     parent.spawn((
    //         button,
    //         Text::new("press"),
    //         TextColor(Color::WHITE),
    //         TextLayout::new_with_justify(JustifyText::Center),
    //         TextFont::default().with_font_size(40.0),
    //     ));
    // });

    commands.spawn((
        Text::new("hello\nbevy!"),
        TextFont {
            // This font is loaded and will be used instead of the default font.
            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
            font_size: 67.0,
            ..default()
        },
        // Set the justification of the Text
        TextLayout::new_with_justify(JustifyText::Center),
        // Set the style of the Node itself.
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        },
    ));
}

#[derive(Default)]
pub struct ButtonPlugin;
impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, setup);
    }
}
