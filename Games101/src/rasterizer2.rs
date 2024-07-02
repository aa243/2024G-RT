use std::collections::HashMap;

use nalgebra::{Matrix4, Vector3, Vector4};
use crate::triangle::Triangle;

#[allow(dead_code)]
pub enum Buffer {
    Color,
    Depth,
    Both,
}

#[allow(dead_code)]
pub enum Primitive {
    Line,
    Triangle,
}

#[derive(Default, Clone)]
pub struct Rasterizer {
    model: Matrix4<f64>,
    view: Matrix4<f64>,
    projection: Matrix4<f64>,
    pos_buf: HashMap<usize, Vec<Vector3<f64>>>,
    ind_buf: HashMap<usize, Vec<Vector3<usize>>>,
    col_buf: HashMap<usize, Vec<Vector3<f64>>>,

    frame_buf: Vec<Vector3<f64>>,
    depth_buf: Vec<f64>,
    /*  You may need to uncomment here to implement the MSAA method  */
    frame_sample: Vec<Vector3<f64>>,
    depth_sample: Vec<f64>,
    width: u64,
    height: u64,
    next_id: usize,
}

#[derive(Clone, Copy)]
pub struct PosBufId(usize);

#[derive(Clone, Copy)]
pub struct IndBufId(usize);

#[derive(Clone, Copy)]
pub struct ColBufId(usize);

impl Rasterizer {
    pub fn new(w: u64, h: u64) -> Self {
        let mut r = Rasterizer::default();
        r.width = w;
        r.height = h;
        r.frame_buf.resize((w * h) as usize, Vector3::zeros());
        r.depth_buf.resize((w * h) as usize, 0.0);

        // 4*MSAA 
        r.frame_sample.resize((w * h * 4) as usize, Vector3::zeros());
        r.depth_sample.resize((w * h * 4) as usize, 0.0);
        r
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        ((self.height - 1 - y as u64) * self.width + x as u64) as usize
    }
    // For MSAA
    // Return the index of the left-upper sample point of the pixel 
    fn get_sample_index(&self, x: usize, y: usize) -> usize {
        ((self.height - 1 - y as u64) * self.width + x as u64) as usize * 4
    }

    fn set_sample(&mut self, index :usize, color: &Vector3<f64>) {
        self.frame_sample[index] = *color;
    }

    fn set_pixel(&mut self, point: &Vector3<f64>, color: &Vector3<f64>) {
        let ind = (self.height as f64 - 1.0 - point.y) * self.width as f64 + point.x;
        self.frame_buf[ind as usize] = *color;
    }

    pub fn clear(&mut self, buff: Buffer) {
        match buff {
            Buffer::Color => {
                self.frame_buf.fill(Vector3::new(0.0, 0.0, 0.0));
                self.frame_sample.fill(Vector3::new(0.0, 0.0, 0.0));
            }
            Buffer::Depth => {
                self.depth_buf.fill(f64::MAX);
                self.depth_sample.fill(f64::MAX);
            }
            Buffer::Both => {
                self.frame_buf.fill(Vector3::new(0.0, 0.0, 0.0));
                self.depth_buf.fill(f64::MAX);
                self.frame_sample.fill(Vector3::new(0.0, 0.0, 0.0));
                self.depth_sample.fill(f64::MAX);
            }
        }
    }

    pub fn set_model(&mut self, model: Matrix4<f64>) {
        self.model = model;
    }

    pub fn set_view(&mut self, view: Matrix4<f64>) {
        self.view = view;
    }

    pub fn set_projection(&mut self, projection: Matrix4<f64>) {
        self.projection = projection;
    }

    fn get_next_id(&mut self) -> usize {
        let res = self.next_id;
        self.next_id += 1;
        res
    }

    pub fn load_position(&mut self, positions: &Vec<Vector3<f64>>) -> PosBufId {
        let id = self.get_next_id();
        self.pos_buf.insert(id, positions.clone());
        PosBufId(id)
    }

    pub fn load_indices(&mut self, indices: &Vec<Vector3<usize>>) -> IndBufId {
        let id = self.get_next_id();
        self.ind_buf.insert(id, indices.clone());
        IndBufId(id)
    }

    pub fn load_colors(&mut self, colors: &Vec<Vector3<f64>>) -> ColBufId {
        let id = self.get_next_id();
        self.col_buf.insert(id, colors.clone());
        ColBufId(id)
    }

