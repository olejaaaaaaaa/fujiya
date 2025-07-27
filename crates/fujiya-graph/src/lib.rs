use std::{collections::HashMap, error::Error};
use ash::vk::{self, CommandBuffer, PipelineStageFlags, SubmitInfo};
use fujiya_render::{CommandPool, FrameSync, GPUBuffer, RenderContext, RenderPass, RenderPipeline};

#[derive(Default)]
pub struct RenderGraphResource {
    pub pipeline: HashMap<&'static str, RenderPipeline>,
    pub buffers: HashMap<&'static str, GPUBuffer>,
    pub command_buffers: Vec<CommandBuffer>,
    pub command_pool: HashMap<&'static str, CommandPool>,
    pub render_pass: HashMap<&'static str, RenderPass>
}

#[derive(Default)]
pub struct RenderGraph {
    pub resources: RenderGraphResource,
    pub nodes: HashMap<&'static str, Box<dyn Fn(&mut RenderGraphResource, &RenderContext, u32) -> Result<(), Box<dyn Error>>>>,
    pub sync: Vec<FrameSync>,
    pub current_frame: usize
}

impl RenderGraph {

    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn register_render_pass(&mut self, name: &'static str, pass: RenderPass) {
        self.resources.render_pass.insert(name, pass);
    }

    pub fn register_command_pool(&mut self, name: &'static str, pool: CommandPool) {
        self.resources.command_pool.insert(name, pool);
    }

    pub fn register_buffer(&mut self, name: &'static str, buffer: GPUBuffer) {
        self.resources.buffers.insert(name, buffer);
    }

    pub fn register_pipeline(&mut self, name: &'static str, pipeline: RenderPipeline) {
        self.resources.pipeline.insert(name, pipeline);
    }

    pub fn add_raw_pass<F>(&mut self, name: &'static str, clojure: F)
        where F: Fn(&mut RenderGraphResource, &RenderContext, u32) -> Result<(), Box<dyn Error>> + 'static
    {
        self.nodes.insert(name, Box::new(clojure));
    }

    pub fn compile(&mut self) {

    }

    pub fn execute(&mut self, ctx: &RenderContext) {

        for (name, func) in &self.nodes {

            if self.sync.is_empty() {
                let frame_count = ctx.window_manager.frame_buffers.raw.len();
                for _ in 0..frame_count {
                    self.sync.push(FrameSync::new(ctx.graphics_device.raw_device()));
                }
            }

            let current_frame = self.current_frame;
            let fence = self.sync[current_frame].fence;
            let swapchain = &ctx.window_manager.swapchain;
            let queue = ctx.graphics_device.universal_queue.raw_graphics();
            let device = ctx.graphics_device.raw_device();
            let sync = &self.sync;

            // 2. Дождаться завершения предыдущего кадра
            unsafe {
                device.wait_for_fences(&[fence], true, u64::MAX).unwrap();
                device.reset_fences(&[fence]).unwrap();
            }

            // 3. Получить новое изображение из swapchain
            let (image_index, _) = unsafe {
                swapchain.swapchain_load.acquire_next_image(
                    swapchain.raw,
                    u64::MAX,
                    sync[current_frame].image_available,
                    vk::Fence::null(),
                )
            }.unwrap();

            // 4. Выполнить рендер-пассы (теперь безопасно)
            if let Err(err) = func(&mut self.resources, ctx, image_index) {
                log::error!("Error in {:?} pass: {:?}", name, err);
            }

            // 5. Отправить команды в очередь
            let binding1 = [sync[current_frame].image_available];
            let binding2 = [sync[current_frame].render_finished];

            let submit_info = vk::SubmitInfo::default()
                .wait_semaphores(&binding1)
                .wait_dst_stage_mask(&[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT])
                .command_buffers(&self.resources.command_buffers)
                .signal_semaphores(&binding2);

            unsafe {
                device.queue_submit(queue, &[submit_info], fence).unwrap();
            }

            // 6. Представить изображение
            let binding1 = [sync[current_frame].render_finished];
            let binding2 = [swapchain.raw];
            let binding3 = [image_index];

            let present_info = vk::PresentInfoKHR::default()
                .wait_semaphores(&binding1)
                .swapchains(&binding2)
                .image_indices(&binding3);

            unsafe {
                swapchain.swapchain_load.queue_present(queue, &present_info).unwrap();
            }

            self.current_frame = (current_frame + 1) % self.sync.len();
        }
    }
}

