use raylib::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct Light {
    pub position: Vector3,
    pub target: Vector3,
    pub color: raylib::color::Color,

    pub position_loc: i32,
    pub target_loc: i32,
    pub color_loc: i32,
}

impl Light  {
    pub fn new(positon: Vector3, targ: Vector3, color: Color, shader: &mut WeakShader) -> Self {
        let mut light = Self {
            position: positon.clone(),
            target: targ.clone(),
            color: color.clone(),

            position_loc: shader.get_shader_location("light.position"),
            target_loc: shader.get_shader_location("light.target"),
            color_loc: shader.get_shader_location("light.color"),
        };
        light.update_light_values(shader);
        light
    }
    pub fn update_light_values(&mut self, shader: &mut WeakShader) {
        shader.set_shader_value(self.position_loc, self.position);
        shader.set_shader_value(self.target_loc, self.target);
        let color: Vector4 = self.color.into();
        shader.set_shader_value(self.color_loc, color);
    }
}
