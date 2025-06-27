#![warn(unused_qualifications)]

use std::{error::Error, ffi::CStr, fs::{read_dir, write, DirEntry, File}, io::Read, mem::offset_of, panic, process::Command};

// mod instance;
// mod core;
// use ash::{util::Align, vk::{AttachmentReference, BufferCreateInfo, CommandBufferLevel, Extent2D, Fence, FenceCreateFlags, Format, MemoryAllocateInfo, MemoryPropertyFlags, PipelineBindPoint, PresentModeKHR, PrimitiveTopology, SurfaceFormatKHR, SurfaceTransformFlagsKHR, API_VERSION_1_2, API_VERSION_1_3}};
// use instance::*;

// mod app;
// use app::*;

// mod utils;
// use utils::*;

// mod phys_device;
// use phys_device::*;

// mod surface;
// use surface::*;

// mod queue;
// use queue::*;

// mod device;
// use device::*;
// use winit::{dpi::PhysicalSize, monitor::VideoMode, raw_window_handle::HasDisplayHandle};

// mod swapchain;
// use swapchain::*;

// mod command_pool;
// use command_pool::*;

// mod image_views;
// use image_views::*;

// mod shaders;
// use shaders::*;

// mod pipeline;
// use pipeline::*;

// mod render_pass;
// use render_pass::*;

// mod frame_buffers;
// use frame_buffers::*;

// mod sync;
// use sync::*;

use ash::vk::{self, API_VERSION_1_0};
mod device;
use device::*;

// #[repr(C)]
// #[derive(Copy, Clone)]
// struct Vertex {
//     pos: [f32; 3],
//     color: [f32; 3],
// }

// impl Vertex {
//     fn get_binding_descriptions() -> [vk::VertexInputBindingDescription; 1] {
//         [vk::VertexInputBindingDescription {
//             binding: 0,
//             stride: std::mem::size_of::<Self>() as u32,
//             input_rate: vk::VertexInputRate::VERTEX,
//         }]
//     }

//     fn get_attribute_descriptions() -> [vk::VertexInputAttributeDescription; 2] {
//         [
//             vk::VertexInputAttributeDescription {
//                 location: 0,
//                 binding: 0,
//                 format: vk::Format::R32G32_SFLOAT,
//                 offset: offset_of!(Self, pos) as u32,
//             },
//             vk::VertexInputAttributeDescription {
//                 binding: 0,
//                 location: 1,
//                 format: vk::Format::R32G32B32_SFLOAT,
//                 offset: offset_of!(Self, color) as u32,
//             },
//         ]
//     }
// }

// const _VERTICES_DATA: [Vertex; 36] = [
//     // Передняя грань (красная)
//     Vertex { pos: [-0.5, -0.5,  0.5], color: [1.0, 0.0, 0.0] },
//     Vertex { pos: [ 0.5, -0.5,  0.5], color: [1.0, 0.0, 0.0] },
//     Vertex { pos: [ 0.5,  0.5,  0.5], color: [1.0, 0.0, 0.0] },
//     Vertex { pos: [ 0.5,  0.5,  0.5], color: [1.0, 0.0, 0.0] },
//     Vertex { pos: [-0.5,  0.5,  0.5], color: [1.0, 0.0, 0.0] },
//     Vertex { pos: [-0.5, -0.5,  0.5], color: [1.0, 0.0, 0.0] },
    
//     // Задняя грань (зелёная)
//     Vertex { pos: [-0.5, -0.5, -0.5], color: [0.0, 1.0, 0.0] },
//     Vertex { pos: [ 0.5, -0.5, -0.5], color: [0.0, 1.0, 0.0] },
//     Vertex { pos: [ 0.5,  0.5, -0.5], color: [0.0, 1.0, 0.0] },
//     Vertex { pos: [ 0.5,  0.5, -0.5], color: [0.0, 1.0, 0.0] },
//     Vertex { pos: [-0.5,  0.5, -0.5], color: [0.0, 1.0, 0.0] },
//     Vertex { pos: [-0.5, -0.5, -0.5], color: [0.0, 1.0, 0.0] },
    
