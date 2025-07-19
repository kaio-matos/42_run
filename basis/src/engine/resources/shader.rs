use std::ops::{Deref, DerefMut};

pub use crate::engine::prelude::*;

pub struct RenderShader(graphics::glw::Shader);
impl Resource for RenderShader {}

impl Default for RenderShader {
    fn default() -> Self {
        let shader = glw::Shader::default();
        shader
            .link_multiple(vec![
                glw::ShaderType::Vertex("basis/src/assets/shaders/vertex_perspective_shader.glsl"),
                glw::ShaderType::Fragment(
                    "basis/src/assets/shaders/fragment_perspective_shader.glsl",
                ),
            ])
            .expect("Shader to be found, compiled and linked");
        Self(shader)
    }
}

impl Deref for RenderShader {
    type Target = graphics::glw::Shader;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for RenderShader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
