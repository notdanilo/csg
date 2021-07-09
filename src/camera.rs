pub struct Camera {
    framebuffer: gpu::Framebuffer
}

impl Camera {
    pub fn default(context: &gpu::Context) -> Self {
        let framebuffer = gpu::Framebuffer::default(context);
        Self { framebuffer }
    }

    pub fn framebuffer(&self) -> &gpu::Framebuffer {
        &self.framebuffer
    }
}