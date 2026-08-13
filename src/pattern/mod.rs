pub mod solid;
pub mod stripes;

use std::fmt::Debug;

use ray_math::Vector4;
use ray_traits::AsAny;

use crate::color::Color;

pub trait Pattern: AsAny + Debug {
    fn pattern_eq(&self, other: &dyn Pattern) -> bool;
    fn color_at_point(&self, point: Vector4) -> Color;
}

impl PartialEq for dyn Pattern {
    fn eq(&self, other: &Self) -> bool {
        self.pattern_eq(other)
    }
}

/// Default implementation for `Pattern::pattern_eq`.
/// Reduces boilerplate in concrete shape implementations.
pub fn default_pattern_eq<T: PartialEq + 'static>(this: &T, other: &dyn Pattern) -> bool {
    other.as_any().downcast_ref::<T>() == Some(this)
}