//     // Верхняя грань (синяя)
//     Vertex { pos: [-0.5,  0.5, -0.5], color: [0.0, 0.0, 1.0] },
//     Vertex { pos: [ 0.5,  0.5, -0.5], color: [0.0, 0.0, 1.0] },
//     Vertex { pos: [ 0.5,  0.5,  0.5], color: [0.0, 0.0, 1.0] },
//     Vertex { pos: [ 0.5,  0.5,  0.5], color: [0.0, 0.0, 1.0] },
//     Vertex { pos: [-0.5,  0.5,  0.5], color: [0.0, 0.0, 1.0] },
//     Vertex { pos: [-0.5,  0.5, -0.5], color: [0.0, 0.0, 1.0] },
    
//     // Нижняя грань (жёлтая)
//     Vertex { pos: [-0.5, -0.5, -0.5], color: [1.0, 1.0, 0.0] },
//     Vertex { pos: [ 0.5, -0.5, -0.5], color: [1.0, 1.0, 0.0] },
//     Vertex { pos: [ 0.5, -0.5,  0.5], color: [1.0, 1.0, 0.0] },
//     Vertex { pos: [ 0.5, -0.5,  0.5], color: [1.0, 1.0, 0.0] },
//     Vertex { pos: [-0.5, -0.5,  0.5], color: [1.0, 1.0, 0.0] },
//     Vertex { pos: [-0.5, -0.5, -0.5], color: [1.0, 1.0, 0.0] },
    
//     // Левая грань (пурпурная)
//     Vertex { pos: [-0.5, -0.5, -0.5], color: [1.0, 0.0, 1.0] },
//     Vertex { pos: [-0.5,  0.5, -0.5], color: [1.0, 0.0, 1.0] },
//     Vertex { pos: [-0.5,  0.5,  0.5], color: [1.0, 0.0, 1.0] },
//     Vertex { pos: [-0.5,  0.5,  0.5], color: [1.0, 0.0, 1.0] },
//     Vertex { pos: [-0.5, -0.5,  0.5], color: [1.0, 0.0, 1.0] },
//     Vertex { pos: [-0.5, -0.5, -0.5], color: [1.0, 0.0, 1.0] },
    
//     // Правая грань (голубая)
//     Vertex { pos: [ 0.5, -0.5, -0.5], color: [0.0, 1.0, 1.0] },
//     Vertex { pos: [ 0.5,  0.5, -0.5], color: [0.0, 1.0, 1.0] },
//     Vertex { pos: [ 0.5,  0.5,  0.5], color: [0.0, 1.0, 1.0] },
//     Vertex { pos: [ 0.5,  0.5,  0.5], color: [0.0, 1.0, 1.0] },
//     Vertex { pos: [ 0.5, -0.5,  0.5], color: [0.0, 1.0, 1.0] },
//     Vertex { pos: [ 0.5, -0.5, -0.5], color: [0.0, 1.0, 1.0] },
// ];




// fn draw_frame(
//     device: &ash::Device,
//     queue: &ash::vk::Queue,
//     swapchain: &Swapchain,
//     command_pool: &CommandPool,
//     buffer: &ash::vk::Buffer,
//     pipeline: &vk::Pipeline,
//     render_pass: &RenderPass,
//     framebuffers: &[vk::Framebuffer],
//     image_available_semaphore: vk::Semaphore,
//     render_finished_semaphore: vk::Semaphore,
//     in_flight_fence: vk::Fence,
// ) -> Result<(), vk::Result> {

//     unsafe {
//         // Ждём завершения предыдущего кадра
//         device.wait_for_fences(&[in_flight_fence], true, u64::MAX)?;
//         device.reset_fences(&[in_flight_fence])?;
//     }

//     // Получаем индекс изображения swapchain
//     let (image_index, _) = unsafe {
//         swapchain.swapchain_load.acquire_next_image(
//             swapchain.swapchain,
//             u64::MAX,
//             image_available_semaphore,
//             vk::Fence::null(),
//         )
//     }?;

//     //info!("{}", image_index);

//     // Записываем команды
//     let command_buffer = {
//         let allocate_info = vk::CommandBufferAllocateInfo::default()
//             .command_pool(command_pool.raw)
//             .level(vk::CommandBufferLevel::PRIMARY)
//             .command_buffer_count(1);

//         let buffers = unsafe { device.allocate_command_buffers(&allocate_info) }?;
//         buffers[0]
//     };

