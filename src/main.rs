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
        height: 30,
        coords: [0.0, 8.0, 0.0],
        animated: false,
    };
    let test_object2 = Object {
        width: 5,
        height: 30,
        coords: [3.0, -8.0, 2.0],
        animated: false,
    };

    let test_object3 = Object {
        width: 5,
        height: 30,
        coords: [9.0, 3.0, -2.0],
        animated: false,
    }; /*
       let test_object4 = Object {
           width: 5,
           height: 30,
           coords: [-3.0, 0.0, 1.0],
       };*/
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
        pixel[0] = 255; // R
        pixel[1] = 27; // G
        pixel[2] = 71; // B
        pixel[3] = 0xff; // A
    }
    let mut wasd: [bool; 6] = [false, false, false, false, false, false];
    event_loop
        .run(move |event, elwt| {
            for pixel in pixels.frame_mut().chunks_exact_mut(4) {
                pixel[0] = 255; // R
                pixel[1] = 97; // G
                pixel[2] = 71; // B
                pixel[3] = 0xff; // A
            }

            rot += 0.01;
            if rot > PI * 2.0 {
                rot = 0.0;
            } else if rot < 0.0 {
                rot = PI * 2.0;
            }
            /*
            if test_object.animated {
                test_object.coords[2] += 0.1;
                if test_object.coords[2] >= 8.0 {
                    test_object.animated = false;
                }
            } else {
                test_object.coords[2] -= 0.1;
                if test_object.coords[2] <= -8.0 {
                    test_object.animated = true;
                }
            }*/
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
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyH) => {
                            if event.state.is_pressed() {
                                wasd[4] = true;
                            } else {
                                wasd[4] = false
                            }
                        }
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyL) => {
                            if event.state.is_pressed() {
                                wasd[5] = true;
                            } else {
                                wasd[5] = false
                            }
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
            } else if wasd[4] {
                player.x += 0.1 * player.frustum.x.cos();
                player.y += 0.1 * player.frustum.x.sin();
            } else if wasd[5] {
                player.x -= 0.1 * player.frustum.x.cos();
                player.y -= 0.1 * player.frustum.x.sin();
            }
        })
        .unwrap();
}