    pub fn draw(&mut self, pos_buffer: PosBufId, ind_buffer: IndBufId, col_buffer: ColBufId, _typ: Primitive) {
        let buf = &self.clone().pos_buf[&pos_buffer.0];
        let ind: &Vec<Vector3<usize>> = &self.clone().ind_buf[&ind_buffer.0];
        let col = &self.clone().col_buf[&col_buffer.0];

        let f1 = (50.0 - 0.1) / 2.0;
        let f2 = (50.0 + 0.1) / 2.0;

        let mvp = self.projection * self.view * self.model;

        for i in ind {
            let mut t = Triangle::new();
            let mut v =
                vec![mvp * to_vec4(buf[i[0]], Some(1.0)), // homogeneous coordinates
                     mvp * to_vec4(buf[i[1]], Some(1.0)), 
                     mvp * to_vec4(buf[i[2]], Some(1.0))];
    
            for vec in v.iter_mut() {
                *vec = *vec / vec.w;
            }
            for vert in v.iter_mut() {
                vert.x = 0.5 * self.width as f64 * (vert.x + 1.0);
                vert.y = 0.5 * self.height as f64 * (vert.y + 1.0);
                vert.z = vert.z * f1 + f2;
            }
            for j in 0..3 {
                // t.set_vertex(j, Vector3::new(v[j].x, v[j].y, v[j].z));
                t.set_vertex(j, v[j]);
                t.set_vertex(j, v[j]);
                t.set_vertex(j, v[j]);
            }
            let col_x = col[i[0]];
            let col_y = col[i[1]];
            let col_z = col[i[2]];
            t.set_color(0, col_x[0], col_x[1], col_x[2]);
            t.set_color(1, col_y[0], col_y[1], col_y[2]);
            t.set_color(2, col_z[0], col_z[1], col_z[2]);

            self.rasterize_triangle(&t);
            // return;
        }
    }

    pub fn rasterize_triangle(&mut self, t: &Triangle) {
        /*  implement your code here  */
        
        // AABB
        println!("Here");
        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;
        for i in 0..3 {
            min_x = min_x.min(t.v[i].x);
            min_y = min_y.min(t.v[i].y);
            max_x = max_x.max(t.v[i].x);
            max_y = max_y.max(t.v[i].y);
        }
        
        // Rendering
        // Using 4*MSAA
        // let orient = [(0.25,0.25),(0.75,0.25),(0.25,0.75),(0.75,0.75)];
        // for index_x in min_x as usize.. max_x.ceil() as usize {
        //     for index_y in min_y as usize.. max_y.ceil() as usize {
        //         let index = self.get_sample_index(index_x, index_y);
        //         let mut changed = false;
        //         for i in 0..4{
        //             let x = index_x as f64 + orient[i].0;
        //             let y = index_y as f64 + orient[i].1;
        //             if inside_triangle(x as f64, y as f64, &t.v) {
        //                 let (c1, c2, c3) = compute_barycentric2d(x, y, &t.v);
        //                 let z = c1 * t.v[0].z + c2 * t.v[1].z + c3 * t.v[2].z;
        //                 let color = t.get_color();
        //                 if z < self.depth_sample[index + i] {
        //                     changed = true;
        //                     self.depth_sample[index + i] = z;
        //                     self.set_sample(index + i , &color);
        //                 }
        //             }
        //         }
        //         if changed{
        //             let mut pixel_color = Vector3::new(0.0, 0.0, 0.0);
        //             for j in 0..4 {
        //                 pixel_color += self.frame_sample[index + j];
        //             }
        //             pixel_color /= 4.0;
        //             self.set_pixel(&Vector3::new(index_x as f64, index_y as f64, 0.0), &pixel_color);
        //         }
        //     }
        // }

        let orient = [(0.25,0.25),(0.75,0.25),(0.25,0.75),(0.75,0.75)];
        for index_x in min_x as usize.. max_x.ceil() as usize {
            for index_y in min_y as usize.. max_y.ceil() as usize {
                let index = self.get_sample_index(index_x, index_y);
                let mut changed = false;
                if all_in_out_triangle(index_x as f64 + 0.5, index_y as f64 + 0.5, &t.v){
                    let x = index_x as f64 + 0.5;
                    let y = index_y as f64 + 0.5;
                    if inside_triangle(x, y, &t.v){
                        let (c1, c2, c3) = compute_barycentric2d(x, y, &t.v);
                        let z = c1 * t.v[0].z + c2 * t.v[1].z + c3 * t.v[2].z;
                        let color = t.get_color();
                        for i in 0..4{
                            if z < self.depth_sample[index + i] {
                                changed = true;
                                self.depth_sample[index + i] = z;
                                self.set_sample(index + i , &color);
                            }
                        }
                    }
                }
                else{
                    for i in 0..4{
                        let x = index_x as f64 + orient[i].0;
                        let y = index_y as f64 + orient[i].1;
                        if inside_triangle(x as f64, y as f64, &t.v) {
                            let (c1, c2, c3) = compute_barycentric2d(x, y, &t.v);
                            let z = c1 * t.v[0].z + c2 * t.v[1].z + c3 * t.v[2].z;
                            let color = t.get_color();
                            if z < self.depth_sample[index + i] {
                                changed = true;
                                self.depth_sample[index + i] = z;
                                self.set_sample(index + i , &color);
                            }
                        }
                    }
                }
                if changed{
                    let mut pixel_color = Vector3::new(0.0, 0.0, 0.0);
                    for j in 0..4 {
                        pixel_color += self.frame_sample[index + j];
                    }
                    pixel_color /= 4.0;
                    self.set_pixel(&Vector3::new(index_x as f64, index_y as f64, 0.0), &pixel_color);
                }
            }
        }

        // Rendering
        // Using 1*MSAA
        // let mut sample: Vec<u32> = Vec::with_capacity(4);
        // sample.resize(((self.width + 1) * (self.height + 1)) as usize, u32::MAX);
        // let orient = [(0,0),(1,0),(0,1),(1,1)];
        // for index_x in min_x as usize.. max_x.ceil() as usize {
        //     for index_y in min_y as usize.. max_y.ceil() as usize {
        //         let mut changed = false;
        //         for i in 0..4{
        //             let x = index_x + orient[i].0;
        //             let y = index_y + orient[i].1;
        //             let index = y * (self.width + 1) as usize+ x;
        //             if sample[index] == u32::MAX {
        //                 sample[index] = inside_triangle(x as f64, y as f64, &t.v) as u32;
        //                 if sample[index] == 1{
        //                     let (c1, c2, c3) = compute_barycentric2d(x as f64, y as f64, &t.v);
        //                     let z = c1 * t.v[0].z + c2 * t.v[1].z + c3 * t.v[2].z;
        //                     let color = t.get_color();
        //                     if z < self.depth_sample[index] {
        //                         changed = true;
        //                         self.depth_sample[index] = z;
        //                         self.set_sample(index , &color);
        //                     }
        //                     else{
        //                         sample[index] = 0;
        //                     }
        //                 }
        //             }
        //             else if sample[index] == 1{
        //                 changed = true;
        //             }
        //         }
        //         if changed{
        //             let mut pixel_color = Vector3::new(0.0, 0.0, 0.0);
        //             for i in 0..4 {
        //                 let x = index_x + orient[i].0;
        //                 let y = index_y + orient[i].1;
        //                 let index = y * (self.width + 1) as usize+ x;
        //                 pixel_color += self.frame_sample[index];
        //             }
        //             pixel_color /= 4.0;
        //             self.set_pixel(&Vector3::new(index_x as f64, index_y as f64, 0.0), &pixel_color);
        //         }
        //     }
        // }

    }

