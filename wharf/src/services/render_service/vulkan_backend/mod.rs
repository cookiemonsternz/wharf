use crate::*;

use std::sync::Arc;

// Vulkano
use vulkano::VulkanLibrary;
use vulkano::buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::command_buffer::allocator::{
    StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo,
};
use vulkano::device::physical::PhysicalDevice;
use vulkano::device::{Device, DeviceCreateInfo, Queue, QueueCreateInfo, QueueFlags};
use vulkano::image::{Image, ImageCreateInfo, ImageType, ImageUsage};
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::sync::{self, GpuFuture};

pub struct VulkanContext {
    instance: Arc<Instance>,
    physical_device: Arc<PhysicalDevice>,
    device: Arc<Device>,
    graphics_queue: Arc<Queue>,
    memory_allocator: Arc<StandardMemoryAllocator>,
    command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
}

impl VulkanContext {
    pub fn new() -> Self {
        let instance = Self::create_instance();
        let physical_device = Self::create_physical_device(instance.clone());
        let queue_family_index = Self::get_graphics_queue_family_index(physical_device.clone());
        let (device, graphics_queue) =
            Self::create_device(physical_device.clone(), queue_family_index);
        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
        let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
            device.clone(),
            StandardCommandBufferAllocatorCreateInfo::default(),
        ));

        Self {
            instance,
            physical_device,
            device,
            graphics_queue,
            memory_allocator,
            command_buffer_allocator,
        }
    }

    fn create_instance() -> Arc<Instance> {
        let library = VulkanLibrary::new().expect("No local Vulkan library/dll");
        Instance::new(
            library,
            InstanceCreateInfo {
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
                ..Default::default()
            },
        )
        .expect("Could not create Vulkan instance")
    }

    fn create_physical_device(instance: Arc<Instance>) -> Arc<PhysicalDevice> {
        instance
            .enumerate_physical_devices()
            .expect("Could not enumerate physical devices")
            .next()
            .expect("No physical devices available")
    }

    fn get_graphics_queue_family_index(physical_device: Arc<PhysicalDevice>) -> u32 {
        physical_device
            .queue_family_properties()
            .iter()
            .position(|queue_family_properties| {
                queue_family_properties
                    .queue_flags
                    .contains(QueueFlags::GRAPHICS)
            })
            .expect("Could not find a graphical queue family") as u32
    }

    fn create_device(
        physical_device: Arc<PhysicalDevice>,
        queue_family_index: u32,
    ) -> (Arc<Device>, Arc<Queue>) {
        let (device, mut queues) = Device::new(
            physical_device,
            DeviceCreateInfo {
                // here we pass the desired queue family to use by index
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .expect("failed to create device");

        (device, queues.next().unwrap())
    }
}

#[derive(Object)]
pub struct VulkanBackend {
    context: VulkanContext,

    id: Uuid,
}

impl VulkanBackend {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            context: VulkanContext::new(),
        }
    }

    pub fn context(&self) -> &VulkanContext {
        &self.context
    }

    pub fn create_buffer<T>(
        &self,
        create_info: BufferCreateInfo,
        allocation_info: AllocationCreateInfo,
        data: T,
    ) -> Subbuffer<T>
    where
        T: BufferContents,
    {
        Buffer::from_data(
            self.context().memory_allocator.clone(),
            create_info,
            allocation_info,
            data,
        )
        .expect("Could not create buffer")
    }

    pub fn create_buffer_host<T>(&self, create_info: BufferCreateInfo, data: T) -> Subbuffer<T>
    where
        T: BufferContents,
    {
        Buffer::from_data(
            self.context().memory_allocator.clone(),
            create_info,
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            data,
        )
        .expect("Could not create buffer")
    }

    pub fn create_buffer_device<T>(&self, create_info: BufferCreateInfo, data: T) -> Subbuffer<T>
    where
        T: BufferContents,
    {
        Buffer::from_data(
            self.context().memory_allocator.clone(),
            create_info,
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            data,
        )
        .expect("Could not create buffer")
    }

    pub fn create_image(
        &self,
        create_info: ImageCreateInfo,
        allocation_info: AllocationCreateInfo,
    ) -> Arc<Image> {
        Image::new(
            self.context().memory_allocator.clone(),
            create_info,
            allocation_info,
        )
        .expect("Could not create image")
    }

    pub fn create_image_host(&self, create_info: ImageCreateInfo) -> Arc<Image> {
        Image::new(
            self.context().memory_allocator.clone(),
            create_info,
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
        )
        .expect("Could not create image")
    }

    pub fn create_image_device(&self, create_info: ImageCreateInfo) -> Arc<Image> {
        Image::new(
            self.context().memory_allocator.clone(),
            create_info,
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
        )
        .expect("Could not create image")
    }
}

