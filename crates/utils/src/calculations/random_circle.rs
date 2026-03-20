use std::f32::consts::PI;

use rand::Rng;

pub fn get_non_uniform_random_point_on_circle(radius: f32) -> (f32, f32) {
    let distance = get_random_distance(radius);

    let theta = get_random_angle();

    convert_to_xy_coordinates(distance, theta)
}

fn convert_to_xy_coordinates(distance: f32, theta: f32) -> (f32, f32) {
    let x = distance * theta.cos();

    let y = distance * theta.sin();

    (x, y)
}

fn get_random_distance(distance: f32) -> f32 {
    let mut rng = rand::rng();

    rng.random_range(-distance..distance)
}

fn get_random_angle() -> f32 {
    let mut rng = rand::rng();

    let two_pi = 2. * PI;

    let theta = rng.random_range(0f32..two_pi);

    theta
}
