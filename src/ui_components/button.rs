use bevy::{
    app::{Plugin, Startup, Update},
    asset::AssetServer,
    color::{
        Color,
        palettes::css::{BLACK, BLUE, WHITE},
    },
    ecs::{
        query::Changed,
        system::{Commands, Query, Res},
    },
    hierarchy::{BuildChildren, ChildBuild},
    text::{JustifyText, TextColor, TextFont, TextLayout},
    ui::{
        BackgroundColor, BorderColor, Interaction, JustifyContent, Node, UiRect, Val, widget::Text,
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
        Node {
            width: Val::Px(200.),
            height: Val::Px(200.),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        BackgroundColor(Color::BLACK),
        BorderColor(Color::BLACK),
    );

    commands.spawn(container).with_children(|parent| {
        parent.spawn((
            button,
            Text::new("press"),
            TextColor(Color::WHITE),
            TextLayout::new_with_justify(JustifyText::Center),
            TextFont {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 67.0,
                ..default()
            },
        ));
    });
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn button_system(
    mut interactions: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Text,
        ),
        Changed<Interaction>,
    >,
) {
    for (interaction, mut color, mut border_color, mut text) in &mut interactions {
        match *interaction {
            Interaction::Pressed => {
                text.0 = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = BLUE.into();
            }
            Interaction::Hovered => {
                text.0 = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = WHITE.into();
            }
            Interaction::None => {
                text.0 = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = BLACK.into();
            }
        }
    }
}

#[derive(Default)]
pub struct ButtonPlugin;
impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, button_system);
    }
}
