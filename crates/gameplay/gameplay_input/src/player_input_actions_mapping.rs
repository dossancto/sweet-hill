//! Input handling for the player.

use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};
use bevy_ahoy::prelude::*;
use bevy_enhanced_input::prelude::{Press, *};

use core_gameplay::{
    flashlight::states::ToggleFlashlight,
    player::states::{BlocksInput, Interact},
};
use gun::inputs::{GunAimTrigger, GunFireTrigger, GunReloadTrigger, ToogleActiveGun};
use states::player::Player;

#[derive(Debug, Component, Default)]
#[component(on_add = PlayerInputContext::on_add)]
pub(crate) struct PlayerInputContext;

impl PlayerInputContext {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        world
            .commands()
            .entity(ctx.entity)
            .insert(actions!(PlayerInputContext[
                (
                    Action::<Movement>::new(),
                    ActionSettings { consume_input: false, ..default() },
                    DeadZone::default(),
                    Bindings::spawn((
                        Cardinal::wasd_keys(),
                        Axial::left_stick()
                    ))
                ),
                (
                    Action::<Jump>::new(),
                    ActionSettings { consume_input: false, ..default() },
                    Press::default(),
                    bindings![
                        KeyCode::Space,
                        GamepadButton::South,
                    ],
                ),
                (
                    Action::<Tac>::new(),
                    ActionSettings { consume_input: false, ..default() },
                    Press::default(),
                    bindings![
                        KeyCode::Space,
                        GamepadButton::South,
                    ],
                ),
                (
                    Action::<Crane>::new(),
                    ActionSettings { consume_input: false, ..default() },
                    Press::default(),
                    bindings![
                        KeyCode::Space,
                        GamepadButton::South,
                    ],
                ),
                (
                    Action::<Mantle>::new(),
                    ActionSettings { consume_input: false, ..default() },
                    Hold::new(0.2),
                    bindings![
                        KeyCode::Space,
                        GamepadButton::South,
                    ],
                ),
                (
                    Action::<Climbdown>::new(),
                    ActionSettings { consume_input: false, ..default() },
                    bindings![KeyCode::ControlLeft, GamepadButton::LeftTrigger2],
                ),
                (
                    Action::<Crouch>::new(),
                    ActionSettings { consume_input: false, ..default() },
                    bindings![KeyCode::ControlLeft, GamepadButton::LeftTrigger2],
                ),
                (
                    Action::<SwimUp>::new(),
                    ActionSettings { consume_input: false, ..default() },
                    bindings![KeyCode::Space, GamepadButton::South],
                ),
                (
                    Action::<PullObject>::new(),
                    ActionSettings { consume_input: true, ..default() },
                    Press::default(),
                    bindings![MouseButton::Right],
                ),
                (
                    Action::<DropObject>::new(),
                    ActionSettings { consume_input: true, ..default() },
                    Press::default(),
                    bindings![MouseButton::Right],
                ),
                (
                    Action::<ThrowObject>::new(),
                    ActionSettings { consume_input: true, ..default() },
                    Press::default(),
                    bindings![MouseButton::Left],
                ),
                (
                    Action::<RotateCamera>::new(),
                    ActionSettings { consume_input: false, ..default() },

                    Bindings::spawn((
                        Spawn((Binding::mouse_motion(), Scale::splat(0.07))),
                        Axial::right_stick(),
                    ))
                ),
                (
                    Action::<Interact>::new(),
                    Hold::new(1f32),
                    bindings![KeyCode::KeyE, GamepadButton::South]
                ),
                (
                    Action::<ToggleFlashlight>::new(),
                    Press::new(1f32),
                    bindings![KeyCode::KeyF, GamepadButton::West]
                ),
                (
                    Action::<GunFireTrigger>::new(),
                    Hold::new(10f32).one_shot(false),
                    bindings![MouseButton::Left, GamepadButton::RightTrigger2]
                ),
                (
                    Action::<GunAimTrigger>::new(),
                    ActionSettings { consume_input: true, ..default() },
                    Hold::new(10f32).one_shot(false),
                    bindings![MouseButton::Right, GamepadButton::LeftTrigger2],
                ),
                (
                    Action::<GunReloadTrigger>::new(),
                    bindings![KeyCode::KeyR, GamepadButton::East]
                ),
                (
                    Action::<ToogleActiveGun>::new(),
                    bindings![Binding::mouse_wheel(), GamepadButton::North]
                ),
            ]));
    }
}

pub(super) fn update_player_input_binding(
    player: Single<Entity, With<Player>>,
    blocks_input: Res<BlocksInput>,
    mut commands: Commands,
) {
    if blocks_input.is_empty() {
        commands.entity(*player).insert(PlayerInputContext);
    } else {
        commands
            .entity(*player)
            .remove_with_requires::<PlayerInputContext>()
            .despawn_related::<Actions<PlayerInputContext>>();
    }
}
