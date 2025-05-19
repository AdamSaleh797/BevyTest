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
    mut dragging: ResMut<DraggingState>,
    draggable: Single<&Transform, With<Draggable>>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    if let Some(cursor_position) = cursor_position(&window) {
        let offset = draggable.translation.xy() - cursor_position;
        if offset.length() <= 50. && input.just_pressed(MouseButton::Left) {
            *dragging = DraggingState::Dragging { offset };
        }
    }
    if input.just_released(MouseButton::Left) {
        *dragging = DraggingState::Idle;
    }
}

fn drag(
    mut query: Query<&mut Transform, With<Draggable>>,
    window: Single<&Window, With<PrimaryWindow>>,
    dragging: Res<DraggingState>,
) {
    // let offset = match *dragging {
    //     DraggingState::Idle => return,
    //     DraggingState::Dragging { offset } => offset
    // };
    let DraggingState::Dragging { offset } = *dragging else {
        return;
    };
    for mut transform in &mut query {
        if let Some(cursor_position) = cursor_position(&window) {
            transform.translation.x = cursor_position.x + offset.x;
            transform.translation.y = cursor_position.y + offset.y;
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
enum DraggingState {
    Dragging { offset: Vec2 },
    Idle,
}
impl DraggingState {
    fn new() -> Self {
        Self::Idle
    }
    fn is_idle(&self) -> bool {
        matches!(self, Self::Idle)
    }
}

pub struct MouseDragPlugin;
impl Plugin for MouseDragPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(DraggingState::new())
            .add_systems(Update, mouse_inputs)
            .add_systems(Update, drag);
    }
}
