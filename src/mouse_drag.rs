use bevy::{
    app::{Plugin, Update},
    ecs::{
        component::Component,
        query::With,
        system::{Query, Res, ResMut, Resource, Single},
    },
    input::{ButtonInput, mouse::MouseButton},
    math::{Vec2, Vec3Swizzles},
    transform::components::Transform,
    window::{PrimaryWindow, Window},
};

//mouse inputs shouldnt have draggable/window params fix later
fn mouse_inputs(
    input: Res<ButtonInput<MouseButton>>,
    mut dragging: ResMut<Dragging>,
    draggable: Single<&Transform, With<Draggable>>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    if let Some(cursor_position) = cursor_position(&window) {
        if (draggable.translation.xy() - cursor_position).length() <= 50.
            && input.just_pressed(MouseButton::Left)
        {
            dragging.dragging = true;
        }
    }
    if input.just_released(MouseButton::Left) {
        dragging.dragging = false;
    }
}

fn drag(
    mut query: Query<&mut Transform, With<Draggable>>,
    window: Single<&Window, With<PrimaryWindow>>,
    dragging: Res<Dragging>,
) {
    if !dragging.dragging {
        return;
    }
    for mut transform in &mut query {
        if let Some(cursor_position) = cursor_position(&window) {
            transform.translation.x = cursor_position.x;
            transform.translation.y = cursor_position.y;
        }
    }
}

fn cursor_position(window: &Window) -> Option<Vec2> {
    window.cursor_position().map(|cursor_position| Vec2 {
        x: (cursor_position.x - (window.width() / 2.)),
        y: ((window.height() / 2.) - cursor_position.y),
    })
}

#[derive(Component)]
pub struct Draggable;

#[derive(Resource)]
struct Dragging {
    dragging: bool,
}
impl Dragging {
    fn new() -> Self {
        Self { dragging: false }
    }
}

pub struct MouseDragPlugin;
impl Plugin for MouseDragPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(Dragging::new())
            .add_systems(Update, mouse_inputs)
            .add_systems(Update, drag);
    }
}
