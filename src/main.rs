#![allow(dead_code)]
// #![feature(once_cell, result_option_inspect)]
use std::{path::PathBuf, sync::Arc};

use clap::Parser;
use tokio::runtime::Builder;
use wgpu::{
    Backends, DeviceDescriptor, Features, Instance, Limits, PowerPreference, PresentMode,
    RequestAdapterOptions, SurfaceConfiguration, TextureFormat, TextureUsages,
};
use winit::{
    dpi::PhysicalPosition,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowAttributes},
};

mod flutter_application;
use flutter_application::{FlutterApplication, FlutterApplicationCallback};

mod action_key;
mod keyboard_logical_key_map;
mod keyboard_physical_key_map;

mod flutter_bindings;
mod utils;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The Flutter application code needs to be snapshotted using
    /// the Flutter tools and the assets packaged in the appropriate
    /// location. This can be done for any Flutter application by
    /// running `flutter build bundle` while in the directory of a
    /// valid Flutter project. This should package all the code and
    /// assets in the "build/flutter_assets" directory. Specify this
    /// directory as the first argument to this utility.
    pub asset_bundle_path: PathBuf,
    /// Typically empty. These extra flags are passed directly to the
    /// Flutter engine. To see all supported flags, run
    /// `flutter_tester --help` using the test binary included in the
    /// Flutter tools.
    pub flutter_flags: Vec<String>,
}

