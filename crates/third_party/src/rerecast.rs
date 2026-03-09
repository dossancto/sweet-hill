use avian_rerecast::prelude::*;
use bevy::prelude::*;
use bevy_rerecast::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((NavmeshPlugins::default(), AvianBackendPlugin::default()));
}