//     // Начинаем запись команд
//     unsafe {
//         device.begin_command_buffer(
//             command_buffer,
//             &vk::CommandBufferBeginInfo::default()
//                 .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT),
//         )?;
//     }

//     // Очистка и рендер-пасс
//     let clear_values = [vk::ClearValue {
//         color: vk::ClearColorValue {
//             float32: [0.0, 0.0, 0.0, 1.0],
//         },
//     }];

//     let render_pass_begin_info = vk::RenderPassBeginInfo::default()
//         .render_pass(render_pass.raw)
//         .framebuffer(framebuffers[image_index as usize])
//         .render_area(vk::Rect2D {
//             offset: vk::Offset2D { x: 0, y: 0 },
//             extent: Extent2D { width: 640, height: 480 },
//         })
//         .clear_values(&clear_values);

//     unsafe {
//         device.cmd_begin_render_pass(
//             command_buffer,
//             &render_pass_begin_info,
//             vk::SubpassContents::INLINE,
//         );

//         device.cmd_bind_pipeline(
//             command_buffer,
//             vk::PipelineBindPoint::GRAPHICS,
//             *pipeline,
//         );

//         device.cmd_bind_vertex_buffers(command_buffer, 0, &[*buffer], &[0]);
//         device.cmd_draw(command_buffer, _VERTICES_DATA.len() as u32, 1, 0, 0);
//         device.cmd_end_render_pass(command_buffer);
//         device.end_command_buffer(command_buffer)?;
//     }

//     // Отправляем в очередь
//     let binding_sema = [image_available_semaphore];
//     let binding_biffers = [command_buffer];
//     let binding = [render_finished_semaphore];

//     let submit_info = vk::SubmitInfo::default()
//         .wait_semaphores(&binding_sema)
//         .wait_dst_stage_mask(&[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT])
//         .command_buffers(&binding_biffers)
//         .signal_semaphores(&binding);

//     unsafe {
//         device.queue_submit(
//             *queue,
//             &[submit_info],
//             in_flight_fence,
//         )?;
//     }

//     // Представляем изображение
//     let binding1 = [render_finished_semaphore];
//     let binding2 = [swapchain.swapchain];
//     let binding3 = [image_index];

//     let present_info = vk::PresentInfoKHR::default()
//         .wait_semaphores(&binding1)
//         .swapchains(&binding2)
//         .image_indices(&binding3);

//     unsafe {
//         swapchain.swapchain_load.queue_present(*queue, &present_info)?;
//         device.device_wait_idle()?;
//     }

//     Ok(())
// }


// fn rotate_vertex(v: &mut Vertex, angle_x: f32, angle_y: f32) {
//     let (sin_x, cos_x) = angle_x.sin_cos();
//     let (sin_y, cos_y) = angle_y.sin_cos();
    
//     // Поворот вокруг Y
//     let x = v.pos[0] * cos_y - v.pos[2] * sin_y;
//     let z = v.pos[0] * sin_y + v.pos[2] * cos_y;
//     v.pos[0] = x;
//     v.pos[2] = z;
    
//     // Поворот вокруг X
//     let y = v.pos[1] * cos_x - v.pos[2] * sin_x;
//     let z = v.pos[1] * sin_x + v.pos[2] * cos_x;
//     v.pos[1] = y;
//     v.pos[2] = z;
// }

use log::*;
use winit::dpi::PhysicalSize;

