use polygon_graphics::*;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    let mut player = Player {
        fov: 60,
        half_fov: 30,
        x: 1.5,
        y: 1.5,
        frustum: VF_DEFAULT,
    };
    let event_loop = EventLoop::new().unwrap();
    let builder = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build(&event_loop)
        .unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let window_size = builder.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &builder);
    let mut pixels = Pixels::new(window_size.width, window_size.height, surface_texture).unwrap();
    for pixel in pixels.frame_mut().chunks_exact_mut(4) {
        pixel[0] = 255; // R
        pixel[1] = 27; // G
        pixel[2] = 71; // B
        pixel[3] = 0xff; // A
    }
    let mut wasd: [bool; 4] = [false, false, false, false];
    event_loop
        .run(move |event, elwt| {
            //            print!("\r");
            //            let now = Instant::now();
            pixels.render().unwrap();
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    println!("The close button was pressed; stopping");
                    elwt.exit();
                }
                Event::AboutToWait => {
                    builder.request_redraw();
                }
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    builder.request_redraw();
                }
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput {
                        device_id: _,
                        event,
                        is_synthetic: _,
                    } => match event.physical_key {
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyA) => {
                            if event.state.is_pressed() {
                                wasd[1] = true;
                            } else {
                                wasd[1] = false
                            }
                        }
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyD) => {
                            if event.state.is_pressed() {
                                wasd[3] = true;
                            } else {
                                wasd[3] = false
                            }
                        }
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyW) => {
                            if event.state.is_pressed() {
                                wasd[0] = true;
                            } else {
                                wasd[0] = false
                            }
                        }
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyS) => {
                            if event.state.is_pressed() {
                                wasd[2] = true;
                            } else {
                                wasd[2] = false
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                },
                _ => (),
            }
            //            print!("{:?}fps", (1.0 / now.elapsed().as_secs_f32()) as u32);
        })
        .unwrap();
}
