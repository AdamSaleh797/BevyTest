use bevy::{
    app::{Plugin, Startup},
    color::Color,
    ecs::system::Commands,
    hierarchy::{BuildChildren, ChildBuild},
    ui::{BackgroundColor, JustifyContent, Node, UiRect, Val},
    utils::default,
};

fn setup(mut commands: Commands) {
    let container = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        ..default()
    };

    let square = Node {
        width: Val::Px(200.),
        height: Val::Px(200.),
        border: UiRect::all(Val::Px(2.)),
        ..default()
    };

    let background_color = BackgroundColor(Color::srgb(0.65, 0.65, 0.65));

    commands.spawn(container).with_children(|parent| {
        parent.spawn((square, background_color));
    });
}

#[derive(Default)]
pub struct ButtonPlugin;
impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, setup);
    }
}
