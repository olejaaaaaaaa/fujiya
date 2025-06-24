#![warn(unused_qualifications)]

use std::{error::Error, fs::{read_dir, write, DirEntry, File}, io::Read, process::Command};

mod instance;
use ash::vk::{AttachmentReference, CommandBufferLevel, Extent2D, Fence, FenceCreateFlags, PipelineBindPoint, PresentModeKHR, PrimitiveTopology, SurfaceFormatKHR, SurfaceTransformFlagsKHR, API_VERSION_1_2, API_VERSION_1_3};
use instance::*;

mod app;
use app::*;

mod utils;
use utils::*;

mod phys_device;
use phys_device::*;

mod surface;
use surface::*;

mod queue;
use queue::*;

mod device;
use device::*;
use winit::dpi::PhysicalSize;

mod swapchain;
use swapchain::*;

mod command_pool;
use command_pool::*;

mod image_views;
use image_views::*;

mod shaders;
use shaders::*;

mod pipeline;
use pipeline::*;

mod render_pass;
use render_pass::*;

mod frame_buffers;
use frame_buffers::*;

mod sync;
use sync::*;

use ash::vk;

struct Vertex {
    pos: [f32; 3],
    color: [f32; 3]
}

struct FrameResources {
    command_buffer: vk::CommandBuffer,
    image_available: vk::Semaphore,
    render_finished: vk::Semaphore,
    in_flight: vk::Fence,
}



fn draw_frame(
    device: &ash::Device,
    queue: &ash::vk::Queue,
    swapchain: &Swapchain,
    command_pool: &CommandPool,
    pipeline: &vk::Pipeline,
    render_pass: &RenderPass,
    framebuffers: &[vk::Framebuffer],
    image_available_semaphore: vk::Semaphore,
    render_finished_semaphore: vk::Semaphore,
    in_flight_fence: vk::Fence,
) -> Result<(), vk::Result> {

    unsafe {
        // Ждём завершения предыдущего кадра
        device.wait_for_fences(&[in_flight_fence], true, u64::MAX)?;
        device.reset_fences(&[in_flight_fence])?;
    }

    // Получаем индекс изображения swapchain
    let (image_index, _) = unsafe {
        swapchain.swapchain_load.acquire_next_image(
            swapchain.swapchain,
            u64::MAX,
            image_available_semaphore,
            vk::Fence::null(),
        )
    }?;

    //info!("{}", image_index);

    // Записываем команды
    let command_buffer = {
        let allocate_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(command_pool.raw)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);

        let buffers = unsafe { device.allocate_command_buffers(&allocate_info) }?;
        buffers[0]
    };

    // Начинаем запись команд
    unsafe {
        device.begin_command_buffer(
            command_buffer,
            &vk::CommandBufferBeginInfo::default()
                .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT),
        )?;
    }

    // Очистка и рендер-пасс
    let clear_values = [vk::ClearValue {
        color: vk::ClearColorValue {
            float32: [0.0, 0.0, 0.0, 1.0],
        },
    }];

    let render_pass_begin_info = vk::RenderPassBeginInfo::default()
        .render_pass(render_pass.raw)
        .framebuffer(framebuffers[image_index as usize])
        .render_area(vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: Extent2D { width: 640, height: 480 },
        })
        .clear_values(&clear_values);

    unsafe {
        device.cmd_begin_render_pass(
            command_buffer,
            &render_pass_begin_info,
            vk::SubpassContents::INLINE,
        );

        device.cmd_bind_pipeline(
            command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            *pipeline,
        );

        device.cmd_draw(command_buffer, 6, 1, 0, 0);
        device.cmd_end_render_pass(command_buffer);
        device.end_command_buffer(command_buffer)?;
    }

    // Отправляем в очередь
    let binding_sema = [image_available_semaphore];
    let binding_biffers = [command_buffer];
    let binding = [render_finished_semaphore];

    let submit_info = vk::SubmitInfo::default()
        .wait_semaphores(&binding_sema)
        .wait_dst_stage_mask(&[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT])
        .command_buffers(&binding_biffers)
        .signal_semaphores(&binding);

    unsafe {
        device.queue_submit(
            *queue,
            &[submit_info],
            in_flight_fence,
        )?;
    }

    // Представляем изображение
    let binding1 = [render_finished_semaphore];
    let binding2 = [swapchain.swapchain];
    let binding3 = [image_index];

    let present_info = vk::PresentInfoKHR::default()
        .wait_semaphores(&binding1)
        .swapchains(&binding2)
        .image_indices(&binding3);

    unsafe {
        swapchain.swapchain_load.queue_present(*queue, &present_info)?;
        device.device_wait_idle()?;
    }

    Ok(())
}

use log::*;

