use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum Action {
    #[actionlike(Axis)]
    Zoom,

    #[actionlike(DualAxis)]
    Move,
    #[actionlike(DualAxis)]
    MoveDrag,

    Delete,

    SelectMove,
    SelectMoveCamera,
    SelectScale,
    SelectOrder,

    // debug
    TogglePhysics,
    ToggleAABB,
}

impl Action {
    pub fn input_map() -> InputMap<Action> {
        InputMap::default()
            .with_axis(Action::Zoom, MouseScrollAxis::Y)
            .with_dual_axis(Action::Move, VirtualDPad::wasd())
            .with_dual_axis(Action::MoveDrag, MouseMove::default())
            .with(Action::SelectMove, MouseButton::Left)
            .with(Action::SelectMoveCamera, MouseButton::Middle)
            .with(Action::SelectScale, KeyCode::ShiftLeft)
            .with(Action::SelectOrder, KeyCode::ControlLeft)
            .with(Action::Delete, KeyCode::KeyX)
            // f1 used to toggle editor
            .with(Action::TogglePhysics, KeyCode::F2)
            .with(Action::ToggleAABB, KeyCode::F3)
    }
}
