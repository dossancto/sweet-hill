use bevy_enhanced_input::prelude::InputAction;

#[derive(Debug, InputAction)]
#[action_output(bool)]
pub struct GunFireTrigger;
