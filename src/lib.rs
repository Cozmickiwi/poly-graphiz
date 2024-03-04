use line_drawing::Bresenham;
use nalgebra::{ComplexField, Matrix3, Point3, Rotation3, Translation3, Vector2, Vector3};
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

pub struct Object {
    pub width: u16,
    pub height: u16,
    pub coords: [f32; 3],
    pub animated: bool,
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
        //   println!("{}", player.frustum.x);

        let l_rad = player.frustum.x - HALF_FOV.to_radians();
        let r_rad = player.frustum.x + HALF_FOV.to_radians();
        let obj_angle = (obj.coords[0] - player.x).atan2(obj.coords[1] - player.y);
        let obj_x = r_rad.sin() - (obj_angle.sin() - l_rad.sin());
        let obj_ax =
            ((obj.coords[0] - player.x).powi(2) + (obj.coords[1] - player.y).powi(2)).sqrt();
        let player_hyp = (player.x.powi(2) + player.y.powi(2)).sqrt();
        let ax_angle = ((window_size.width / 2) as f32
            + ((window_size.width / 2) as f32
                * (((obj_ax.atan2(obj.coords[0] - player.x)) - player.frustum.x).cos() * -1.0)))
            as u32;
        /*
        println!(
            "{}",
            ((obj.coords[0] - player.x).atan2(obj_ax) - player.frustum.x).to_degrees()
        );*/
        let deg = ((obj.coords[0] - player.x).atan2(obj_ax) - player.frustum.x).to_degrees();
        let obj_x2 = (window_size.width as f32 / 2.0) + ((window_size.width / 2) as f32 * obj_x);
        if ax_angle < window_size.width && obj_angle.cos() > l_rad.sin() {
            let distance =
                ((obj.coords[0] - player.x).powi(2) + (obj.coords[1] - player.y).powi(2)).sqrt();
            if (100.0 / (distance / 5.0)) + ax_angle as f32 >= window_size.width as f32 {
                //                println!("Not in view!!");
                continue;
            }
            //            println!("{obj_x2}");
            let corners = find_corners(obj, *rot);
            draw_corners(&corners, player, frame, window_size, ax_angle);
        } else {
            println!("Not in view!!asda")
        }
    }
}

