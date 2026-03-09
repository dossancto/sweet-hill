//! The game's main screen states and transitions between them.

mod gameplay;
pub(crate) mod loading;
mod splash;
mod title;

use bevy::prelude::*;
use states::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();

    app.add_plugins((
        gameplay::plugin,
        loading::plugin,
        splash::plugin,
        title::plugin,
    ));
}
