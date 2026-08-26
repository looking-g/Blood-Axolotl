//! Mod that desribes the [`Aabb`] struct and it's functions.

use bevy::math::Vec2;
use bevy::ecs::component::Component;

/// Simple AABB used for this game.
#[derive(Copy, Clone, Component, Default)]
pub struct Aabb {
    /// Top Left
    /// Value is in **local space** in **Bevy units**
    tl: Vec2,
    /// Bottom Right
    /// Value is in **local space** in **Bevy units**
    br: Vec2,
}

impl Aabb {
    /// Crates a new [`Aabb`], see struct definition for more info.
    pub fn new(tl: Vec2, br: Vec2) -> Self {
        Self {
            tl,
            br,
        }
    }

    /// Returns if this [`Aabb`] is colliding with another
    pub fn collide(&self, other: &Self) -> bool{
        self.tl.y <= other.br.y &&
        self.br.y >= other.tl.y &&
        self.br.x <= other.tl.x &&
        self.tl.x >= other.br.x
    }
}
