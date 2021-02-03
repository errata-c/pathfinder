// pathfinder/examples/canvas_moire/src/main.rs
//
// Copyright © 2019 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glutin::dpi::{PhysicalSize, LogicalSize};
use glutin::{ContextBuilder, GlProfile, GlRequest};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::window::WindowBuilder;

use pathfinder_canvas::{Canvas, CanvasFontContext, CanvasRenderingContext2D, FillStyle, Path2D};
use pathfinder_color::{ColorF, ColorU};
use pathfinder_geometry::vector::{Vector2F, Vector2I, vec2f, vec2i};
use pathfinder_gl::{GLDevice, GLVersion};
use pathfinder_renderer::concurrent::rayon::RayonExecutor;
use pathfinder_renderer::concurrent::scene_proxy::SceneProxy;
use pathfinder_renderer::gpu::options::{DestFramebuffer, RendererMode, RendererOptions};
use pathfinder_renderer::gpu::renderer::Renderer;
use pathfinder_renderer::options::BuildOptions;
use pathfinder_resources::embedded::EmbeddedResourceLoader;
use std::f32::consts::PI;
use std::f32;



const VELOCITY: f32 = 0.02;
const OUTER_RADIUS: f32 = 64.0;
const INNER_RADIUS: f32 = 48.0;

// FIXME(pcwalton): Adding more circles causes clipping problems. Fix them!
const CIRCLE_COUNT: u32 = 12;

const CIRCLE_SPACING: f32 = 48.0;
const CIRCLE_THICKNESS: f32 = 16.0;

const COLOR_CYCLE_SPEED: f32 = 0.0025;

fn main() {
    // Open a window.
    let event_loop = EventLoop::new();
    let window_size = vec2i(1067, 800);
    let physical_window_size = PhysicalSize::new(window_size.x() as f64, window_size.y() as f64);
    let logical_size = LogicalSize::new(window_size.x() as f64, window_size.y() as f64);

    // Open a window.
    let window_builder = 
        WindowBuilder::new().with_title("Moire example")
        .with_inner_size(physical_window_size);

    // Create an OpenGL 3.x context for Pathfinder to use.
    let windowed_context = ContextBuilder::new().with_gl(GlRequest::Latest)
                                          .with_gl_profile(GlProfile::Core)
                                          .build_windowed(window_builder, &event_loop)
                                          .unwrap();

    // Load OpenGL, and make the context current.
    let gl_context = unsafe { windowed_context.make_current().unwrap() };
    gl::load_with(|name| gl_context.get_proc_address(name) as *const _);

    // Get the real size of the window, taking HiDPI into account.
    let hidpi_factor: f64 = 1.0;// window.hidpi_factor();
    let physical_size: PhysicalSize<f64> = logical_size.to_physical(hidpi_factor);
    let framebuffer_size = vec2i(physical_size.width as i32, physical_size.height as i32);

    // Create a Pathfinder renderer.
    let device = GLDevice::new(GLVersion::GL3, 0);
    let mode = RendererMode::default_for_device(&device);
    let options = RendererOptions {
        background_color: Some(ColorF::white()),
        dest: DestFramebuffer::full_window(window_size),
        ..RendererOptions::default()
    };
    let renderer = Renderer::new(device, &EmbeddedResourceLoader, mode, options);

    let mut moire_renderer = MoireRenderer::new(renderer, window_size, framebuffer_size);

    // Wait for a keypress.
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::MainEventsCleared => {
                moire_renderer.render();

                gl_context.swap_buffers().unwrap();

                gl_context.window().request_redraw();
            },
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } |
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput {
                    input: KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Escape), .. },
                    ..
                },
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            },
            _ => {
                *control_flow = ControlFlow::Poll;
            },
        };
    })
}

struct MoireRenderer {
    renderer: Renderer<GLDevice>,
    font_context: CanvasFontContext,
    scene: SceneProxy,
    frame: i32,
    window_size: Vector2I,
    drawable_size: Vector2I,
    device_pixel_ratio: f32,
    colors: ColorGradient,
}

impl MoireRenderer {
    fn new(renderer: Renderer<GLDevice>, window_size: Vector2I, drawable_size: Vector2I)
           -> MoireRenderer {
        let level = renderer.mode().level;
        MoireRenderer {
            renderer,
            font_context: CanvasFontContext::from_system_source(),
            scene: SceneProxy::new(level, RayonExecutor),
            frame: 0,
            window_size,
            drawable_size,
            device_pixel_ratio: drawable_size.x() as f32 / window_size.x() as f32,
            colors: ColorGradient::new(),
        }
    }

    fn render(&mut self) {
        // Calculate animation values.
        let time = self.frame as f32;
        let (sin_time, cos_time) = (f32::sin(time * VELOCITY), f32::cos(time * VELOCITY));
        let color_time = time * COLOR_CYCLE_SPEED;
        let background_color = self.colors.sample(color_time);
        let foreground_color = self.colors.sample(color_time + 0.5);

        // Calculate outer and inner circle centers (circle and Leminscate of Gerono respectively).
        let window_center = self.window_size.to_f32() * 0.5;
        let outer_center = window_center + vec2f(sin_time, cos_time) * OUTER_RADIUS;
        let inner_center = window_center + vec2f(1.0, sin_time) * (cos_time * INNER_RADIUS);

        // Clear to background color.
        self.renderer.options_mut().background_color = Some(background_color);

        // Make a canvas.
        let mut canvas =    
            Canvas::new(self.drawable_size.to_f32()).get_context_2d(self.font_context.clone());
        canvas.set_line_width(CIRCLE_THICKNESS * self.device_pixel_ratio);
        canvas.set_stroke_style(FillStyle::Color(foreground_color.to_u8()));
        canvas.set_global_alpha(0.75);

        // Draw circles.
        self.draw_circles(&mut canvas, outer_center);
        self.draw_circles(&mut canvas, inner_center);

        // Build and render scene.
        self.scene.replace_scene(canvas.into_canvas().into_scene());
        self.scene.build_and_render(&mut self.renderer, BuildOptions::default());

        self.frame += 1;
    }

    fn draw_circles(&self, canvas: &mut CanvasRenderingContext2D, mut center: Vector2F) {
        center *= self.device_pixel_ratio;
        for index in 0..CIRCLE_COUNT {
            let radius = (index + 1) as f32 * CIRCLE_SPACING * self.device_pixel_ratio;
            let mut path = Path2D::new();
            path.ellipse(center, radius, 0.0, 0.0, PI * 2.0);
            canvas.stroke_path(path);
        }
    }
}

struct ColorGradient([ColorF; 5]);

impl ColorGradient {
    fn new() -> ColorGradient {
        // Extracted from https://stock.adobe.com/69426938/
        ColorGradient([
            ColorU::from_u32(0x024873ff).to_f32(),
            ColorU::from_u32(0x03658cff).to_f32(),
            ColorU::from_u32(0x0388a6ff).to_f32(),
            ColorU::from_u32(0xf28e6bff).to_f32(),
            ColorU::from_u32(0xd95a4eff).to_f32(),
        ])
    }

    fn sample(&self, mut t: f32) -> ColorF {
        let count = self.0.len();
        t *= count as f32;
        let (lo, hi) = (t.floor() as usize % count, t.ceil() as usize % count);
        self.0[lo].lerp(self.0[hi], f32::fract(t))
    }
}