fn main() -> Result<(), Box<dyn Error>> {

    unsafe { std::env::set_var("RUST_LOG", "DEBUG") };
    env_logger::init();

    let main_loop = winit::event_loop::EventLoop::new().unwrap();
    let window = winit::window::WindowBuilder::new()
        .with_title("Game")
        .with_inner_size(PhysicalSize::new(640, 480))
        .build(&main_loop)
        .unwrap();

    let app = AppBuilder::new()
        .with_app_name(c"Bomb")
        .with_app_verison(000_000_002)
        .with_engine_name(c"Fujiya")
        .with_engine_verison(24_06_2025)
        .with_api_version(API_VERSION_1_3)
        .build();

    let instance = InstanceBuilder::new()
        .add_extensions(vec![
            c"VK_EXT_debug_utils",
            c"VK_EXT_debug_report",
            c"VK_KHR_win32_surface",
            c"VK_KHR_surface"
        ])
        .add_layer(c"VK_LAYER_KHRONOS_validation")
        .with_app_info(&app.raw)
        .build();

    let phys_dev = PhysicalDeviceBuilder::new()
        .with_insatnce(&instance.raw)
        .build();

    let surface = SurfaceBuilder::new()
        .with_entry(&instance.raw_entry)
        .with_instance(&instance.raw)
        .with_window(&window)
        .build();

    let phys_ddev = &phys_dev.raw;

    let queue_family = QueuesFamilyBuilder::new()
        .with_queue_family_prop(&phys_dev.phys_info.queue_family_prop)
        .with_surface(&surface.raw)
        .with_surface_load(&surface.raw_load)
        .with_phys_dev(phys_ddev)
        .build();

    let device = DeviceBuilder::new()
        .add_extension(c"VK_KHR_swapchain")
        .queue_family(&queue_family)
        .with_instance(&instance.raw)
        .with_phys_dev(&phys_dev.raw)
        .build();

    let queue = UniversalQueue::new(&device.raw, queue_family);

    let formats = surface.get_surface_formats(phys_ddev);
    let caps = surface.get_surface_capabilities(phys_ddev);
    let present_modes = surface.get_surface_present_modes(phys_ddev);

    let format = formats[0].format;
    let color_space = formats[0].color_space;
    let extent = caps.current_extent;

    info!("Текущее разрешение: {}x{}", caps.current_extent.width, caps.current_extent.height);
    info!("Цветовой формат: {:?} Цветовое пространство: {:?}", format, color_space);
    info!("Поддержка VSync: {:?}", present_modes.contains(&PresentModeKHR::FIFO));

    let swapchain = SwapchainBuilder::new()
        .with_color_space(color_space)
        .with_format(format)
        .with_resolution(extent)
        .with_transform(caps.supported_transforms)
        .with_present_mode(PresentModeKHR::FIFO)
        .with_instance(&instance.raw)
        .with_device(&device.raw)
        .with_surface(&surface.raw)
        .build();

    let command_pool = CommandPoolBuilder::new()
        .device(&device.raw)
        .family_index(0)
        .build();

    let buffer = command_pool.create_buffers(&device.raw, 1, CommandBufferLevel::PRIMARY)[0];

    let present_images = swapchain.get_swapchain_images();
    info!("Количество изображений в Swapchain: {}", present_images.len());

    let image_views = ImageViewBuilder::new()
        .with_device(&device.raw)
        .with_format(format)
        .with_image_views(&present_images)
        .build();

    info!("Количество Image Views на буфер: {}", image_views.raw.len());

    let shader = ShaderBuilder::new()
        .with_device(&device.raw)
        .with_fragment_shader_source("./shaders/spv/ray-tracing-frag.spv")
        .with_vertex_shader_source("./shaders/spv/triangle-vert.spv")
        .build();

    let subpass = SubpassBuilder::new()
        .add_color_attachment_ref(
            AttachmentReference::default()
                .attachment(0)
                .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        )
        .with_bind_point(vk::PipelineBindPoint::GRAPHICS)
        .build();

    let attachment_desc = AttachmentDescription::default(format);
    let subpass_deps = SubpassDependency::default();

    let render_pass = RenderPassBuilder::new()
        .with_device(&device.raw)
        .add_subpass(subpass.raw)
        .add_subpass_dependency(subpass_deps.raw)
        .add_attachments_desc(attachment_desc.raw)
        .build();

    let pipeline = RenderPipelineBuilder::new()
        .vertex_shader(shader.vertex_shader)
        .fragment_shader(shader.fragment_shader)
        .resolution(extent)
        .format(format)
        .input_assembly_info(PrimitiveTopology::TRIANGLE_LIST)
        .render_pass(&render_pass.raw)
        .device(&device.raw)
        .build();

    let framebuffers = FrameBufferBuilder::new()
        .device(&device.raw)
        .image_views(&image_views.raw)
        .resolution(caps.current_extent)
        .render_pass(&render_pass.raw)
        .build();

    info!("Буферизация: {}", framebuffers.frame_buffers.len());

    let (image_available_semaphore, render_finished_semaphore) = create_semaphores(&device.raw);
    let fence_info = vk::FenceCreateInfo::default()
        .flags(FenceCreateFlags::SIGNALED);

    let fence = unsafe { device.raw.create_fence(&fence_info, None).unwrap() };

    let _ = main_loop.run(|ev, ev_window| {
    match ev {
        winit::event::Event::WindowEvent { window_id: _, event } => match event {
            winit::event::WindowEvent::CloseRequested => ev_window.exit(),
            winit::event::WindowEvent::RedrawRequested => {
                    let _ = draw_frame(
                        &device.raw,
                        &queue.raw[0][0],
                        &swapchain,
                        &command_pool,
                        &pipeline.raw,
                        &render_pass,
                        &framebuffers.frame_buffers,
                        image_available_semaphore,
                        render_finished_semaphore,
                        fence
                    );
            }
            winit::event::WindowEvent::Resized(_) => {
                // Обработка изменения размера
            }
            _ => {}
        },
        winit::event::Event::AboutToWait => {
            window.request_redraw();
        }
        _ => {}
    }
    });

    Ok(())
}