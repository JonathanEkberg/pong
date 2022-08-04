use bevy::prelude::*;

pub fn get_screen_dimensions(windows: &Res<Windows>) -> (f32, f32) {
    let (mut width, mut height) = (0., 0.);

    for window in windows.iter() {
        (width, height) = (window.width(), window.height());
    }

    (width, height)
}

pub fn lerp(current: f32, target: f32, step: f32) -> f32 {
    current + step * (target - current)
}
