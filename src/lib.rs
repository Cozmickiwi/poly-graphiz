use line_drawing::Bresenham;
use nalgebra::{Point3, Rotation3, Translation3, Vector3};
use std::{cmp::min, fs::File, io::Read};
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
    pub animated: bool,
    pub vertices: Vec<[f32; 3]>,
    pub indices: Vec<Vec<usize>>,
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
            if new_x > window_size.width as usize {
                break;
            }
            new_y = w_height as usize - (y as usize + row);
            if new_y >= window_size.height as usize {
                return;
            }
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
        if x >= window_size.width as i32 || x < 0 || y <= 0 || y >= window_size.height as i32 {
            continue;
        }
        let offset =
            (((window_size.height - y as u32) * window_size.width + x as u32) * 4) as usize;
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
        let l_rad = player.frustum.x - HALF_FOV.to_radians();
        let obj_angle = (obj.coords[0] - player.x).atan2(obj.coords[1] - player.y);
        let obj_ax =
            ((obj.coords[0] - player.x).powi(2) + (obj.coords[1] - player.y).powi(2)).sqrt();
        let rel_angle =
            (obj.coords[0] - player.x).atan2(obj.coords[1] - player.y) - player.frustum.x;
        let rel_angle2 = ((obj.coords[0] + obj.width as f32) - player.x)
            .atan2((obj.coords[1] + obj.width as f32) - player.y)
            - player.frustum.x;
        if rel_angle2.sin() < -0.5 || rel_angle.sin() > 0.5 || rel_angle.cos() < 0.0 {
            continue;
        }
        let ax_angle = ((window_size.width / 2) as f32
            + ((window_size.width / 2) as f32
                * (((obj_ax.atan2(obj.coords[0] - player.x)) - player.frustum.x).cos() * -1.0)))
            as u32;
        if ax_angle < window_size.width && obj_angle.cos() > l_rad.sin() {
            let distance =
                ((obj.coords[0] - player.x).powi(2) + (obj.coords[1] - player.y).powi(2)).sqrt();
            if (100.0 / (distance / 5.0)) + ax_angle as f32 >= window_size.width as f32 {
                continue;
            }
            //            let corners = find_corners(obj, *rot);
            draw_corners(obj, player, frame, window_size, ax_angle, *rot);
        } else {
            println!("Not in view!!asda")
        }
    }
}

pub fn draw_corners(
    shape: &Object,
    player: &Player,
    frame: &mut [u8],
    window_size: &PhysicalSize<u32>,
    ax2: u32,
    rot: f32,
) {
    let mut points: Vec<[i32; 2]> = Vec::new();
    let rot_v = rotate_cube(&shape.vertices, rot, 'z', shape);
    for point in &rot_v {
        let x2;
        if let Some(x) = projection(window_size, player, *point) {
            x2 = x;
        } else {
            println!("None!");
            return;
        }
        if ax2 < window_size.width {
            let distance = ((point[0] - player.x).powi(2) + (point[1] - player.y).powi(2)).sqrt();
            if (100.0 / (distance / 5.0)) + ax2 as f32 >= window_size.width as f32 {
                continue;
            }
            points.push([x2[0], x2[1]]);

            draw_square(
                frame,
                window_size,
                x2[0] as u32 - 2,
                x2[1] as u32 - 2,
                5,
                5,
                PURPLE1,
            );
        } else {
            println!("Not in view!");
        }
    }
    /*
    if points.len() > 1 {
        for p in (0..points.len() - 1).step_by(2) {
            let list = bresenham_points(points[p], points[p + 1]);
            draw_bresenham(frame, window_size, list, PURPLE1);
        }
    }*/
    for ind in &shape.indices {
        for i in 0..ind.len() - 1 {
            let list = bresenham_points(points[ind[i]], points[ind[i + 1]]);
            draw_bresenham(frame, window_size, list, PURPLE1);
        }
        let list = bresenham_points(points[ind[ind.len() - 1]], points[ind[0]]);
        draw_bresenham(frame, window_size, list, PURPLE1);
    }
    if points.len() < 8 {
    }
    /*
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
    );*/
}

pub fn fill_bresenham(
    points: [Vec<[i32; 2]>; 2],
    frame: &mut [u8],
    window_size: &PhysicalSize<u32>,
    color: [u8; 4],
) {
    
    let chosen_vec: usize;
    let smaller_vec: usize;
    if points[0].len() > points[1].len() {
        chosen_vec = 0;
        smaller_vec = 1;
    } else {
        chosen_vec = 1;
        smaller_vec = 0;
    }
    let vec1: &Vec<[i32; 2]> = &points[chosen_vec];
    let mut tempvec = Vec::new();
    for i in &points[smaller_vec] {
        let tempvec2 = [i[0] as f32, i[1] as f32];
        tempvec.push(tempvec2);
    }
    let mut xvec = Vec::new();
    let mut yvec = Vec::new();
    for i in tempvec {
        xvec.push(i[0]);
        yvec.push(i[1]);
    }
    let mut new_vec2 = Vec::new();
    let scale = points[smaller_vec].len() as f32 / vec1.len() as f32;
    for i in 0..vec1.len() {
        let n = (i as f32 * scale) as usize;
        new_vec2.push(points[smaller_vec][n]);
    }
    for x in 0..vec1.len() {
        draw_bresenham(
            frame,
            window_size,
            bresenham_points(vec1[x], new_vec2[x]),
            color,
        );
    }
}

const BASE_ALIGNED_Y: [usize; 4] = [0, 1, 6, 7];