fn main() -> Result<(), std::io::Error> {
    env_logger::init();
    let args = Args::parse();

    let event_loop: EventLoop<FlutterApplicationCallback> =
        EventLoop::<FlutterApplicationCallback>::with_user_event()
            .build()
            .unwrap();

    let window_attr = WindowAttributes::default().with_title("Flutter Embedder");

    #[cfg(any(x11_platform, wayland_platform))]
    if let Some(token) = event_loop.read_token_from_env() {
        startup_notify::reset_activation_token_env();
        info!("Using token {:?} to activate a window", token);
        window_attributes = window_attributes.with_activation_token(token);
    }

    #[cfg(x11_platform)]
    match std::env::var("X11_VISUAL_ID") {
        Ok(visual_id_str) => {
            info!("Using X11 visual id {visual_id_str}");
            let visual_id = visual_id_str.parse()?;
            window_attributes = window_attributes.with_x11_visual(visual_id);
        }
        Err(_) => info!("Set the X11_VISUAL_ID env variable to request specific X11 visual"),
    }

    #[cfg(x11_platform)]
    match std::env::var("X11_SCREEN_ID") {
        Ok(screen_id_str) => {
            info!("Placing the window on X11 screen {screen_id_str}");
            let screen_id = screen_id_str.parse()?;
            window_attributes = window_attributes.with_x11_screen(screen_id);
        }
        Err(_) => {
            info!("Set the X11_SCREEN_ID env variable to place the window on non-default screen")
        }
    }

    log::info!("Creating window with attributes {:?}", window_attr);

    let window = event_loop.create_window(window_attr).unwrap();

    let rt = Arc::new(Builder::new_multi_thread().build()?);
    let inner_rt = rt.clone();

    let instance = Instance::new(&wgpu::InstanceDescriptor {
        backends: Backends::VULKAN,
        ..Default::default()
    });

    for adapter in instance.enumerate_adapters(Backends::VULKAN) {
        log::info!("Found Adapter: {:?} ", adapter.get_info(),);
    }

    let surface = instance.create_surface(&window).unwrap();

    rt.block_on(async {
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: None,
                    required_features: Features::CLEAR_TEXTURE,
                    required_limits: Limits::downlevel_defaults(),
                    memory_hints: wgpu::MemoryHints::Performance,
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let size = window.inner_size();

        let capabilites = surface.get_capabilities(&adapter);
        log::debug!("Supported formats: {:?}", capabilites.formats);
        let formats = capabilites.formats;
        let format = formats
            .into_iter()
            .find(|&format| format == TextureFormat::Bgra8Unorm)
            .expect("Adapter doesn't support BGRA8 render buffer.");

        surface.configure(
            &device,
            &SurfaceConfiguration {
                usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::COPY_DST,
                format,
                width: size.width,
                height: size.height,
                present_mode: PresentMode::Fifo,
                desired_maximum_frame_latency: 2,
                alpha_mode: wgpu::CompositeAlphaMode::Auto,
                view_formats: Vec::new(),
            },
        );

        let mut app = FlutterApplication::new(
            inner_rt,
            &args.asset_bundle_path,
            args.flutter_flags,
            surface,
            Arc::new(instance),
            device,
            queue,
            event_loop.create_proxy(),
            &window,
            |cursor| {
                // if let Some(cursor) = cursor {
                //     window.set_cursor_visible(true);
                //     window.set_cursor_icon(cursor);
                // } else {
                //     window.set_cursor_visible(false);
                // }
                // // TODO: pass window somehow
            },
        );

        log::info!("Created Flutter App, and running it...");

        app.run();

        // Trigger a FlutterEngineSendWindowMetricsEvent to communicate the initial
        // size of the window.
        metrics_changed(&app, &window);

        let _ = event_loop.run(|event, active_event_loop| {
            // let _ = &adapter;
            // active_event_loop.set_control_flow(ControlFlow::Wait);

            // *control_flow = ControlFlow::Wait;
            match event {
                Event::UserEvent(handler) => {
                    if handler(&mut app) {
                        active_event_loop.exit();
                        // *control_flow = ControlFlow::Exit;
                    }
                }

                // Event::RedrawRequested(_window_id) => {
                //     app.schedule_frame();
                // }
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        // *control_flow = ControlFlow::Exit;
                        active_event_loop.exit();
                    }
                    WindowEvent::Moved(_)
                    | WindowEvent::Resized(_)
                    | WindowEvent::ScaleFactorChanged { .. } => {
                        metrics_changed(&app, &window);
                    }
                    WindowEvent::MouseInput {
                        device_id,
                        state,
                        button,
                        ..
                    } => {
                        app.mouse_buttons(device_id, state, button);
                    }
                    WindowEvent::CursorEntered { device_id } => {
                        app.mouse_entered(device_id);
                    }
                    WindowEvent::CursorLeft { device_id } => {
                        app.mouse_left(device_id);
                    }
                    WindowEvent::CursorMoved {
                        device_id,
                        position,
                        ..
                    } => {
                        app.mouse_moved(device_id, position);
                    }
                    WindowEvent::MouseWheel {
                        device_id,
                        delta,
                        phase,
                        ..
                    } => {
                        app.mouse_wheel(device_id, delta, phase);
                    }
                    WindowEvent::ModifiersChanged(state) => {
                        app.modifiers_changed(state.state());
                    }
                    WindowEvent::KeyboardInput {
                        event,
                        device_id,
                        is_synthetic,
                    } => {
                        app.key_event(device_id, event, is_synthetic);
                    }
                    WindowEvent::Focused(focused) => {
                        app.focused(focused);
                    }

                    WindowEvent::RedrawRequested => {
                        app.schedule_frame();
                    }
                    _ => {}
                },
                _ => {}
            }
        });
    });
    Ok(())
}

fn metrics_changed(application: &FlutterApplication, window: &Window) {
    log::info!("Metrics Changed");

    let size = window.inner_size();
    let position = window
        .inner_position()
        .unwrap_or(PhysicalPosition { x: 0, y: 0 });
    log::debug!(
        "scale_factor = {:?}",
        window.scale_factor(),
        // window
        //     .current_monitor()
        //     .map(|monitor| monitor.scale_factor())
    );
    application.metrics_changed(
        size.width,
        size.height,
        window
            .current_monitor()
            .map(|monitor| monitor.scale_factor())
            .unwrap_or(1.0),
        position.x,
        position.y,
    );
}
