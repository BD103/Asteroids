use bevy::prelude::*;

pub const WHITE: Color = {
    const BRIGHTNESS: f32 = 5.0;
    Color::rgb_linear(BRIGHTNESS, BRIGHTNESS, BRIGHTNESS)
};

pub const BRIGHT_WHITE: Color = {
    const BRIGHTNESS: f32 = 100.0;
    Color::rgb_linear(BRIGHTNESS, BRIGHTNESS, BRIGHTNESS)
};
