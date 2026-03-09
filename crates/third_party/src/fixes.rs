use bevy::prelude::*;
use bevy_fix_cursor_unlock_web::FixPointerUnlockPlugin;

pub fn plugin(app: &mut App) {
    app.add_plugins(FixPointerUnlockPlugin);
}