#[cfg(test)]
mod tests {
    use vulkano::{
        command_buffer::{self, AutoCommandBufferBuilder, CommandBufferUsage, CopyBufferInfo},
        format::Format,
    };

    use super::*;

    #[test]
    fn try_create_instance() {
        VulkanContext::create_instance();
    }

    #[test]
    fn try_create_physical_device() {
        let instance = VulkanContext::create_instance();
        VulkanContext::create_physical_device(instance.clone());
    }

    #[test]
    fn try_get_graphics_queue_family_index() {
        let instance = VulkanContext::create_instance();
        let physical_device = VulkanContext::create_physical_device(instance.clone());
        VulkanContext::get_graphics_queue_family_index(physical_device.clone());
    }

    #[test]
    fn try_create_device() {
        let instance = VulkanContext::create_instance();
        let physical_device = VulkanContext::create_physical_device(instance.clone());
        let queue_family_index =
            VulkanContext::get_graphics_queue_family_index(physical_device.clone());
        let (device, graphics_queue) =
            VulkanContext::create_device(physical_device.clone(), queue_family_index);
    }

    #[test]
    fn try_create_vulkan_context() {
        VulkanContext::new();
    }

    #[test]
    fn try_create_vulkan_backend() {
        VulkanBackend::new();
    }

    #[test]
    fn try_create_buffer() {
        let backend = VulkanBackend::new();
        backend.create_buffer(
            BufferCreateInfo {
                usage: BufferUsage::UNIFORM_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            12 as i32,
        );
    }

    #[test]
    fn try_create_buffer_host() {
        let backend = VulkanBackend::new();
        backend.create_buffer_host(
            BufferCreateInfo {
                usage: BufferUsage::UNIFORM_BUFFER,
                ..Default::default()
            },
            12 as i32,
        );
    }

    #[test]
    fn try_create_buffer_device() {
        let backend = VulkanBackend::new();
        backend.create_buffer_device(
            BufferCreateInfo {
                usage: BufferUsage::UNIFORM_BUFFER,
                ..Default::default()
            },
            12 as i32,
        );
    }

    fn try_create_image() {
        let backend = VulkanBackend::new();
        backend.create_image(
            ImageCreateInfo {
                image_type: ImageType::Dim2d,
                format: Format::R8G8B8A8_UNORM,
                extent: [1024, 1024, 1],
                usage: ImageUsage::TRANSFER_DST | ImageUsage::TRANSFER_SRC,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
                ..Default::default()
            },
        );
    }

    fn try_create_image_host() {
        let backend = VulkanBackend::new();
        backend.create_image_host(ImageCreateInfo {
            image_type: ImageType::Dim2d,
            format: Format::R8G8B8A8_UNORM,
            extent: [1024, 1024, 1],
            usage: ImageUsage::TRANSFER_DST | ImageUsage::TRANSFER_SRC,
            ..Default::default()
        });
    }

    fn try_create_image_device() {
        let backend = VulkanBackend::new();
        backend.create_image_device(ImageCreateInfo {
            image_type: ImageType::Dim2d,
            format: Format::R8G8B8A8_UNORM,
            extent: [1024, 1024, 1],
            usage: ImageUsage::TRANSFER_DST | ImageUsage::TRANSFER_SRC,
            ..Default::default()
        });
    }

    #[test]
    fn try_create_command_buffer_copy() {
        let backend = VulkanBackend::new();
        let src_buffer = Buffer::from_iter(
            backend.context().memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::TRANSFER_SRC,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            0..64,
        )
        .expect("Could not create src buffer");

        let dst_buffer = Buffer::from_iter(
            backend.context().memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::TRANSFER_DST,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_RANDOM_ACCESS,
                ..Default::default()
            },
            (0..64).map(|_| 0),
        )
        .expect("Could not create dst buffer");

        let mut builder = AutoCommandBufferBuilder::primary(
            backend.context().command_buffer_allocator.clone(),
            backend.context().graphics_queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .expect("Could not create builder");

        builder
            .copy_buffer(CopyBufferInfo::buffers(
                src_buffer.clone(),
                dst_buffer.clone(),
            ))
            .unwrap();

        let command_buffer = builder.build().unwrap();

        let future = sync::now(backend.context().device.clone())
            .then_execute(backend.context().graphics_queue.clone(), command_buffer)
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap();

        future.wait(None).unwrap();

        let src_content = src_buffer.read().unwrap();
        let destination_content = dst_buffer.read().unwrap();
        assert_eq!(&*src_content, &*destination_content);
    }
}
