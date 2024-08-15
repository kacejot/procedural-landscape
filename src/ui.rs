use glium::Surface;
use imgui::{Context, Condition};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::winit::dpi::LogicalSize;
use imgui_winit_support::winit::event::{Event, WindowEvent};
use imgui_winit_support::winit::event_loop::EventLoop;
use imgui_winit_support::winit::window::WindowBuilder;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::Instant;

pub const FONT_SIZE: f32 = 13.0;

pub fn hello_world_imgui() {
    let mut imgui = Context::create();
    imgui.set_ini_filename(None);

    let event_loop = EventLoop::new().expect("Failed to create EventLoop");

    let builder = WindowBuilder::new()
        .with_title("Hello")
        .with_inner_size(LogicalSize::new(1024, 768));
    
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .set_window_builder(builder)
        .build(&event_loop);
    
    let mut renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");
    let mut platform = WinitPlatform::init(&mut imgui);
    
    {
        let dpi_mode = if let Ok(factor) = std::env::var("IMGUI_EXAMPLE_FORCE_DPI_FACTOR") {
            // Allow forcing of HiDPI factor for debugging purposes
            match factor.parse::<f64>() {
                Ok(f) => HiDpiMode::Locked(f),
                Err(e) => panic!("Invalid scaling factor: {}", e),
            }
        } else {
            HiDpiMode::Default
        };

        platform.attach_window(imgui.io_mut(), &window, dpi_mode);
    }

    imgui.style_mut().window_rounding = 10.0;

    let mut last_frame = Instant::now();
    event_loop
        .run(move |event, window_target| match event {
            Event::NewEvents(_) => {
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            }
            Event::AboutToWait => {
                platform
                    .prepare_frame(imgui.io_mut(), &window)
                    .expect("Failed to prepare frame");
                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                let ui = imgui.frame();

                let mut run = true;
                
                ui.window("Hello world")
                .size([300.0, 110.0], Condition::FirstUseEver)
                .build(|| {
                    ui.text_wrapped("Hello world!");
                    ui.text_wrapped("こんにちは世界！");
                    ui.button("Test Text");
                    ui.button("This...is...imgui-rs!");
                    ui.separator();
                    let mouse_pos = ui.io().mouse_pos;
                    ui.text(format!(
                        "Mouse Position: ({:.1},{:.1})",
                        mouse_pos[0], mouse_pos[1]
                    ));
                });

                if !run {
                    window_target.exit();
                }

                let mut target = display.draw();
                target.clear_color_srgb(1.0, 1.0, 1.0, 1.0);
                platform.prepare_render(ui, &window);
                let draw_data = imgui.render();
                renderer
                    .render(&mut target, draw_data)
                    .expect("Rendering failed");
                target.finish().expect("Failed to swap buffers");
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                ..
            } => {
                if new_size.width > 0 && new_size.height > 0 {
                    display.resize((new_size.width, new_size.height));
                }
                platform.handle_event(imgui.io_mut(), &window, &event);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => window_target.exit(),
            event => {
                platform.handle_event(imgui.io_mut(), &window, &event);
            }
        })
        .expect("EventLoop error");
}
