use std::fmt::Debug;

use crate::prelude::*;

///
/// Camerable are basically entities that may replace the current view matrix used by the shaders
///
pub trait Camerable {
    fn get_view_matrix(&self) -> Mat4;
}
