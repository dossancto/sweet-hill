use std::any::TypeId;

use bevy::{platform::collections::HashSet, prelude::*};
use bevy_enhanced_input::prelude::*;

#[derive(Debug, InputAction)]
#[action_output(bool)]
pub struct Interact;

#[derive(Resource, Default, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct BlocksInput(HashSet<TypeId>);