fn main() -> Result<(), Box<dyn Error>> {

    unsafe { std::env::set_var("RUST_LOG", "DEBUG") };
    env_logger::init();

    let main_loop = winit::event_loop::EventLoop::new().unwrap();

    let monitor = main_loop.primary_monitor().unwrap();
    let mut video_mode = monitor.video_modes().next().unwrap();

    let window = winit::window::WindowBuilder::new()
        .with_title("Game")
        .with_inner_size(PhysicalSize::new(640, 480))
        .build(&main_loop)
        .unwrap();

    let cfg = DeviceConfigBuilder::new()
        .with_app(
            AppBuilder::new()
                .with_api_version(vk::API_VERSION_1_0)
                .with_app_name(c"App")
                .build()
        )
        .with_instance(|app_info, instance| {
            instance
                .with_app_info(app_info)
                .build()
        });

    // let windows_extensions = ash_window::enumerate_required_extensions(window.display_handle().unwrap().into()).unwrap_or(&[]);
    // let mut extensions = vec![];

    // for ext in windows_extensions {
    //     unsafe { extensions.push(CStr::from_ptr(*ext)); }
    // }

    // if (true) {

    //     let debug_extensions = [
    //         c"VK_EXT_debug_utils",
    //         c"VK_EXT_debug_report"
    //     ];

    //     for i in debug_extensions {
    //         extensions.push(i);
    //     }
    // }

    // let instance = InstanceBuilder::new()
    //     .add_extensions(extensions)
    //     .add_layer(c"VK_LAYER_KHRONOS_validation")
    //     .with_app_info(&app.raw)
    //     .build();

    // let phys_dev = PhysicalDeviceBuilder::new()
    //     .with_insatnce(&instance.raw)
    //     .build();

    // let surface = SurfaceBuilder::new()
    //     .with_entry(&instance.raw_entry)
    //     .with_instance(&instance.raw)
    //     .with_window(&window)
    //     .build();

    // let phys_ddev = &phys_dev.raw;

    // let queue_family = QueuesFamilyBuilder::new()
    //     .with_queue_family_prop(&phys_dev.phys_info.queue_family_prop)
    //     .with_surface(&surface.raw)
    //     .with_surface_load(&surface.raw_load)
    //     .with_phys_dev(phys_ddev)
    //     .build();

    // let device = DeviceBuilder::new()
    //     .add_extension(c"VK_KHR_swapchain")
    //     .queue_family(&queue_family)
    //     .with_instance(&instance.raw)
    //     .with_phys_dev(&phys_dev.raw)
    //     .build();

    // // Абстрактная очередь, выбирает нужную очередь исходя из требований
    // let queue = UniversalQueue::new(&device.raw, queue_family);

    // let formats = surface.get_surface_formats(phys_ddev);
    // let caps = surface.get_surface_capabilities(phys_ddev);
    // let present_modes = surface.get_surface_present_modes(phys_ddev);

    // //-----------------------------
    // //Выбор режима отрисовки
    // let present_mode =
    // if present_modes.contains(&PresentModeKHR::FIFO) {
    //     PresentModeKHR::FIFO
    // } else if present_modes.contains(&PresentModeKHR::MAILBOX) {
    //     PresentModeKHR::MAILBOX
    // } else {
    //     panic!("ERRROR");
    // };
    // //-----------------------------

    // //-----------------------------
    // // Выбор формата вывода изображений из имеющихся
    // let mut format: Option<SurfaceFormatKHR> = None;
    // for (index, surface_format_khr) in formats.iter().enumerate() {
    //     if surface_format_khr.format == Format::R8G8B8A8_SRGB {
    //         format = Some(formats[index])
    //     }
    // }

    // if format.is_none() {
    //     for (index, surface_format_khr) in formats.iter().enumerate() {
    //         if surface_format_khr.format == Format::B8G8R8A8_SRGB {
    //             format = Some(formats[index])
    //         }
    //     }
    // }

    // if format.is_none() {
    //     for (index, surface_format_khr) in formats.iter().enumerate() {
    //         if surface_format_khr.format == Format::R8G8B8A8_UNORM {
    //             format = Some(formats[index])
    //         }
    //     }
    // }

    // if format.is_none() {
    //     panic!("Нет нужного формата на устройстве");
    // }
    // //-----------------------------

    // let color_space = format.unwrap().color_space;
    // let format = format.unwrap().format;
    // let extent = caps.current_extent;

    // info!("Режим вывода: {:?}", present_mode);
    // info!("Формат вывода: {:?}, Цветовое пространство: {:?}", format, color_space);
    // info!("Разрешение экрана: {:?}", extent);

    // let swapchain = SwapchainBuilder::new()
    //     .with_color_space(color_space)
    //     .with_format(format)
    //     .with_resolution(extent)
    //     .with_transform(caps.supported_transforms)
    //     .with_present_mode(present_mode)
    //     .with_instance(&instance.raw)
    //     .with_device(&device.raw)
    //     .with_surface(&surface.raw)
    //     .build();

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

    // let shader = ShaderBuilder::new()
    //     .with_device(&device.raw)
    //     .with_fragment_shader_source("./shaders/spv/triangle-frag.spv")
    //     .with_vertex_shader_source("./shaders/spv/triangle-vert.spv")
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

    //     let binding_description = Vertex::get_binding_descriptions();
    //     let attribute_description = Vertex::get_attribute_descriptions();

    //     let vertex_input_state_info = vk::PipelineVertexInputStateCreateInfo::default()
    //         .vertex_attribute_descriptions(&attribute_description)
    //         .vertex_binding_descriptions(&binding_description);

    //     let mut cube_vertices = _VERTICES_DATA;
    //     for v in &mut cube_vertices {
    //         rotate_vertex(v, 0.5, 0.5); // Углы в радианах (~30°)
    //     }

    //     let vertex_buffer_size = (std::mem::size_of::<Vertex>() * _VERTICES_DATA.len()) as u64;

    //     let buffer_info = vk::BufferCreateInfo::default()
    //         .size(vertex_buffer_size)
    //         .usage(vk::BufferUsageFlags::VERTEX_BUFFER)
    //         .sharing_mode(vk::SharingMode::EXCLUSIVE);

    //     let buffer = unsafe { device.raw.create_buffer(&buffer_info, None).unwrap() };

    //     // Get memory requirements
    //     let req = unsafe { device.raw.get_buffer_memory_requirements(buffer) };

    //     // Find suitable memory type
    //     let index = find_memorytype_index(
    //         &req, 
    //         &phys_dev.phys_info.memory_prop, 
    //         vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT
    //     ).expect("Failed to find suitable memory type");

    //     // Allocate memory
    //     let alloc_info = vk::MemoryAllocateInfo::default()
    //         .allocation_size(req.size)
    //         .memory_type_index(index);

    //     let mem = unsafe { device.raw.allocate_memory(&alloc_info, None).unwrap() };

    //     // Bind memory to buffer
    //     unsafe { device.raw.bind_buffer_memory(buffer, mem, 0).expect("Failed to bind buffer memory") };

    //     // Map memory and copy data
    //     let ptr = unsafe {
    //         device.raw.map_memory(
    //             mem, 
    //             0, 
    //             vertex_buffer_size, 
    //             vk::MemoryMapFlags::empty()
    //         ).unwrap()
    //     };

    //     // Create properly aligned slice
    //     let mut slice = unsafe {
    //         std::slice::from_raw_parts_mut(
    //             ptr as *mut Vertex,
    //             _VERTICES_DATA.len()
    //         )
    //     };

    //     // Copy data to GPU memory
    //     slice.copy_from_slice(&cube_vertices);

    //     // Unmap memory if you want (optional for HOST_COHERENT)
    //     unsafe { device.raw.unmap_memory(mem) };
    // //-------------------------------

    // let pipeline = RenderPipelineBuilder::new()
    //     .with_vertex_shader(shader.vertex_shader)
    //     .with_fragment_shader(shader.fragment_shader)
    //     .with_resolution(extent)
    //     .with_format(format)
    //     .with_vertex_input_info(vertex_input_state_info)
    //     .with_input_assembly_info(
    //         vk::PipelineInputAssemblyStateCreateInfo::default()
    //                     .topology(PrimitiveTopology::TRIANGLE_LIST)
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

    // let (image_available_semaphore, render_finished_semaphore) = create_semaphores(&device.raw);
    // let fence_info = vk::FenceCreateInfo::default()
    //     .flags(FenceCreateFlags::SIGNALED);

    // let fence = unsafe { device.raw.create_fence(&fence_info, None).unwrap() };

    let _ = main_loop.run(|ev, ev_window| {
    match ev {
        winit::event::Event::WindowEvent { window_id: _, event } => match event {
            winit::event::WindowEvent::CloseRequested => ev_window.exit(),
            winit::event::WindowEvent::RedrawRequested => {
                    // let _ = draw_frame(
                    //     &device.raw,
                    //     &queue.raw[0][0],
                    //     &swapchain,
                    //     &command_pool,
                    //     &buffer,
                    //     &pipeline.raw,
                    //     &render_pass,
                    //     &framebuffers.frame_buffers,
                    //     image_available_semaphore,
                    //     render_finished_semaphore,
                    //     fence
                    // );
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