    pub fn frame_buffer(&self) -> &Vec<Vector3<f64>> {
        &self.frame_buf
    }
}

fn to_vec4(v3: Vector3<f64>, w: Option<f64>) -> Vector4<f64> {
    Vector4::new(v3.x, v3.y, v3.z, w.unwrap_or(1.0))
}

fn point_to_line_distance(x: f64, y: f64, p1: Vector4<f64>, p2: Vector4<f64>) -> f64 {
    // 直线方程参数 A, B, C (Ax + By + C = 0)
    let a = p2.y - p1.y;
    let b = p1.x - p2.x;
    let c = p2.x * p1.y - p1.x * p2.y;

    // 点到直线的距离公式
    let distance = (a * x + b * y + c).abs() / (a.powi(2) + b.powi(2)).sqrt();
    distance
}

fn all_in_out_triangle(x: f64, y: f64, v: &[Vector4<f64>; 3]) -> bool {
    let dist1 = point_to_line_distance(x, y, v[0], v[1]);
    let dist2 = point_to_line_distance(x, y, v[1], v[2]);
    let dist3 = point_to_line_distance(x, y, v[2], v[0]);
    
    return dist1 >= 1.0 && dist2 >= 1.0 && dist3 >= 1.0;
}

fn inside_triangle(x: f64, y: f64, v: &[Vector4<f64>; 3]) -> bool {
    /*  implement your code here  */

    let p0 = v[0];
    let p1 = v[1];
    let p2 = v[2];
    let a = (p1.x - p0.x) * (y - p0.y) - (x - p0.x) * (p1.y - p0.y);
    let b = (p2.x - p1.x) * (y - p1.y) - (x - p1.x) * (p2.y - p1.y);
    let c = (p0.x - p2.x) * (y - p2.y) - (x - p2.x) * (p0.y - p2.y);
    if a >= 0.0 && b >= 0.0 && c >= 0.0 {
        return true
    }
    else if a <= 0.0 && b <= 0.0 && c <= 0.0 {
        return true
    }

    false
}

fn compute_barycentric2d(x: f64, y: f64, v: &[Vector4<f64>; 3]) -> (f64, f64, f64) {
    let c1 = (x * (v[1].y - v[2].y) + (v[2].x - v[1].x) * y + v[1].x * v[2].y - v[2].x * v[1].y)
        / (v[0].x * (v[1].y - v[2].y) + (v[2].x - v[1].x) * v[0].y + v[1].x * v[2].y - v[2].x * v[1].y);
    let c2 = (x * (v[2].y - v[0].y) + (v[0].x - v[2].x) * y + v[2].x * v[0].y - v[0].x * v[2].y)
        / (v[1].x * (v[2].y - v[0].y) + (v[0].x - v[2].x) * v[1].y + v[2].x * v[0].y - v[0].x * v[2].y);
    let c3 = (x * (v[0].y - v[1].y) + (v[1].x - v[0].x) * y + v[0].x * v[1].y - v[1].x * v[0].y)
        / (v[2].x * (v[0].y - v[1].y) + (v[1].x - v[0].x) * v[2].y + v[0].x * v[1].y - v[1].x * v[0].y);
    (c1, c2, c3)
}