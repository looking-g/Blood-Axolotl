//! Mod that desribes the [`Aabb`] struct and it's functions.

use bevy::math::Vec2;
use bevy::ecs::component::Component;

/// Simple AABB used for this game.
#[derive(Copy, Clone, Component, Default)]
pub struct Aabb {
    /// Top Left
    /// Value is in **local space** in **Bevy units**
    pub lt: Vec2,
    /// Bottom Right
    /// Value is in **local space** in **Bevy units**
    pub rb: Vec2,
}

impl Aabb {
    /// Crates a new [`Aabb`], see struct definition for more info.
    pub fn new(lt: Vec2, rb: Vec2) -> Self {
        Self {
            lt,
            rb,
        }
    }

    /// Returns if this [`Aabb`] is colliding with another
    /// If there is a collision, the overlaps are returned
    pub fn collide(&self, other: &Self) -> bool{
        let self_max_x = self.lt.x.max(self.rb.x);
        let self_max_y = self.lt.y.max(self.rb.y);
        let other_max_x = other.lt.x.max(other.rb.x);
        let other_max_y = other.lt.y.max(other.rb.y);

        let self_min_x = self.lt.x.min(self.rb.x);
        let self_min_y = self.lt.y.min(self.rb.y);
        let other_min_x = other.lt.x.min(other.rb.x);
        let other_min_y = other.lt.y.min(other.rb.y);

        self_max_x > other_min_x &&
        self_min_x < other_max_x &&
        self_max_y > other_min_y &&
        self_min_y < other_max_y
    }

    /// Returns the side with the most collision overlap
    /// returns 0, 0 if there is no overlap
    pub fn collideing_side(&self, other: &Self) -> Vec2 {
        let self_max_x = self.lt.x.max(self.rb.x);
        let self_max_y = self.lt.y.max(self.rb.y);
        let other_max_x = other.lt.x.max(other.rb.x);
        let other_max_y = other.lt.y.max(other.rb.y);

        let self_min_x = self.lt.x.min(self.rb.x);
        let self_min_y = self.lt.y.min(self.rb.y);
        let other_min_x = other.lt.x.min(other.rb.x);
        let other_min_y = other.lt.y.min(other.rb.y);

        let mut out = Vec2::new(
            (self_max_x.min(other_max_x) -
                self_min_x.max(other_min_x)).max(0.0),
            (self_max_y.min(other_max_y) -
                self_min_y.max(other_min_y)).max(0.0),
        );

        if self.center().x < other.center().x{
            out *= -1.0;
        }

        if self.center().y < other.center().y{
            out *= -1.0;
        }

        out

    }

    /// translates the Aabb
    pub fn translate(mut self, pos: Vec2) -> Self{
        self.lt += pos;
        self.rb += pos;

        self
    }

    pub fn center(&self) -> Vec2 { (self.lt+self.rb) * 0.5 }
}

