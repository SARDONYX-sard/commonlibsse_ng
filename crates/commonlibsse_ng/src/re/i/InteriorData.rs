use crate::re::{
    BGSDirectionalAmbientLightingColors::BGSDirectionalAmbientLightingColors, Color::Color,
};

bitflags::bitflags! {
    /// Bitflags representing inheritance settings for interior lighting data.
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Inherit: u32 {
        const AMBIENT_COLOR         = 1 << 0;
        const DIRECTIONAL_COLOR     = 1 << 1;
        const FOG_COLOR             = 1 << 2;
        const FOG_NEAR              = 1 << 3;
        const FOG_FAR               = 1 << 4;
        const DIRECTIONAL_ROTATION  = 1 << 5;
        const DIRECTIONAL_FADE      = 1 << 6;
        const CLIP_DISTANCE         = 1 << 7;
        const FOG_POWER             = 1 << 8;
        const FOG_MAX               = 1 << 9;
        const LIGHT_FADE_DISTANCES  = 1 << 10;
    }
}

/// Represents interior lighting data.
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct INTERIOR_DATA {
    pub ambient: Color,                                                           // 00
    pub directional: Color,                                                       // 04
    pub fog_color_near: Color,                                                    // 08
    pub fog_near: f32,                                                            // 0C
    pub fog_far: f32,                                                             // 10
    pub directional_xy: u32,                                                      // 14
    pub directional_z: u32,                                                       // 18
    pub directional_fade: f32,                                                    // 1C
    pub clip_dist: f32,                                                           // 20
    pub fog_power: f32,                                                           // 24
    pub directional_ambient_lighting_colors: BGSDirectionalAmbientLightingColors, // 28
    pub fog_color_far: Color,                                                     // 48
    pub fog_clamp: f32,                                                           // 4C
    pub light_fade_start: f32,                                                    // 50
    pub light_fade_end: f32,                                                      // 54
    pub lighting_template_inheritance_flags: Inherit,                             // 58
    pub unk5c: u32, // 5C - interiorOffset?
}

const _: () = {
    assert!(core::mem::size_of::<INTERIOR_DATA>() == 0x60);
};
