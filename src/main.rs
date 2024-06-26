use std::time::{
    Duration,
    Instant,
};

use wgpu::{Adapter, Device, Instance, Queue, Surface, SurfaceConfiguration, TextureFormat};

use winit::{
    event::{
        Event,
        WindowEvent,
    },
    event_loop::EventLoop,
    platform::pump_events::{
        EventLoopExtPumpEvents,
        PumpStatus,
    },
    window::{
        Window,
        WindowBuilder,
    },
    dpi::{
        LogicalSize,
        LogicalPosition,
    },
};
use winit::window::WindowButtons;
use crate::sprint_the_game::Application;

mod sprint_the_game;
pub mod logic;
pub mod renderer;

async fn build_backend(window: &Window) -> (Instance, Surface, SurfaceConfiguration, Adapter, Device, Queue) {
    let instance = wgpu::Instance::default();

    let surface = instance.create_surface(window).unwrap();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    let mut size = window.inner_size();
    size.width = size.width.max(1);
    size.height = size.height.max(1);

    let mut config = surface
        .get_default_config(&adapter, size.width, size.height)
        .unwrap();

    config.format = TextureFormat::Bgra8Unorm;

    surface.configure(&device, &config);

    return (instance, surface, config, adapter, device, queue);
}


fn main() {
    let mut event_loop = EventLoop::new().unwrap();
    let builder = WindowBuilder::new();
    let window = builder
        .with_title("Sprint The Game")
        .with_inner_size(LogicalSize::new(1280, 720))
        .with_resizable(false)
        .with_enabled_buttons(WindowButtons::CLOSE | WindowButtons::MINIMIZE)
        .build(&event_loop).unwrap();

    if let Some(monitor) = window.current_monitor() {
        let screen_size = monitor.size();
        let window_size = window.outer_size();

        window.set_outer_position(winit::dpi::PhysicalPosition {
            x: screen_size.width.saturating_sub(window_size.width) as f64 / 2.
                + monitor.position().x as f64,
            y: screen_size.height.saturating_sub(window_size.height) as f64 / 2.
                + monitor.position().y as f64,
        });
    }

    let (_instance, surface, mut config, adapter, device, queue) = pollster::block_on(build_backend(&window));

    let mut application = Application::new(&device, &surface, &adapter, &queue, &config);

    let start = Instant::now();
    let mut last_tick_time = start.elapsed();
    let mut last_timer_time = start.elapsed();

    let mut frames = 0;

    'main: loop {
        let now = start.elapsed();
        let delta_time = (now - last_tick_time).as_secs_f32();
        last_tick_time = now;

        application.update(delta_time);

        let timeout = Some(Duration::ZERO);
        let status = event_loop.pump_events(timeout, |event, target| {
            match event {
                Event::AboutToWait => window.request_redraw(),
                Event::WindowEvent {
                    event,
                    ..
                } => {
                    match event {
                        WindowEvent::Resized(new_size) => {
                            config.width = new_size.width.max(1);
                            config.height = new_size.height.max(1);

                            surface.configure(&device, &config);

                            application.process_resize((new_size.width, new_size.height), &queue);

                            window.request_redraw();
                        }
                        WindowEvent::CloseRequested => target.exit(),
                        WindowEvent::RedrawRequested => application.render(&device, &surface, &queue),
                        WindowEvent::KeyboardInput {
                            event,
                            ..
                        } => application.process_keyboard(event),
                        WindowEvent::MouseInput {
                            state,
                            button,
                            ..
                        } => application.process_mouse(state, button),
                        _ => {}
                    }
                }
                _ => {}
            }
        });

        if let PumpStatus::Exit(_) = status {
            break 'main;
        }

        frames += 1;

        if (now - last_timer_time).as_secs_f32() > 1.0 {
            last_timer_time += Duration::from_secs_f32(1.0);

            println!("Sprint The Game is running at {} fps", frames);

            frames = 0;
        }
    }
}
