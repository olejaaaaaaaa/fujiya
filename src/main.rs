#![warn(unused_qualifications)]

use std::{error::Error, ffi::CStr, fs::{read_dir, write, DirEntry, File}, io::Read, mem::offset_of, panic, process::Command};

use ash::vk::{self, AttachmentReference, BufferUsageFlags, Extent2D, FenceCreateFlags, Format, PhysicalDeviceType, PresentModeKHR, PrimitiveTopology, SurfaceFormatKHR, API_VERSION_1_0, API_VERSION_1_3};
use fujiya_sound::enumerate_sound_device;
use winit::raw_window_handle::*;
use log::*;
use winit::{dpi::PhysicalSize, raw_window_handle::HasDisplayHandle};

use fujiya_render::*;
use fujiya_macros::Vertex;

#[repr(C)]
#[derive(Copy, Clone)]
struct Vertex {
    pos: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    fn get_binding_descriptions() -> [vk::VertexInputBindingDescription; 1] {
        [vk::VertexInputBindingDescription {
            binding: 0,
            stride: std::mem::size_of::<Self>() as u32,
            input_rate: vk::VertexInputRate::VERTEX,
        }]
    }

    fn get_attribute_descriptions() -> [vk::VertexInputAttributeDescription; 2] {
        [
            vk::VertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: vk::Format::R32G32B32_SFLOAT,
                offset: offset_of!(Self, pos) as u32,
            },
            vk::VertexInputAttributeDescription {
                binding: 0,
                location: 1,
                format: vk::Format::R32G32B32_SFLOAT,
                offset: offset_of!(Self, color) as u32,
            },
        ]
    }
}

const _VERTICES_DATA: [Vertex; 5] = [
    // Передняя грань (красная)
    Vertex { pos: [-0.5, -0.5,  0.5], color: [1.0, 0.0, 0.0] },
    Vertex { pos: [ 0.5, -0.5,  0.5], color: [1.0, 0.0, 0.0] },
    Vertex { pos: [ 0.5,  0.5,  0.5], color: [1.0, 0.0, 0.0] },
    Vertex { pos: [-0.5,  0.5,  0.5], color: [1.0, 0.0, 0.0] },
    Vertex { pos: [-0.5, -0.5,  0.5], color: [1.0, 0.0, 0.0] },
];


