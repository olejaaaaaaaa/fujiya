#![warn(unused_qualifications)]

use std::{collections::HashMap, error::Error, ffi::CStr, fs::{read_dir, write, DirEntry, File}, io::Read, mem::offset_of, panic, process::Command, rc::Rc, time::Instant, u64};

use ash::vk::{self, AttachmentReference, BufferUsageFlags, CommandBuffer, CommandBufferLevel, Extent2D, Fence, FenceCreateFlags, Format, PhysicalDeviceType, PresentModeKHR, PrimitiveTopology, SurfaceFormatKHR, VertexInputAttributeDescription, VertexInputBindingDescription, API_VERSION_1_0, API_VERSION_1_3};
use fujiya_sound::enumerate_sound_device;
use winit::raw_window_handle::*;
use log::*;
use winit::{dpi::PhysicalSize, raw_window_handle::HasDisplayHandle};

use fujiya_render::*;
use fujiya_macros::Vertex;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
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

const _VERTICES_DATA: [Vertex; 3] = [
    Vertex { pos: [0.0, 0.5,  0.0], color: [1.0, 0.0, 0.0] },
    Vertex { pos: [ -0.5, -0.5,  0.0], color: [0.0, 1.0, 0.0] },
    Vertex { pos: [ 0.5,  -0.5,  0.0], color: [0.0, 0.0, 1.0] },
];

use fujiya_graph::*;
use fujiya_assets::{load_mesh_data, open_gltf};


#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
struct UniformBufferObject {
    model: [[f32; 4]; 4],
    view: [[f32; 4]; 4],
    projection: [[f32; 4]; 4],
}


fn apply_isometric_transform(vertex: [f32; 3]) -> [f32; 3] {
    let angle_y = 45.0f32.to_radians();
    let angle_x = 35.264f32.to_radians();
    
    // Поворот
    let (sin_y, cos_y) = angle_y.sin_cos();
    let x = vertex[0] * cos_y - vertex[2] * sin_y;
    let z = vertex[0] * sin_y + vertex[2] * cos_y;
    
    let (sin_x, cos_x) = angle_x.sin_cos();
    let y = vertex[1] * cos_x - z * sin_x;
    let z = vertex[1] * sin_x + z * cos_x;
    
    // Смещаем Z в положительный диапазон для Vulkan
    [x, y, z + 0.5] // +0.5 чтобы центрировать
}

