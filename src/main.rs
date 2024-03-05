use std::f32::consts::PI;

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
        x: 0.0,
        y: -40.0,
        frustum: VF_DEFAULT,
    };
    let mut test_object = Object {
        width: 10,
        height: 10,
        coords: [-5.0, -5.0, -5.0],
        animated: false,
        vertices: Vec::new(),
        indices: Vec::new(),
    };
    (test_object.vertices, test_object.indices) = parse_obj();
    let mut rot: f32 = -1.0;
    let event_loop = EventLoop::new().unwrap();
    let builder = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT))
        .with_title("poly graphiz")
        .build(&event_loop)
        .unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let window_size = builder.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &builder);
    let mut pixels = Pixels::new(window_size.width, window_size.height, surface_texture).unwrap();
    for pixel in pixels.frame_mut().chunks_exact_mut(4) {
        pixel[0] = 0; // R
        pixel[1] = 0; // G
        pixel[2] = 0; // B
        pixel[3] = 0xff; // A
    }
    let mut wasd: [bool; 6] = [false, false, false, false, false, false];
    parse_obj();
    event_loop
        .run(move |event, elwt| {
            //            println!("{:?}", player.frustum.x.to_degrees());
            for pixel in pixels.frame_mut().chunks_exact_mut(4) {
                pixel[0] = 0; // R
                pixel[1] = 0; // G
                pixel[2] = 0; // B
                pixel[3] = 0xff; // A
            }
            rot += 0.001;
            if rot > PI * 2.0 {
                rot = 0.0;
            } else if rot < 0.0 {
                rot = PI * 2.0;
            }
            if test_object.animated {
                test_object.coords[2] += 0.1;
                if test_object.coords[2] >= 8.0 {
                    test_object.animated = false;
                }
            } else {
                test_object.coords[2] -= 0.1;
                if test_object.coords[2] <= -13.0 {
                    test_object.animated = true;
                }
            }
            scan_scene(
                &vec![&test_object],
                &player,
                pixels.frame_mut(),
                &window_size,
                &rot,
            );
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
                            wasd[1] = event.state.is_pressed();
                        }
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyD) => {
                            wasd[3] = event.state.is_pressed();
                        }
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyW) => {
                            wasd[0] = event.state.is_pressed();
                        }
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyS) => {
                            wasd[2] = event.state.is_pressed();
                        }
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyH) => {
                            wasd[4] = event.state.is_pressed();
                        }
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyL) => {
                            wasd[5] = event.state.is_pressed();
                        }
                        _ => {}
                    },
                    _ => {}
                },
                _ => (),
            }
            if wasd[1] {
                player.frustum.x -= 0.02;
                if player.frustum.x <= 0.0 {
                    player.frustum.x = 2.0 * PI;
                }
            } else if wasd[3] {
                player.frustum.x += 0.02;
                if player.frustum.x >= 2.0 * PI {
                    player.frustum.x = 0.0;
                }
            } else if wasd[0] {
                player.y += 0.3 * player.frustum.x.cos();
                player.x += 0.3 * player.frustum.x.sin();
            } else if wasd[2] {
                player.y -= 0.3 * player.frustum.x.cos();
                player.x -= 0.3 * player.frustum.x.sin();
            } else if wasd[5] {
                player.x += 0.3 * player.frustum.x.cos();
                player.y += 0.3 * player.frustum.x.sin();
            } else if wasd[4] {
                player.x -= 0.3 * player.frustum.x.cos();
                player.y -= 0.3 * player.frustum.x.sin();
            }
        })
        .unwrap();
}
