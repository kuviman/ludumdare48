use super::*;

pub struct Camera {
    pub center: Vec2<f32>,
    pub fov: f32,
    pub target_fov: f32,
}

impl Camera {
    pub fn new(fov: f32) -> Self {
        Self {
            center: vec2(0.0, 0.0),
            fov,
            target_fov: fov,
        }
    }
    pub fn update(&mut self, delta_time: f32) {
        self.fov += (self.target_fov - self.fov) * delta_time.min(1.0);
    }
    fn view_matrix(&self) -> Mat4<f32> {
        Mat4::scale_uniform(1.0 / self.fov) * Mat4::translate(-self.center.extend(0.0))
    }
    fn projection_matrix(&self, framebuffer_size: Vec2<f32>) -> Mat4<f32> {
        Mat4::scale(vec3(
            2.0 * framebuffer_size.y / framebuffer_size.x,
            2.0,
            1.0,
        ))
    }
    pub fn uniforms(&self, framebuffer_size: Vec2<f32>) -> impl ugli::Uniforms {
        ugli::uniforms! {
            u_projection_matrix: self.projection_matrix(framebuffer_size),
            u_view_matrix: self.view_matrix(),
        }
    }
    pub fn world_to_screen(&self, framebuffer_size: Vec2<f32>, pos: Vec2<f32>) -> Vec2<f32> {
        let pos = (self.projection_matrix(framebuffer_size) * self.view_matrix())
            * pos.extend(0.0).extend(1.0);
        vec2(
            (pos.x + 1.0) / 2.0 * framebuffer_size.x,
            (pos.y + 1.0) / 2.0 * framebuffer_size.y,
        )
    }
    pub fn screen_to_world(&self, framebuffer_size: Vec2<f32>, pos: Vec2<f32>) -> Vec2<f32> {
        let pos = vec2(
            pos.x / framebuffer_size.x * 2.0 - 1.0,
            pos.y / framebuffer_size.y * 2.0 - 1.0,
        );
        let pos = (self.projection_matrix(framebuffer_size) * self.view_matrix()).inverse()
            * pos.extend(0.0).extend(1.0);
        pos.xy()
    }
}