fn main() -> Result<(), Box<dyn Error>> {

    unsafe { std::env::set_var("RUST_LOG", "DEBUG") };
    env_logger::init();

    let main_loop = winit::event_loop::EventLoop::new().unwrap();
    let window = winit::window::WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(800, 600))
        .with_title("Game")
        .build(&main_loop)
        .unwrap();

    let ctx: RenderContext = RenderContext::default(window);

    let buffer_size = size_of::<UniformBufferObject>() as u64;

    let uniform_buffer = GPUBuffer::new(
        &ctx.graphics_device.device.raw,
        &ctx.graphics_device.phys_dev.phys_info.memory_prop,
        buffer_size,
        vk::BufferUsageFlags::UNIFORM_BUFFER,
        vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
    ).unwrap();

    let mut ubo = UniformBufferObject {
        model: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.5, 0.0, 0.0, 1.0],
        ],
        view: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, -2.0, 1.0],
        ],
        projection: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, -2.0, 1.0],
        ],
    };

    uniform_buffer.upload_data(&ctx.graphics_device.device.raw, &[ubo]);

    let layout = DescriptorSetLayoutBuilder::new()
        .with_device(ctx.graphics_device.raw_device())
        .with_bindings(&[
            vk::DescriptorSetLayoutBinding::default()
                .binding(0)
                .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::VERTEX)
            ]
        )
        .build();

    let descriptor_pool = DescriptorPoolBuilder::new()
        .with_device(ctx.graphics_device.raw_device())
        .with_max_sets(1)
        .with_pool_sizes(&[
            vk::DescriptorPoolSize::default()
                .ty(vk::DescriptorType::UNIFORM_BUFFER)
                .descriptor_count(1)
        ])
        .build();

    // Выделяем Descriptor Set
    let layout = std::slice::from_ref(&layout.raw);
    let allocate_info = vk::DescriptorSetAllocateInfo::default()
        .descriptor_pool(descriptor_pool.raw)
        .set_layouts(layout);

    let descriptor_sets = unsafe {
        ctx.graphics_device.device.raw
            .allocate_descriptor_sets(&allocate_info)
            .unwrap()
    };

    let descriptor_set = descriptor_sets[0];

    let buffer_info = [vk::DescriptorBufferInfo::default()
        .buffer(uniform_buffer.raw)
        .offset(0)
        .range(buffer_size)
    ];

    let write_descriptor = vk::WriteDescriptorSet::default()
        .dst_set(descriptor_set)
        .dst_binding(0)  // Как в layout_binding
        .dst_array_element(0)
        .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
        .buffer_info(&buffer_info);

    unsafe {
        ctx.graphics_device.device.raw
            .update_descriptor_sets(&[write_descriptor], &[]);
    }

    let pipeline = StandartPipelineBuilder::new()
        .with_graphics_device(&ctx)
        .with_fragment_shader(load_spv(r"C:\Users\Oleja\Desktop\d\fujiya\shared\shaders\spv\triangle-frag.spv"))
        .with_vertex_shader(load_spv(r"C:\Users\Oleja\Desktop\d\fujiya\shared\shaders\spv\triangle-vert.spv"))
        .build(layout[0]);

    let (gltf, index) = &load_mesh_data(&open_gltf("./shared/assets/models/box.glb").unwrap())[0];

    let mut data: Vec<Vertex> = vec![];

    let mut color = [0.2, 0.2, 0.0];
    let mut count = 0;

    for vertex in gltf {

        let mut v = *vertex;
        v[0] /= 2.0;
        v[1] /= 2.0;
        v[2] /= 2.0;

        v = apply_isometric_transform(v);

        if count % 3 == 0 {
            count = 0;
            color[0] += 0.01;
            color[1] += 0.01;
            color[2] += 0.01;
        } else {
            count += 1;
        }

        data.push(Vertex {
            pos: v, // Y остаётся без изменений
            color
        });
    }

    let command_pool = CommandPoolBuilder::new()
        .device(&ctx.graphics_device.device.raw)
        .family_index(ctx.graphics_device.universal_queue.graphics_index())
        .build();

    let gpu_buffer = GPUBuffer::new(
        &ctx.graphics_device.device.raw,
        &ctx.graphics_device.phys_dev.phys_info.memory_prop,
        (size_of::<Vertex>() * data.len()) as u64,
        BufferUsageFlags::VERTEX_BUFFER,
        vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT
    ).unwrap();

    gpu_buffer.upload_data(ctx.graphics_device.raw_device(), &data);

    println!("{:?}", index.len() as u64);

    let index_buffer = GPUBuffer::new(
        &ctx.graphics_device.device.raw,
        &ctx.graphics_device.phys_dev.phys_info.memory_prop,
        (std::mem::size_of::<u32>() * index.len()) as u64,
        BufferUsageFlags::INDEX_BUFFER,
        vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT
    ).unwrap();

    index_buffer.upload_data(ctx.graphics_device.raw_device(), &index);

    println!("Vertex count: {}", data.len());
    println!("vertex: {:?}", data);
    println!("Index count: {}", index.len());
    println!("First 3 indices: {:?}", &index);

    //------------------------------
    let mut graph = RenderGraph::new();
    graph.register_command_pool("pool", command_pool);
    graph.register_buffer("buf", gpu_buffer);
    graph.register_buffer("index_buf", index_buffer);
    graph.register_pipeline("pipe", pipeline);
    graph.add_raw_pass("Simple", |res, ctx, image_index| {

        let device = ctx.graphics_device.raw_device();
        let buffer = res.buffers.get("buf").ok_or("ERR")?;
        let index_buffer = res.buffers.get("index_buf").ok_or("ERR")?;
        let pipeline = res.pipeline.get("pipe").ok_or("ERR")?;
        let command_pool = res.command_pool.get("pool").ok_or("ERR")?;
        let command_buffer = command_pool.create_command_buffers(device, 1, CommandBufferLevel::PRIMARY)[0];
        let render_pass = &ctx.window_manager.render_pass;
        let current_extent = ctx.window_manager.caps.current_extent;

        let frame_buffer = ctx.window_manager.frame_buffers.raw[image_index as usize];

        let clear_values = [vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.0, 0.0, 0.0, 1.0],
            },
        }];

        let render_pass_begin_info = vk::RenderPassBeginInfo::default()
            .render_pass(render_pass.raw)
            .framebuffer(frame_buffer)
            .render_area(vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: current_extent,
            })
            .clear_values(&clear_values);

        let begin_info = vk::CommandBufferBeginInfo::default()
            .flags(vk::CommandBufferUsageFlags::SIMULTANEOUS_USE);

        unsafe {

            device.begin_command_buffer(command_buffer, &begin_info)
                .expect("Failed to begin command buffer");

            device.cmd_begin_render_pass(
                command_buffer,
                &render_pass_begin_info,
                vk::SubpassContents::INLINE,
            );

            device.cmd_bind_pipeline(
                command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                pipeline.raw,
            );

            device.cmd_bind_vertex_buffers(command_buffer, 0, &[buffer.raw], &[0]);
            device.cmd_bind_index_buffer(command_buffer, index_buffer.raw, 0, vk::IndexType::UINT32);
            device.cmd_draw_indexed(
                command_buffer,
                36, // Для куба обычно 36 индексов (12 треугольников × 3 вершины)
                1,  // instance count
                0,  // first index
                0,  // vertex offset
                0   // first instance
            );

            //device.cmd_draw(command_buffer, 36, 1, 0, 0);
            device.cmd_end_render_pass(command_buffer);
            device.end_command_buffer(command_buffer)
                .expect("Failed to end command buffer");
        }

        res.command_buffers.push(command_buffer);
        Ok(())
    });



    let _ = main_loop.run(move |ev, ev_window| {
    match ev {
        winit::event::Event::WindowEvent { window_id: _, event } => match event {
            winit::event::WindowEvent::KeyboardInput { event, .. } => {
                match event {
                    _ => {}
                }
            },
            winit::event::WindowEvent::CloseRequested => ev_window.exit(),
            winit::event::WindowEvent::RedrawRequested => {
                graph.execute(&ctx);
            },
            winit::event::WindowEvent::Resized(_) => {

            },
            _ => {}
        },
        winit::event::Event::AboutToWait => {
            ctx.window_manager.window.request_redraw();
        }
        _ => {}
    }
    });

    Ok(())
}