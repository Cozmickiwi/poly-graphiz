use std::cmp::min;

use winit::dpi::PhysicalSize;

/// if the aspect ratio of base_width:base_height != SCREEN_WIDTH:SCREEN_HEIGHT, problems may
/// occur.
pub struct ViewingFrustum {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub base_width: f32,
    pub base_height: f32,
}

pub struct Player {
    pub fov: u16,
    pub half_fov: u16,
    pub x: f32,
    pub y: f32,
    pub frustum: ViewingFrustum,
}

pub const SCREEN_WIDTH: u16 = 1000;
pub const SCREEN_HEIGHT: u16 = 600;
//const HALF_HEIGHT: u16 = 450;
pub const SCALE: u16 = 1;

pub const VF_DEFAULT: ViewingFrustum = ViewingFrustum {
    x: 0.0,
    y: 0.0,
    z: 0.0,
    base_width: 1.0,
    base_height: 0.6,
};

//const TESTCOLOR: [u8; 4] = [0, 27, 71, 0];
pub const BLUE1: [u8; 4] = [25, 122, 154, 0];
pub const PURPLE1: [u8; 4] = [131, 60, 169, 0];
pub const RED1: [u8; 4] = [154, 25, 70, 0];

pub fn draw_square(
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
