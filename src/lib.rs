use std::cmp::min;

use line_drawing::Bresenham;
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

pub struct Object {
    pub width: u16,
    pub height: u16,
    pub coords: [f32; 3],
}

pub const SCREEN_WIDTH: u16 = 1000;
pub const SCREEN_HEIGHT: u16 = 600;
pub const SCALE: u16 = 1;

pub const VF_DEFAULT: ViewingFrustum = ViewingFrustum {
    x: 0.0,
    y: 0.0,
    z: 0.0,
    base_width: 4.0,
    base_height: 2.4,
};

pub const TESTCOLOR: [u8; 4] = [0, 27, 71, 0];
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
        for a in 0..width {
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

pub fn draw_bresenham(
    frame: &mut [u8],
    window_size: &PhysicalSize<u32>,
    points: Vec<[i32; 2]>,
    color: [u8; 4],
) {
    for i in points {
        let (x, y) = (i[0], i[1]);
        let offset =
            (((window_size.height - y as u32) as u32 * window_size.width + x as u32) * 4) as usize;
        frame[offset] = color[0];
        frame[offset + 1] = color[1];
        frame[offset + 2] = color[2];
    }
}

const HALF_FOV: f32 = 30.0;

pub fn scan_scene(
    object_list: &Vec<&Object>,
    player: &Player,
    frame: &mut [u8],
    window_size: &PhysicalSize<u32>,
    rot: &f32,
) {
    for obj in object_list {
        println!("{}", player.frustum.x);
        let l_rad = player.frustum.x - HALF_FOV.to_radians();
        let r_rad = player.frustum.x + HALF_FOV.to_radians();
        let obj_angle = (obj.coords[0] - player.x).atan2(obj.coords[1] - player.y);
        /*
        println!("obj: {}", obj_angle.cos());
        println!("{l_rad}, {r_rad}");
        println!("l: {}", l_rad.sin());
        println!("r: {}", r_rad.sin());
        */
        let obj_x = r_rad.sin() - (obj_angle.sin() - l_rad.sin());
        //println!("obj angle: {obj_x}");
        let obj_x2 = (window_size.width as f32 / 2.0) + ((window_size.width / 2) as f32 * obj_x);
        if obj_x2 >= 0.0 && obj_x2 < window_size.width as f32 && obj_angle.cos() > l_rad.sin() {
            let distance =
                ((obj.coords[0] - player.x).powi(2) + (obj.coords[1] - player.y).powi(2)).sqrt();
            if (100.0 / (distance / 5.0)) + obj_x2 >= window_size.width as f32 {
                println!("Not in view!!");
                continue;
            }
            println!("{obj_x2}");
            let corners = find_corners(obj, *rot, distance);
            draw_corners(&corners, player, frame, window_size);
            /*
            for i in corners {
                let x = projection(window_size, player, i);
                draw_square(frame, window_size, x, y, width, height, color)
            }

            draw_square(
                frame,
                window_size,
                obj_x2 as u32,
                200,
                (100.0 / (distance / 5.0)) as usize,
                (100.0 / (distance / 5.0)) as usize,
                BLUE1,
            );*/
        } else {
            println!("Not in view!!")
        }
    }
}

pub fn draw_corners(
    corner_list: &Vec<[f32; 3]>,
    player: &Player,
    frame: &mut [u8],
    window_size: &PhysicalSize<u32>,
) {
    let mut ticker = -1;
    let mut points: Vec<[i32; 2]> = Vec::new();
    for point in corner_list {
        ticker += 1;
        let l_rad = player.frustum.x - HALF_FOV.to_radians();
        let r_rad = player.frustum.x + HALF_FOV.to_radians();
        let obj_angle = (point[0] - player.x).atan2(point[1] - player.y);
        let obj_x = r_rad.sin() - (obj_angle.sin() - l_rad.sin());
        let x2 = projection(window_size, player, *point);
        let obj_x2 = (window_size.width as f32 / 2.0) + ((window_size.width / 2) as f32 * obj_x);
        if obj_x2 >= 0.0 && obj_x2 < window_size.width as f32 && obj_angle.cos() > l_rad.sin() {
            let distance = ((point[0] - player.x).powi(2) + (point[1] - player.y).powi(2)).sqrt();
            if (100.0 / (distance / 5.0)) + obj_x2 >= window_size.width as f32 {
                continue;
            }
            let height;
            if ticker % 2 == 0 {
                height = 300.0 - distance;
            } else {
                height = 100.0 + distance;
            }
            points.push([x2 as i32, height as i32]);
            draw_square(frame, window_size, x2 as u32, height as u32, 5, 5, PURPLE1);
        }
    }
    if points.len() > 1 {
        for p in (0..points.len() - 1).step_by(2) {
            let list = bresenham_points(points[p], points[p + 1]);
            draw_bresenham(frame, window_size, list, PURPLE1);
        }
    }
    draw_bresenham(
        frame,
        window_size,
        bresenham_points(points[0], points[2]),
        PURPLE1,
    );
    draw_bresenham(
        frame,
        window_size,
        bresenham_points(points[0], points[6]),
        PURPLE1,
    );
    draw_bresenham(
        frame,
        window_size,
        bresenham_points(points[2], points[4]),
        PURPLE1,
    );
    draw_bresenham(
        frame,
        window_size,
        bresenham_points(points[4], points[6]),
        PURPLE1,
    );
    draw_bresenham(
        frame,
        window_size,
        bresenham_points(points[1], points[3]),
        PURPLE1,
    );
    draw_bresenham(
        frame,
        window_size,
        bresenham_points(points[1], points[7]),
        PURPLE1,
    );
    draw_bresenham(
        frame,
        window_size,
        bresenham_points(points[3], points[5]),
        PURPLE1,
    );
    draw_bresenham(
        frame,
        window_size,
        bresenham_points(points[5], points[7]),
        PURPLE1,
    );
}

const BASE_ALIGNED_Y: [usize; 4] = [0, 1, 6, 7];

pub fn find_corners(shape: &Object, rot: f32, distance: f32) -> Vec<[f32; 3]> {
    let mut base: Vec<[f32; 3]> = Vec::new();
    for _ in 0..8 {
        base.push([0.0, 0.0, 0.0]);
    }
    for i in 0..4 {
        base[i] = [shape.coords[0], 0.0, 0.0];
    }

    for j in 4..8 {
        base[j] = [shape.coords[0] + shape.width as f32, 0.0, 0.0];
    }
    //    let mut tick = 0;
    /*
    for x in (0..8).step_by(2) {
        base[x][1] = shape.coords[1] + (shape.width as f32 / 2.0);
        base[x + 1][1] = shape.coords[1] - (shape.width as f32 / 2.0);
    }*/
    for x in 0..8 {
        if BASE_ALIGNED_Y.binary_search(&x).is_ok() {
            base[x][1] = shape.coords[1] + shape.width as f32;
        } else {
            base[x][1] = shape.coords[1];
        }
    } /*
      for a in 0..8 {
          let new_x = (base[a][0] * rot.cos()) - (base[a][1] * rot.sin());
          let new_y = (base[a][0] * rot.sin()) + (base[a][1] * rot.cos());
          base[a][0] = new_x;
          base[a][1] = new_y;
      }*/
    println!("{:?}", base);
    base
}

fn bresenham_points(p1: [i32; 2], p2: [i32; 2]) -> Vec<[i32; 2]> {
    let mut points: Vec<[i32; 2]> = Vec::new();
    for (x, y) in Bresenham::new((p1[0], p1[1]), (p2[0], p2[1])) {
        points.push([x, y]);
    }
    points
}
const CAM: [f32; 3] = [0.0, 0.0, 0.0];
const POINT: [f32; 3] = [11.0, 10.0, 20.0];
//const SCREEN_WIDTH: u16 = 1000;
//const SCALE: f32 = 0.5;
fn projection(window_size: &PhysicalSize<u32>, player: &Player, coords: [f32; 3]) -> u32 {
    let distance = ((coords[0] - player.x).powi(2) + (coords[1] - player.y).powi(2)).sqrt();
    let angle = distance.atan2(coords[0] - player.x);
    //    println!("angle: {angle}");

    let obj_angle = coords[0].atan2(distance);
    let projected_x = ((window_size.width as f32 * angle) * 0.5) as u32 + (window_size.width / 2);
    let half_fov = (player.half_fov as f32).to_radians();
    let angle_sin = (obj_angle - player.frustum.x);
    //println!("sin: {angle_sin}");
    //println!("corner angle: {}, camera angle: {}", angle, player.frustum.x);
    let new_x = (window_size.width / 2) + (((window_size.width / 2) as f32 * angle_sin) * 0.5) as u32;
    //let new_x = (window_size.width as f32 * (((obj_angle - player.frustum.x).sin()) - 0.5)) as u32;
    //let new_x = (window_size.width as f32 * (angle - ))
    //    let total_hyp = (distance.powi(2) + (POINT[2] - CAM[2]).powi(2)).sqrt();
    //projected_x
    println!("{new_x}");
    new_x
}