pub fn draw_corners(
    corner_list: &Vec<[f32; 3]>,
    player: &Player,
    frame: &mut [u8],
    window_size: &PhysicalSize<u32>,
    ax2: u32,
) {
    let mut ticker = -1;
    let mut points: Vec<[i32; 2]> = Vec::new();
    //  println!("{:?}", corner_list);
    for point in corner_list {
        ticker += 1;
        let l_rad = player.frustum.x - HALF_FOV.to_radians();
        let r_rad = player.frustum.x + HALF_FOV.to_radians();
        let obj_angle = (point[0] - player.x).atan2(point[1] - player.y);
        let obj_x = r_rad.sin() - (obj_angle.sin() - l_rad.sin());
        let x2;
        if let Some(x) = projection(window_size, player, *point) {
            x2 = x;
        } else {
            println!("None!");
            return;
        }
        let obj_x2 = (window_size.width as f32 / 2.0) + ((window_size.width / 2) as f32 * obj_x);
        if ax2 >= 0 && ax2 < window_size.width
        /*&& obj_angle.cos() > l_rad.sin()*/
        {
            let distance = ((point[0] - player.x).powi(2) + (point[1] - player.y).powi(2)).sqrt();
            if (100.0 / (distance / 5.0)) + ax2 as f32 >= window_size.width as f32 {
                continue;
            } /*
              let height;
              if ticker % 2 == 0 {
                  height = 300.0 - distance;
              } else {
                  height = 150.0 + distance;
              }
              */
            points.push([x2[0] as i32, x2[1] as i32]);
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
    if points.len() > 1 {
        for p in (0..points.len() - 1).step_by(2) {
            let list = bresenham_points(points[p], points[p + 1]);
            draw_bresenham(frame, window_size, list, PURPLE1);
        }
    }
    if points.len() < 8 {
        return;
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
    // fill sides
    /*
    fill_bresenham(
        [
            bresenham_points(points[0], points[1]),
            bresenham_points(points[2], points[3]),
        ],
        frame,
        window_size,
        PURPLE1,
    );
    fill_bresenham(
        [
            bresenham_points(points[0], points[2]),
            bresenham_points(points[4], points[6]),
        ],
        frame,
        window_size,
        [0, 128, 128, 255],
    );
    fill_bresenham(
        [
            bresenham_points(points[1], points[7]),
            bresenham_points(points[3], points[5]),
        ],
        frame,
        window_size,
        [255, 0, 255, 255],
    );
    fill_bresenham(
        [
            bresenham_points(points[0], points[6]),
            bresenham_points(points[1], points[7]),
        ],
        frame,
        window_size,
        [0, 0, 128, 255],
    );
    fill_bresenham(
        [
            bresenham_points(points[2], points[4]),
            bresenham_points(points[3], points[5]),
        ],
        frame,
        window_size,
        [0, 128, 0, 255],
    );
    fill_bresenham(
        [
            bresenham_points(points[4], points[5]),
            bresenham_points(points[6], points[7]),
        ],
        frame,
        window_size,
        [128, 0, 0, 255],
    );
    */
}

pub fn fill_bresenham(
    points: [Vec<[i32; 2]>; 2],
    frame: &mut [u8],
    window_size: &PhysicalSize<u32>,
    color: [u8; 4],
) {
    let vec1: &Vec<[i32; 2]>;
    let vec2: &Vec<[i32; 2]>;
    let chosen_vec: usize;
    let smaller_vec: usize;
    if points[0].len() > points[1].len() {
        chosen_vec = 0;
        smaller_vec = 1;
    } else {
        chosen_vec = 1;
        smaller_vec = 0;
    }
    vec1 = &points[chosen_vec];
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
    /*
    let normalized_x = Vector2::from_vec(xvec).normalize() * 10.0;
    println!("{:?}", normalized_x);
    println!("{:?}", vec1);
    let normalized_y = Vector2::from_vec(yvec).normalize() * vec1.len() as f32;
    let mut new_vec: Vec<[i32; 2]> = Vec::new();
    for j in 0..vec1.len() {
        new_vec.push([normalized_x[j] as i32, normalized_y[j] as i32]);
    }
    vec2 = &new_vec;
    */
    let mut new_vec2 = Vec::new();
    let scale = points[smaller_vec].len() as f32 / vec1.len() as f32;
    for i in 0..vec1.len() {
        let n = (i as f32 * scale) as usize;
        new_vec2.push(points[smaller_vec][n].clone());
    }
    //    println!("{:?}", new_vec2);
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
    let center_x = shape.coords[0] + (shape.width as f32 / 2.0);
    let center_y = shape.coords[1] + (shape.width as f32 / 2.0);
    //    let center_z = shape.coords[2] + (shape.width as f32 / 2.0);
    // Rotation
    let rot2 = rot * 2.0;
    //    let mut cube_corners = Vec::new();
    for i in 0..8 {
        if i % 2 == 0 {
            base[i][2] = shape.coords[2] + shape.width as f32;
        } else {
            base[i][2] = shape.coords[2];
        }
    }
    base = find_corners_2(shape, rot);
    /*
    for a in 0..8 {
        cube_corners.push(Point3::new(base[a][0], base[a][1], base[a][2]));
        let new_x =
            center_x + (base[a][0] - center_x) * rot2.cos() - (base[a][1] - center_y) * rot2.sin();
        let new_y =
            center_y + (base[a][0] - center_x) * rot2.sin() + (base[a][1] - center_y) * rot2.cos();
        base[a][0] = new_x;
        base[a][1] = new_y;
    }
    let rotation_matrix = Matrix3::new(
        rot.cos(),
        0.0,
        -rot.sin(),
        0.0,
        1.0,
        0.0,
        rot.sin(),
        0.0,
        rot.cos(),
    );
    let rotated_points: Vec<Point3<f32>> = cube_corners
        .iter()
        .map(|&point| rotation_matrix * point.coords)
        .map(|coords| Point3::from(coords))
        .collect();

    base = rotated_points
        .iter()
        .map(|point| [point.x, point.y, point.z])
        .collect();
    */
    /*
    for i in 0..8 {
        let point = &base[i];
        let rot_angle = [
            point[0] * rot.cos() - point[2] * rot.sin(),
            point[1],
            point[0] * rot.sin() - point[2] * rot.cos(),
        ];
        base[i] = rot_angle;
    }

    let rotated_cube = base
        .iter()
        .map(|&[x, y, z]| {
            let y_rotated = y * rot.cos() - z * rot.sin();
            let z_rotated = y * rot.sin() + z * rot.cos();
            let x_yaw = x * rot.cos() + z_rotated * rot.sin();
            let z_yaw = -x * rot.sin() + z_rotated * rot.cos();
            [x_yaw, y, z_yaw]
        })
        .collect();

    // Orbit

    for a in 0..8 {
        let new_x = base[a][0] * rot.cos() - base[a][1] * rot.sin();
        let new_y = base[a][0] * rot.sin() + base[a][1] * rot.cos();
        base[a][0] = new_x;
        base[a][1] = new_y;
    }*/

    //println!("{:?}", base);
    base
    //rotated_cube
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
) -> Option<[u32; 2]> {
    let distance = ((coords[0] - player.x).powi(2) + (coords[1] - player.y).powi(2)).sqrt();
    let obj_angle = coords[0].atan2(distance);
    let angle_sin = obj_angle - player.frustum.x;
    let new_x = ((window_size.width / 2) as f32
        + ((window_size.width / 2) as f32 * angle_sin.sin())) as u32;
    //  println!("{}", angle_sin.sin());
    let distance2 = (distance.powi(2) + coords[2].powi(2)).sqrt();
    let ydeg = (coords[2].atan2(distance2)).to_degrees();
    let new_y;
    if ydeg < 35.0 && ydeg > -35.0 {
        new_y = ((window_size.height / 2) as f32
            + ((window_size.height / 2) as f32 * (ydeg / 35.0))) as u32;
    } else {
        return None;
    }
    return Some([new_x, new_y]);
}

fn find_corners_2(shape: &Object, rot: f32) -> Vec<[f32; 3]> {
    let mut base = Vec::new();
    let half_width = shape.width as f32 / 2.0;
    let center: [f32; 3] = [
        shape.coords[0] + half_width,
        shape.coords[1] + half_width,
        shape.coords[2] + half_width,
    ];
    //    let rot = 45.0_f32.to_radians();
    let sine1 = 225.0_f32.to_radians().sin();
    let sine2 = 45.0_f32.to_radians().sin();
    let pos_scale = half_width * sine2;
    let neg_scale = half_width * sine1;
    let mut angle_vec: Vec<[f32; 3]> = Vec::new();
    for i in 0..8 {
        let px;
        let py;
        let pz;
        let mut angles = vec![0.0, 0.0, 0.0];
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
        //   angle_vec.push([0.0, 0.0, 0.0]);
    }
    angle_vec = vec![
        // Pitch, Roll, Yaw
        [315.0, 315.0, 315.0],
        [225.0, 225.0, 315.0],
        [45.0, 315.0, 225.0],
        [135.0, 225.0, 225.0],
        [45.0, 45.0, 135.0],
        [135.0, 135.0, 135.0],
        [315.0, 45.0, 45.0],
        [225.0, 135.0, 45.0],
    ];
    return rotate_cube(&base, rot);
    /*
    for x in 0..8 {
        // Roll
        base[x][0] = center[0] + ((angle_vec[x][1].to_radians() + rot).sin() * half_width);
        base[x][2] = center[2] + ((angle_vec[x][1].to_radians() + rot).cos() * half_width);
        // Yaw
        /*
        base[x][0] = center[0] + ((angle_vec[x][2].to_radians() + rot).sin() * half_width);
        base[x][1] = center[1] + ((angle_vec[x][2].to_radians() + rot).cos() * half_width);
        // Pitch

        base[x][1] = center[1] + ((angle_vec[x][0].to_radians() + rot).sin() * half_width);
        base[x][2] = center[2] + ((angle_vec[x][0].to_radians() + rot).cos() * half_width);

        let x_translated = base[x][0] + center[0];
        let y_translated = base[x][1] + center[1];
        let z_translated = base[x][2] + center[2];
        let x_rotated = x_translated * rot.cos() - z_translated * rot.sin();
        let z_rotated = x_rotated * rot.sin() + z_translated * rot.cos();
        base[x][0] = x_rotated - center[0];
        base[x][2] = z_rotated - center;
        */
    }
        println!("{:?}", base);
    let base_y = shape.coords[1] + shape.width as f32;
    for x in 0..8 {
        let x_angle = base_y.atan2(base[x][0]);
        let shape_x_angle = base_y.atan2(center[0]);
        let rel_x_angle = (shape_x_angle.cos() - x_angle.cos()).cosh();
        println!("{}", j);
    }*/

    //    base
}

//[5, 13, 5]

fn rotate_cube(corner_list: &Vec<[f32; 3]>, rot: f32) -> Vec<[f32; 3]> {
    let center = Point3::new(5.0, 13.0, 5.0);
    let axis = Vector3::y_axis();
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
            //            let v = Vector3::new(corner[0], corner[1], corner[2]);
            let rotated_point =
                rotation_matrix.transform_point(&Point3::new(corner[0], corner[1], corner[2]));
            let translation_back = Translation3::from(center.coords);
            //rotation_matrix * v
            let t = translation_back * rotated_point;
            return [t.x, t.y, t.z];
        })
        .collect();
    /*
    let mut v2: Vec<[f32; 3]> = Vec::new();
    for i in rotated_points {
        v2.push([i[0], i[1], i[2]]);
    }*/
    rotated_points
    //    println!("{:?}", rotated_points);
}
