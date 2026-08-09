use crate::*;
use service::Service;

mod vulkan_backend;
use vulkan_backend::VulkanBackend;

pub enum Backend {
    Vulkan(VulkanBackend),
}

#[derive(Object)]
pub struct RenderService {
    backend: Backend,

    id: Uuid,
}

impl RenderService {
    pub fn new() -> Self {
        Self {
            backend: Backend::Vulkan(VulkanBackend::new()),
            id: Uuid::new_v4(),
        }
    }
}

impl Service for RenderService {}
