use bevy::{
    app::{Plugin, Update},
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Query, Res, ResMut, Resource, Single},
    },
    input::{ButtonInput, mouse::MouseButton},
    math::{Vec2},
    window::{PrimaryWindow, Window},
};
use bevy_world_space::{position::Position, win_info::WinInfo, world_unit::{AspectRatio, WorldUnit, WorldVec2}};

use crate::inertia::{Inertia, InertiaParams};

//mouse inputs shouldnt have draggable/window params fix later
fn mouse_inputs(
    mut commands: Commands,
    input: Res<ButtonInput<MouseButton>>,
    mut dragging: ResMut<DraggingState>,
    draggable_query: Query<(&Position, Entity), With<Draggable>>,
    window: Single<&Window, With<PrimaryWindow>>,
    win_info: Res<WinInfo>,
    aspect_ratio: Res<AspectRatio>,
) {
    if let Some(cursor_position) = cursor_position(&window) {
        for (position, id) in &draggable_query {
            let offset = position.pos - WorldVec2::from_window_screen_pos(cursor_position, &win_info, &aspect_ratio); 
            if offset.length() <= WorldUnit::ONE * (50.) && input.just_pressed(MouseButton::Left) {
                *dragging = DraggingState::Dragging { offset, id };
                commands.entity(id).insert(Inertia::new(
                    position.pos,
                    InertiaParams::plunging(),
                ));
                break;
            }
        }
    }
    if input.just_released(MouseButton::Left) {
        if let DraggingState::Dragging { id, .. } = *dragging {
            commands.entity(id).remove::<Inertia>();
            *dragging = DraggingState::Idle;
        }
    }
}

fn drag(
    mut query: Query<&mut Inertia, With<Draggable>>,
    window: Single<&Window, With<PrimaryWindow>>,
    dragging: Res<DraggingState>,
    win_info: Res<WinInfo>,
    aspect_ratio: Res<AspectRatio>,
) {
    // let offset = match *dragging {
    //     DraggingState::Idle => return,
    //     DraggingState::Dragging { offset } => offset
    // };
    let DraggingState::Dragging { offset, id } = *dragging else {
        return;
    };
    let Ok(mut inertia) = query.get_mut(id) else {
        return;
    };
    if let Some(cursor_position) = cursor_position(&window) {
        let cursor_position = WorldVec2::from_window_screen_pos(cursor_position, &win_info, &aspect_ratio);
        inertia.target_position.x = cursor_position.x + offset.x;
        inertia.target_position.y = cursor_position.y + offset.y;
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
    Dragging { offset: WorldVec2, id: Entity },
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
