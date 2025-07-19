use std::ops::{Deref, DerefMut};

use crate::prelude::*;

#[derive(Copy, Clone)]
pub struct Deltatime(pub f32);
impl Resource for Deltatime {}
impl Deref for Deltatime {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Deltatime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
