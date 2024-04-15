use std::time::{
    Duration,
    Instant,
};

use wgpu::{
    Adapter,
    Device,
    Instance,
    Queue,
    Surface,
    SurfaceConfiguration,
};

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

    let config = surface
        .get_default_config(&adapter, size.width, size.height)
        .unwrap();
    surface.configure(&device, &config);

    return (instance, surface, config, adapter, device, queue);
}


fn main() {
    let mut event_loop = EventLoop::new().unwrap();
    let builder = WindowBuilder::new();
    let window = builder
        .with_title("Sprint The Game")
        .with_inner_size(LogicalSize::new(1280, 720))
        .with_position(LogicalPosition::new((((1920.0 / 1.25) - 1280.0) / 2.0) as u32, (((1080.0 / 1.25) - 720.0) / 2.0) as u32))
        .build(&event_loop).unwrap();

    let (_instance, surface, mut config, adapter, device, queue) = pollster::block_on(build_backend(&window));

    let application = Application::new();

    let start = Instant::now();
    let mut last_frame_time = start.elapsed();
    let mut last_tick_time = start.elapsed();
    let mut last_timer_time = start.elapsed();

    const FRAME_RATE: f32 = 1.0 / 120.0;
    const TICK_RATE: f32 = 1.0 / 60.0;

    let mut ticks = 0usize;
    let mut frames = 0usize;

    'main: loop {
        let now = start.elapsed();
        let delta_time = (now - last_tick_time).as_secs_f32();

        if delta_time > TICK_RATE {
            last_tick_time += Duration::from_secs_f32(TICK_RATE);

            ticks += 1;
        } else if delta_time > FRAME_RATE {
            last_frame_time += Duration::from_secs_f32(FRAME_RATE);

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

                                window.request_redraw();
                            }
                            WindowEvent::CloseRequested => target.exit(),
                            WindowEvent::RedrawRequested => {}
                            WindowEvent::KeyboardInput {
                                event,
                                ..
                            } => {}
                            WindowEvent::MouseInput {
                                state,
                                button,
                                ..
                            } => {}
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
        }

        if (now - last_timer_time).as_secs_f32() > 1.0 {
            last_timer_time += Duration::from_secs_f32(1.0);

            println!("Sprint The Game is running at {} tps and {} fps", ticks, frames);

            frames = 0;
            ticks = 0;
        }
    }
}
