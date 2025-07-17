use std::fmt::Debug;

use crate::math::prelude::*;

///
/// Camerable are basically entities that may replace the current view matrix used by the shaders
///
pub trait Camerable: Debug {
    fn get_view_matrix(&self) -> Mat4;
}
