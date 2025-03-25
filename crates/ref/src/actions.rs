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

    SelectAll,
    SelectMove,
    SelectMoveCamera,
    SelectScale,
    SelectOrder,

    Paste,
}

impl Action {
    pub fn input_map() -> InputMap<Action> {
        InputMap::default()
            .with_axis(Action::Zoom, MouseScrollAxis::Y)
            .with_dual_axis(Action::Move, VirtualDPad::wasd())
            .with_dual_axis(Action::MoveDrag, MouseMove::default())            
            .with(Action::SelectAll, KeyCode::ShiftLeft)
            .with(Action::SelectAll, KeyCode::ShiftRight)
            
            .with(Action::SelectMove, MouseButton::Left)
            .with(Action::SelectMoveCamera, MouseButton::Middle)
            .with(Action::SelectScale, KeyCode::ShiftLeft)
            .with(Action::SelectOrder, KeyCode::ControlLeft)
            .with(Action::Delete, KeyCode::KeyX)
            .with(Action::Paste,  ButtonlikeChord::new([KeyCode::ControlLeft, KeyCode::KeyV]))
            // f1 used to toggle editor
    }
}