fn draw_frame(
    device: &ash::Device,
    queue: &ash::vk::Queue,
    swapchain: &Swapchain,
    command_pool: &CommandPool,
    buffer: &ash::vk::Buffer,
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

        device.cmd_bind_vertex_buffers(command_buffer, 0, &[*buffer], &[0]);
        device.cmd_draw(command_buffer, _VERTICES_DATA.len() as u32, 1, 0, 0);
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


fn main() -> Result<(), Box<dyn Error>> {

    unsafe { std::env::set_var("RUST_LOG", "DEBUG") };
    env_logger::init();

    let main_loop = winit::event_loop::EventLoop::new().unwrap();
    let winit_window = winit::window::Window::new(&main_loop).unwrap();

    let device = GraphicsDeviceBuilder::new()
        .with_app(|| {
            AppBuilder::new()
                .with_app_name(c"Marion")
                .build()
        })
        .with_window(&winit_window)
        .with_default_instance();

    let window = WindowManagerBuilder::new(winit_window)
        .with_default_surface(&device.state.instance);

    let device = device
        .with_default_phys_dev(&window.state.surface)
        .with_default_queue_family(&window.state.surface)
        .build();

    let window = window
        .with_default_format(&device.phys_dev)
        .with_default_mode(&device.phys_dev)
        .with_default_swapchain(&device.instance, &device.device);

    // let pipeline = StandartGraphicsPipelineBuilder::new()
    //     .with_fragment_shader(load_from_file(r"C:\Users\Oleja\Desktop\d\fujiya\shared\shaders\spv\ray-tracing-frag.spv"))
    //     .with_vertex_shader(load_from_file(r"C:\Users\Oleja\Desktop\d\fujiya\shared\shaders\spv\triangle-vert.spv"))
    //     .with_uniforms()
    //     .build();


    let mut data = _VERTICES_DATA.to_vec();

    //-----------------------------

    // let command_pool = CommandPoolBuilder::new()
    //     .device(&device.raw)
    //     .family_index(0)
    //     .build();

    // let present_images = swapchain.get_swapchain_images();
    // info!("Количество изображений в Swapchain: {}", present_images.len());

    // let image_views = ImageViewBuilder::new()
    //     .with_device(&device.raw)
    //     .with_format(format)
    //     .with_image_views(&present_images)
    //     .build();

    // info!("Количество Image Views на буфер: {}", image_views.raw.len());
    // let shader = ShaderProgramBuilder::new()
    //     .with_device(&device.raw)
    //     .with_fragment_shader(load_from_file(r"C:\Users\Oleja\Desktop\d\fujiya\shared\shaders\spv\ray-tracing-frag.spv"))
    //     .with_vertex_shader(load_from_file(r"C:\Users\Oleja\Desktop\d\fujiya\shared\shaders\spv\triangle-vert.spv"))
    //     .build();

    // let subpass = SubpassBuilder::new()
    //     .add_color_attachment_ref(
    //         AttachmentReference::default()
    //             .attachment(0)
    //             .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
    //     )
    //     .with_bind_point(vk::PipelineBindPoint::GRAPHICS)
    //     .build();

    // let render_pass = RenderPassBuilder::new()
    //     .with_device(&device.raw)
    //     .add_subpass(subpass.raw)
    //     .add_subpass_dependency(
    //         vk::SubpassDependency {
    //             src_subpass: vk::SUBPASS_EXTERNAL,
    //             src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
    //             dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_READ | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
    //             dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
    //             ..Default::default()
    //         })
    //     .add_attachments_desc(vk::AttachmentDescription {
    //             format: format,
    //             samples: vk::SampleCountFlags::TYPE_1,
    //             load_op: vk::AttachmentLoadOp::CLEAR,
    //             store_op: vk::AttachmentStoreOp::STORE,
    //             final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
    //             ..Default::default()
    //         })
    //     .build();

    // let buffer = GPUBuffer::new(
    //     &device.raw,
    //     &phys_dev.phys_info.memory_prop,
    //     (size_of::<Vertex>() * _VERTICES_DATA.len()) as u64,
    //     BufferUsageFlags::VERTEX_BUFFER,
    //     vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT
    // ).unwrap();

    // let binding_description = Vertex::get_binding_descriptions();
    // let attribute_description = Vertex::get_attribute_descriptions();

    // let vertex_input_state_info = vk::PipelineVertexInputStateCreateInfo::default()
    //     .vertex_attribute_descriptions(&attribute_description)
    //     .vertex_binding_descriptions(&binding_description);

    // let pipeline = RenderPipelineBuilder::new()
    //     .with_vertex_shader(shader.vertex_shader)
    //     .with_fragment_shader(shader.fragment_shader)
    //     .with_resolution(extent)
    //     .with_format(format)
    //     .with_vertex_input_info(vertex_input_state_info)
    //     .with_input_assembly_info(
    //         vk::PipelineInputAssemblyStateCreateInfo::default()
    //                     .topology(PrimitiveTopology::TRIANGLE_STRIP)
    //                     .primitive_restart_enable(false)
    //     )
    //     .with_render_pass(&render_pass.raw)
    //     .with_device(&device.raw)
    //     .build();

    // let framebuffers = FrameBufferBuilder::new()
    //     .device(&device.raw)
    //     .image_views(&image_views.raw)
    //     .resolution(caps.current_extent)
    //     .render_pass(&render_pass.raw)
    //     .build();

    // info!("Количество FrameBuffers: {}", framebuffers.frame_buffers.len());

    // //enumerate_sound_device();

    // let (image_available_semaphore, render_finished_semaphore) = create_semaphores(&device.raw);
    // let fence_info = vk::FenceCreateInfo::default()
    //     .flags(FenceCreateFlags::SIGNALED);

    // let fence = unsafe { device.raw.create_fence(&fence_info, None).unwrap() };

    let _ = main_loop.run(|ev, ev_window| {
    match ev {
        winit::event::Event::WindowEvent { window_id: _, event } => match event {
            winit::event::WindowEvent::KeyboardInput { event, .. } => {
                match event {
                    _ => {}
                }
            },
            winit::event::WindowEvent::CloseRequested => ev_window.exit(),
            winit::event::WindowEvent::RedrawRequested => {

                // for i in &mut data {
                //     i.pos[0] += 0.001;
                //     i.pos[1] -= 0.001;
                // }

                // buffer.upload_data(&device.raw, &data);

                // let _ = draw_frame(
                //     &device.raw,
                //     &queue.raw[0][0],
                //     &swapchain,
                //     &command_pool,
                //     &buffer.buffer,
                //     &pipeline.raw,
                //     &render_pass,
                //     &framebuffers.frame_buffers,
                //     image_available_semaphore,
                //     render_finished_semaphore,
                //     fence
                // );
            }
            winit::event::WindowEvent::Resized(_) => {

            }
            _ => {}
        },
        winit::event::Event::AboutToWait => {
            //window.request_redraw();
        }
        _ => {}
    }
    });

    Ok(())
}