pub fn find_corners(shape: &Object, rot: f32) -> Vec<[f32; 3]> {
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
    for x in 0..8 {
        if BASE_ALIGNED_Y.binary_search(&x).is_ok() {
            base[x][1] = shape.coords[1] + shape.width as f32;
        } else {
            base[x][1] = shape.coords[1];
        }
    }
    for i in 0..8 {
        if i % 2 == 0 {
            base[i][2] = shape.coords[2] + shape.width as f32;
        } else {
            base[i][2] = shape.coords[2];
        }
    }
    base = find_corners_2(shape, rot);
    base
}

const WIRE_THICKNESS: i32 = 2;

fn bresenham_points(p1: [i32; 2], p2: [i32; 2]) -> Vec<[i32; 2]> {
    let mut points: Vec<[i32; 2]> = Vec::new();
    for i in 0..WIRE_THICKNESS {
        for (x, y) in Bresenham::new((p1[0] - i, p1[1] - i), (p2[0] - i, p2[1] - i)) {
            points.push([x, y]);
        }
    }
    points
}

fn projection(
    window_size: &PhysicalSize<u32>,
    player: &Player,
    coords: [f32; 3],
) -> Option<[i32; 2]> {
    let distance = ((coords[0] - player.x).powi(2) + (coords[1] - player.y).powi(2)).sqrt();
    let obj_angle = (coords[0] - player.x).atan2(coords[1] - player.y);
    let angle_sin = obj_angle - player.frustum.x;
    let new_x = (window_size.width as f32 * (angle_sin.sin() + 0.5)) as i32;
    let distance2 = (distance.powi(2) + coords[2].powi(2)).sqrt();
    let x = (distance2.atan2(coords[2]) + 90.0_f32.to_radians()).sin();
    let new_y = (window_size.height as f32
        * ((x * (SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32)) + 0.5)) as i32;
    Some([new_x, new_y])
}

fn find_corners_2(shape: &Object, rot: f32) -> Vec<[f32; 3]> {
    let mut base = Vec::new();
    let half_width = shape.width as f32 / 2.0;
    let center: [f32; 3] = [
        shape.coords[0] + half_width,
        shape.coords[1] + half_width,
        shape.coords[2] + half_width,
    ];
    let sine1 = 225.0_f32.to_radians().sin();
    let sine2 = 45.0_f32.to_radians().sin();
    let pos_scale = half_width * sine2;
    let neg_scale = half_width * sine1;
    for i in 0..8 {
        let px;
        let py;
        let pz;
        if i < 4 {
            px = center[0] + neg_scale;
        } else {
            px = center[0] + pos_scale;
        }
        if i % 2 == 0 {
            pz = center[2] + pos_scale;
        } else {
            pz = center[2] + neg_scale;
        }
        if BASE_ALIGNED_Y.contains(&(i as usize)) {
            py = center[1] + pos_scale;
        } else {
            py = center[1] + neg_scale;
        }
        base.push([px, py, pz]);
    }
    base = rotate_cube(&rotate_cube(&base, rot, 'y', shape), rot, 'x', shape);
    //    base = rotate_cube(&base, rot, 'y');
    base
}

//[5, 13, 5]

fn rotate_cube(corner_list: &Vec<[f32; 3]>, rot: f32, ax: char, _shape: &Object) -> Vec<[f32; 3]> {
    let center = Point3::new(
        //shape.coords[0] + (shape.width as f32 / 2.0),
        //shape.coords[1] + (shape.width as f32 / 2.0),
        //shape.coords[2] + (shape.width as f32 / 2.0),
        0.0, 0.0, 0.0,
    );
    let axis;
    if ax == 'x' {
        axis = Vector3::x_axis();
    } else if ax == 'y' {
        axis = Vector3::y_axis();
    } else if ax == 'z' {
        axis = Vector3::z_axis();
    } else {
        panic!();
    }
    let origin_translation = Translation3::from(-center.coords);
    let rotation_matrix = Rotation3::from_axis_angle(&axis, rot);
    let mut v1: Vec<[f32; 3]> = Vec::new();
    for i in corner_list {
        let point = Point3::new(i[0], i[1], i[2]);
        let translated_point = origin_translation * point;
        v1.push([translated_point.x, translated_point.y, translated_point.z]);
    }
    let rotated_points: Vec<[f32; 3]> = v1
        .iter()
        .map(|&corner| {
            let rotated_point =
                rotation_matrix.transform_point(&Point3::new(corner[0], corner[1], corner[2]));
            let translation_back = Translation3::from(center.coords);
            let t = translation_back * rotated_point;
            [t.x, t.y, t.z]
        })
        .collect();
    rotated_points
}

pub fn parse_obj() -> (Vec<[f32; 3]>, Vec<Vec<usize>>) {
    let mut file = File::open("../sphere1.obj").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    for line in buffer.lines() {
        let split_line: Vec<&str> = line.split_whitespace().collect();
        if split_line[0] == "v" {
            let x: f32 = split_line[1].parse().unwrap();
            let y: f32 = split_line[2].parse().unwrap();
            let z: f32 = split_line[3].parse().unwrap();
            vertices.push([x, y, z]);
        } else if split_line[0] == "f" {
            let mut i_list = Vec::new();
            for i in 1..split_line.len() {
                let vs: Vec<&str> = split_line[i].split('/').collect();
                let v: usize = vs[0].parse().unwrap();
                i_list.push(v - 1);
            }
            indices.push(i_list);
        }
    }
    (vertices, indices)
}
