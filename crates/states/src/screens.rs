use bevy::prelude::*;

/// The game's main screen states.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub enum Screen {
    #[default]
    Splash,
    Title,
    Loading,
    Gameplay,
}

/// The game's main screen states.
#[derive(SubStates, Debug, Hash, PartialEq, Eq, Clone, Default)]
#[source(Screen = Screen::Loading)]
#[states(scoped_entities)]
pub enum LoadingScreen {
    #[default]
    Assets,
    Shaders,
    Level,
}
