use std::{cmp::min, f32::consts::PI, time::Instant};

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

struct Raycaster {
    increment_angle: f32,
    precision: u8,
}

struct Player {
    fov: u16,
    half_fov: u16,
    x: f32,
    y: f32,
    angle: i16,
}

const MAP: [[u8; 10]; 10] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 1, 1, 0, 1, 0, 0, 1],
    [1, 0, 0, 1, 0, 0, 1, 0, 0, 1],
    [1, 0, 0, 1, 0, 0, 1, 0, 0, 1],
    [1, 0, 0, 1, 0, 1, 1, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

const SCREEN_WIDTH: u16 = 1500;
const SCREEN_HEIGHT: u16 = 900;
//const HALF_HEIGHT: u16 = 450;
const SCALE: u16 = 1;

fn main() {
    let mut player = Player {
        fov: 60,
        half_fov: 30,
        x: 1.5,
        y: 1.5,
        angle: 45,
    };
    let mut raycaster = Raycaster {
        increment_angle: player.fov as f32 / SCREEN_WIDTH as f32,
        precision: 64,
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
            print!("\r");
            let now = Instant::now();
            raycast(&player, &mut raycaster, pixels.frame_mut(), &window_size);
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
            if wasd[1] {
                player.angle -= 1;
                if player.angle < 0 {
                    player.angle = 360;
                }
            }
            if wasd[3] {
                player.angle += 1;
                if player.angle > 360 {
                    player.angle = 0;
                }
            }
            if wasd[0] {
                let rad = player.angle as f32 * (PI / 180.0);
                let psin = rad.sin() / 50.0;
                let pcos = rad.cos() / 50.0;
                let new_x = player.x + pcos;
                let new_y = player.y + psin;
                let check_x = (new_x + pcos * 20.0) as usize;
                let check_y = (new_y + psin * 20.0) as usize;
                if MAP[check_y][player.x as usize] == 0 {
                    player.y = new_y;
                }
                if MAP[player.y as usize][check_x] == 0 {
                    player.x = new_x;
                }
            }
            if wasd[2] {
                let rad = player.angle as f32 * (PI / 180.0);
                player.y -= rad.sin() / 50.0;
                player.x -= rad.cos() / 50.0;
            }
            print!("{:?}fps", (1.0 / now.elapsed().as_secs_f32()) as u32);
        })
        .unwrap();
}

//const TESTCOLOR: [u8; 4] = [0, 27, 71, 0];
const BLUE1: [u8; 4] = [25, 122, 154, 0];
const PURPLE1: [u8; 4] = [131, 60, 169, 0];
const RED1: [u8; 4] = [154, 25, 70, 0];

fn draw_square(
    frame: &mut [u8],
    window_size: &PhysicalSize<u32>,
    x: u32,
    y: u32,
    width: usize,
    height: usize,
    color: [u8; 4],
) {
    let w_height = window_size.height;
    let mut new_x: usize;
    let mut new_y: usize;
    let mut pixel_index: usize;
    for row in (0..min(height, window_size.height as usize)).rev() {
        for a in (0..width).step_by(4) {
            new_x = x as usize + a;
            new_y = w_height as usize - (y as usize + row);
            pixel_index = (new_y * window_size.width as usize + (new_x)) * 4;
            if pixel_index > frame.len() - 3 {
                break;
            }
            for i in frame[pixel_index..pixel_index + 4].chunks_exact_mut(4) {
                i[0] = color[0];
                i[1] = color[1];
                i[2] = color[2];
            }
        }
    }
}

struct Ray {
    x: f32,
    y: f32,
}

fn raycast(
    player: &Player,
    ray: &mut Raycaster,
    frame: &mut [u8],
    window_size: &PhysicalSize<u32>,
) {
    let mut ray_angle = player.angle as f32 - player.half_fov as f32;
    for i in 0..window_size.width as u16 / SCALE {
        let mut ray_struct = Ray {
            x: player.x,
            y: player.y,
        };
        // Increment
        ray_angle += ray.increment_angle * SCALE as f32;
        let ray_rad = ray_angle.to_radians();
        let pres = ray.precision as f32;
        let ray_cos = ray_rad.cos() / pres;
        let ray_sin = ray_rad.sin() / pres;
        // Wall check
        let mut wall = 0;
        while wall == 0 {
            ray_struct.x += ray_cos;
            ray_struct.y += ray_sin;
            wall = MAP[ray_struct.y as usize][ray_struct.x as usize];
        }
        let mut distance =
            ((player.x - ray_struct.x).powi(2) + (player.y - ray_struct.y).powi(2)).sqrt();
        let panglef = ray_angle - player.angle as f32;
        // Wall height
        distance = distance * (panglef.to_radians()).cos();
        let wall_height = (window_size.height / 2) as f32 / distance;
        let mut wall_start = window_size.height / 2 - wall_height as u32;
        if distance <= 1.0 {
            wall_start = 1;
        }
        //        let true_height = wall_height as u32 * 2;
        draw_square(
            frame,
            window_size,
            i as u32 * SCALE as u32,
            wall_start - 1,
            SCALE as usize,
            wall_height as usize * 2,
            PURPLE1,
        );
        draw_square(
            frame,
            window_size,
            i as u32 * SCALE as u32,
            0,
            SCALE as usize,
            wall_start as usize,
            BLUE1,
        );
        draw_square(
            frame,
            window_size,
            i as u32 * SCALE as u32,
            window_size.height - wall_start - 1,
            SCALE as usize,
            //            wall_height as usize - (wall_start + (wall_height as u32 * 2)) as usize,
            wall_start as usize + 4,
            RED1,
        );
    }
}
