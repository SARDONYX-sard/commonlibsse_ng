use crate::re::Color::Color;

/// Represents a min-max range of values.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaxMin<T> {
    pub max: T, // 0x0
    pub min: T, // variadic
}
const _: () = assert!(size_of::<MaxMin<Color>>() == 0x8);

impl<T> MaxMin<T> {
    /// Creates a new `MaxMin` with specified `max` and `min` values.
    #[inline]
    pub const fn new(max: T, min: T) -> Self {
        Self { max, min }
    }
}

/// Represents the directional ambient lighting colors.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Directional {
    pub x: MaxMin<Color>, // 00
    pub y: MaxMin<Color>, // 08
    pub z: MaxMin<Color>, // 10
}
const _: () = assert!(size_of::<Directional>() == 0x18);

/// Represents the ambient lighting colors with specular and fresnel power.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BGSDirectionalAmbientLightingColors {
    pub directional: Directional, // 00
    pub specular: Color,          // 18
    pub fresnel_power: f32,       // 1C
}
const _: () = assert!(size_of::<BGSDirectionalAmbientLightingColors>() == 0x20);
