use bevy::render::color::Color;

const C_255: f32 = 255.;

/// Create a Color from RGB values.
/// Conveniently divides the values by 255, to get the necessary values between 0 and 1.
pub fn rgb(r: f32, g: f32, b: f32) -> Color {
    Color::rgb(r / C_255, g / C_255, b / C_255)
}

/// Create a Color from RGBA values.
/// Conveniently divides the values by 255.
pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color::rgba(r / C_255, g / C_255, b / C_255, a)
}
