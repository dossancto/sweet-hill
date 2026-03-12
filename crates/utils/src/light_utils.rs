use bevy::prelude::*;
use states::world::PostPhysicsAppSystems;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Flicker {
    pub base_intensity: f32,
    pub frequency: f32,
    pub percentage: f32,
}

impl Default for Flicker {
    fn default() -> Self {
        Flicker {
            base_intensity: 1.0,
            frequency: 20.0,
            percentage: 0.1,
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (flicker_light_pointlight, flicker_light_spotlight)
            .run_if(in_state(states::screens::Screen::Gameplay))
            .in_set(PostPhysicsAppSystems::Update),
    );
}

pub fn flicker_light_pointlight(
    time: Res<Time>,
    mut query: Query<(&mut PointLight, &Flicker), With<Flicker>>,
) {
    for (mut light, flicker_config) in &mut query {
        light.intensity = get_flicker_intensity(&time, flicker_config);
    }
}

pub fn flicker_light_spotlight(
    time: Res<Time>,
    mut query: Query<(&mut SpotLight, &Flicker), With<Flicker>>,
) {
    for (mut light, flicker_config) in &mut query {
        light.intensity = get_flicker_intensity(&time, flicker_config);
    }
}

fn get_flicker_intensity(time: &Res<Time>, flicker: &Flicker) -> f32 {
    let flickers_per_second = flicker.frequency;

    let flicker_percentage = flicker.percentage;

    let flicks = (time.elapsed_secs() * flickers_per_second).sin();

    let base_intensity = flicker.base_intensity;

    base_intensity + flicks * base_intensity * flicker_percentage
}
