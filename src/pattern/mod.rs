pub mod solid;
pub mod stripes;

use std::any::Any;
use std::fmt::Debug;

use ray_math::Vector4;

use crate::color::Color;

pub trait Pattern: Debug {
    fn as_any(&self) -> &dyn Any;
    fn pattern_eq(&self, other: &dyn Pattern) -> bool;
    fn color_at_point(&self, point: Vector4) -> Color;
}

impl PartialEq for dyn Pattern {
    fn eq(&self, other: &Self) -> bool {
        self.pattern_eq(other)
    }
}
