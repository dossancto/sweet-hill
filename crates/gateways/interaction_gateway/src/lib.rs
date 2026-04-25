use bevy::prelude::*;

mod detect_can_interact;
mod ensurements;
mod interact;
mod ui;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        detect_can_interact::plugin,
        ui::plugin,
        interact::plugin,
        ensurements::plugin,
    ));
}
