mod gpu;
mod input;
mod sprite;
pub use sprite::{GPUCamera, GPUSprite};

pub use gpu::WGPU;
mod engine;
pub use engine::Engine;

#[async_trait::async_trait]
pub trait Game {
    async fn init(&mut self, engine: &mut Engine);
    fn update(&mut self, engine: &mut Engine);
}
