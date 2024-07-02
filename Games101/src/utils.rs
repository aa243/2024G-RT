#![allow(warnings)]
use std::os::raw::c_void;
use std::result;
use nalgebra::{Matrix3, Matrix4, Vector3, Vector4};
use opencv::core::{Mat, MatTraitConst};
use opencv::imgproc::{COLOR_RGB2BGR, cvt_color};
use crate::shader::{FragmentShaderPayload, VertexShaderPayload};
use crate::texture::Texture;
use crate::triangle::Triangle;

pub type V3f = Vector3<f64>;
pub type M4f = Matrix4<f64>;

pub(crate) fn get_view_matrix(eye_pos: V3f) -> M4f {
    let mut view: Matrix4<f64> = Matrix4::identity();
    /*  implement your code here  */
    view.m14 = -eye_pos.x;
    view.m24 = -eye_pos.y;
    view.m34 = -eye_pos.z;
    // println!("view matrix: {:?}", view);
    view
}

pub(crate) fn get_model_matrix(rotation_angle: f64,scale: f64) -> M4f {
    let mut model: Matrix4<f64> = Matrix4::identity();
    /*  implement your code here  */
    let radian_rotation_angle = rotation_angle.to_radians();
    model.m11 = radian_rotation_angle.cos();
    model.m31 = -radian_rotation_angle.sin();
    model.m13 = radian_rotation_angle.sin();
    model.m33 = radian_rotation_angle.cos();
    let mut scale_matrix: Matrix4<f64> = Matrix4::identity();
    scale_matrix.m11 = scale;
    scale_matrix.m22 = scale;
    scale_matrix.m33 = scale;
    scale_matrix * model
}

pub(crate) fn get_projection_matrix(eye_fov: f64, aspect_ratio: f64, z_near: f64, z_far: f64) -> M4f {
    let mut projection: Matrix4<f64> = Matrix4::identity();
    /*  implement your code here  */

    let radian_eye_fov = eye_fov.to_radians();
    projection.m11 = z_near;
    projection.m22 = z_near;
    projection.m33 = (z_near+z_far);
    projection.m34 = -z_near*z_far;
    projection.m43 = 1.0;
    projection.m44 = 0.0;
    // println!("projection matrix: {:?}", projection);

    let mut ortho_scale: Matrix4<f64> = Matrix4::identity();
    let mut ortho_trans: Matrix4<f64> = Matrix4::identity();
    let top = -z_near * (radian_eye_fov/2.0).tan();
    let bottom = -top;
    let right = top * aspect_ratio;
    let left = -right;
    ortho_scale.m11 = 2.0/(right-left);
    ortho_scale.m22 = 2.0/(top-bottom);
    ortho_scale.m33 = 2.0/(z_near-z_far);

    ortho_trans.m34 = -(z_far+z_near)/2.0;

    // println!("projection matrix: {:?}", ortho_trans);

    ortho_scale * ortho_trans * projection
}

pub(crate) fn get_rotation_matrix(mut axis: Vector3<f64>, angle:f64) -> Matrix4<f64> {
    let mut rotation: Matrix4<f64> = Matrix4::identity();
    /*  implement your code here  */
    let radian_angle = angle.to_radians();
    let cos = radian_angle.cos();
    let sin = radian_angle.sin();
    axis = axis.normalize();
    let x = axis.x;
    let y = axis.y;
    let z = axis.z;

    let axis4 = Vector4::new(x, y, z, 0.0);

    rotation *= cos;
    rotation.m44 = 1.0;
    rotation = rotation + (1.0 - cos) * axis4 * axis4.transpose();
    rotation += sin * Matrix4::new(0.0, -z, y, 0.0,
                                   z, 0.0, -x, 0.0,
                                   -y, x, 0.0, 0.0,
                                   0.0, 0.0, 0.0, 0.0);
    // println!("rotation matrix: {:?}", rotation);
    rotation
}

pub(crate) fn frame_buffer2cv_mat(frame_buffer: &Vec<V3f>) -> Mat {
    let mut image = unsafe {
        Mat::new_rows_cols_with_data(
            700, 700,
            opencv::core::CV_64FC3,
            frame_buffer.as_ptr() as *mut c_void,
            opencv::core::Mat_AUTO_STEP,
        ).unwrap()
    };
    let mut img = Mat::copy(&image).unwrap();
    image.convert_to(&mut img, opencv::core::CV_8UC3, 1.0, 1.0).expect("panic message");
    cvt_color(&img, &mut image, COLOR_RGB2BGR, 0).unwrap();
    image
}

pub fn load_triangles(obj_file: &str) -> Vec<Triangle> {
    let (models, _) = tobj::load_obj(&obj_file, &tobj::LoadOptions::default()).unwrap();
    let mesh = &models[0].mesh;
    let n = mesh.indices.len() / 3;
    let mut triangles = vec![Triangle::default(); n];

    // 遍历模型的每个面
    for vtx in 0..n {
        let rg = vtx * 3..vtx * 3 + 3;
        let idx: Vec<_> = mesh.indices[rg.clone()].iter().map(|i| *i as usize).collect();

        // 记录图形每个面中连续三个顶点（小三角形）
        for j in 0..3 {
            let v = &mesh.positions[3 * idx[j]..3 * idx[j] + 3];
            triangles[vtx].set_vertex(j, Vector4::new(v[0] as f64, v[1] as f64, v[2] as f64, 1.0));
            let ns = &mesh.normals[3 * idx[j]..3 * idx[j] + 3];
            triangles[vtx].set_normal(j, Vector3::new(ns[0] as f64, ns[1] as f64, ns[2] as f64));
            let tex = &mesh.texcoords[2 * idx[j]..2 * idx[j] + 2];
            triangles[vtx].set_tex_coord(j, tex[0] as f64, tex[1] as f64);
        }
    }
    triangles
}

// 选择对应的Shader
pub fn choose_shader_texture(method: &str,
                             obj_path: &str) -> (fn(&FragmentShaderPayload) -> Vector3<f64>, Option<Texture>) {
    let mut active_shader: fn(&FragmentShaderPayload) -> Vector3<f64> = phong_fragment_shader;
    let mut tex = None;
    if method == "normal" {
        println!("Rasterizing using the normal shader");
        active_shader = normal_fragment_shader;
    } else if method == "texture" {
        println!("Rasterizing using the texture
         shader");
        active_shader = texture_fragment_shader;
        tex = Some(Texture::new(&(obj_path.to_owned() + "spot_texture.png")));
    } else if method == "phong" {
        println!("Rasterizing using the phong shader");
        active_shader = phong_fragment_shader;
    } else if method == "bump" {
        println!("Rasterizing using the bump shader");
        active_shader = bump_fragment_shader;
    } else if method == "displacement" {
        println!("Rasterizing using the displacement shader");
        active_shader = displacement_fragment_shader;
    }
    (active_shader, tex)
}

pub fn vertex_shader(payload: &VertexShaderPayload) -> V3f {
    payload.position
}

#[derive(Default)]
struct Light {
    pub position: V3f,
    pub intensity: V3f,
}

pub fn normal_fragment_shader(payload: &FragmentShaderPayload) -> V3f {
    let result_color =
        (payload.normal.xyz().normalize() + Vector3::new(1.0, 1.0, 1.0)) / 2.0;
    result_color * 255.0
}

pub fn phong_fragment_shader(payload: &FragmentShaderPayload) -> V3f {
    // 泛光、漫反射、高光系数
    let ka = Vector3::new(0.005, 0.005, 0.005);
    let kd = payload.color;
    let ks = Vector3::new(0.7937, 0.7937, 0.7937);

    // 灯光位置和强度
    let l1 = Light {
        position: Vector3::new(20.0, 20.0, 20.0),
        intensity: Vector3::new(500.0, 500.0, 500.0),
    };
    let l2 = Light {
        position: Vector3::new(-20.0, 20.0, 0.0),
        intensity: Vector3::new(500.0, 500.0, 500.0),
    };
    let lights = vec![l1, l2];
    let amb_light_intensity = Vector3::new(10.0, 10.0, 10.0);
    let eye_pos = Vector3::new(0.0, 0.0, 10.0);

    let p = 150.0;

    // ping point的信息
    let normal = payload.normal;
    let point = payload.view_pos;
    let color = payload.color;

    let mut result_color = Vector3::zeros(); // 保存光照结果
    
    // <遍历每一束光>
    for light in lights {
        // LAB3 TODO: For each light source in the code, calculate what the *ambient*, *diffuse*, and *specular* 
        // components are. Then, accumulate that result on the *result_color* object.

        let l = (light.position - point).normalize();
        let v = (eye_pos - point).normalize();
        let h = (l + v).normalize();
        let r = (light.position-point).norm();

        let amb = ka.component_mul(&amb_light_intensity);
        let diff = kd.component_mul(&((light.intensity /(r * r)) * normal.dot(&l).max(0.0)));
        let spec = ks.component_mul(&((light.intensity /(r * r)) * h.dot(&normal).max(0.0).powf(p)));
        result_color += amb + diff + spec;
    }
    result_color * 255.0
}

pub fn texture_fragment_shader(payload: &FragmentShaderPayload) -> V3f {
    let ka = Vector3::new(0.005, 0.005, 0.005);
    let texture_color: Vector3<f64> = match &payload.texture {
        // LAB3 TODO: Get the texture value at the texture coordinates of the current fragment
        // <获取材质颜色信息>

        None => Vector3::new(0.0, 0.0, 0.0),
        Some(texture) => payload.texture.as_ref().unwrap().get_color_bilinear(payload.tex_coords[0],payload.tex_coords[1]), // Do modification here
    };
    let kd = texture_color / 255.0; // 材质颜色影响漫反射系数
    let ks = Vector3::new(0.7937, 0.7937, 0.7937);

    let l1 = Light {
        position: Vector3::new(20.0, 20.0, 20.0),
        intensity: Vector3::new(500.0, 500.0, 500.0),
    };
    let l2 = Light {
        position: Vector3::new(-20.0, 20.0, 0.0),
        intensity: Vector3::new(500.0, 500.0, 500.0),
    };
    let lights = vec![l1, l2];
    let amb_light_intensity = Vector3::new(10.0, 10.0, 10.0);
    let eye_pos = Vector3::new(0.0, 0.0, 10.0);

    let p = 150.0;

    let color = texture_color;
    let point = payload.view_pos;
    let normal = payload.normal;

    let mut result_color = Vector3::zeros();

    for light in lights {
        // LAB3 TODO: For each light source in the code, calculate what the *ambient*, *diffuse*, and *specular* 
        // components are. Then, accumulate that result on the *result_color* object.
        let l = (light.position - point).normalize();
        let v = (eye_pos - point).normalize();
        let h = (l + v).normalize();
        let r = (light.position-point).norm();

        let amb = ka.component_mul(&amb_light_intensity);
        let diff = kd.component_mul(&((light.intensity /(r * r)) * normal.dot(&l).max(0.0)));
        let spec = ks.component_mul(&((light.intensity /(r * r)) * h.dot(&normal).max(0.0).powf(p)));
        result_color += amb + diff + spec;
    }

    result_color * 255.0
}

pub fn bump_fragment_shader(payload: &FragmentShaderPayload) -> V3f {
    let ka = Vector3::new(0.005, 0.005, 0.005);
    let kd = payload.color;
    let ks = Vector3::new(0.7937, 0.7937, 0.7937);

    let l1 = Light {
        position: Vector3::new(20.0, 20.0, 20.0),
        intensity: Vector3::new(500.0, 500.0, 500.0),
    };
    let l2 = Light {
        position: Vector3::new(-20.0, 20.0, 0.0),
        intensity: Vector3::new(500.0, 500.0, 500.0),
    };
    let lights = vec![l1, l2];
    let amb_light_intensity = Vector3::new(10.0, 10.0, 10.0);
    let eye_pos = Vector3::new(0.0, 0.0, 10.0);

    let p = 150.0;

    let normal = payload.normal;
    let point = payload.view_pos;
    let color = payload.color;

    let (kh, kn) = (0.2, 0.1);

    // LAB3 TODO: Implement bump mapping here 
    // Let n = normal = (x, y, z)
    // Vector t = (x*y/sqrt(x*x+z*z),sqrt(x*x+z*z),z*y/sqrt(x*x+z*z))
    // Vector b = n cross product t
    // Matrix TBN = [t b n]
    // dU = kh * kn * (h(u+1/w,v)-h(u,v))
    // dV = kh * kn * (h(u,v+1/h)-h(u,v))
    // Vector ln = (-dU, -dV, 1)
    // Normal n = normalize(TBN * ln)

    let (x,y,z) = (normal.x, normal.y, normal.z);
    let t = Vector3::new(x*y/(x*x+z*z).sqrt(),(x*x+z*z).sqrt(),z*y/(x*x+z*z).sqrt());
    let b = normal.cross(&t);

    let tbn = Matrix3::new(t.x, b.x, x,
                            t.y, b.y, y,
                            t.z, b.z, z);
    let w = payload.texture.as_ref().unwrap().width as f64;
    let h = payload.texture.as_ref().unwrap().height as f64;
    let u = payload.tex_coords[0];
    let v = payload.tex_coords[1];

    let color_u_v = payload.texture.as_ref().unwrap().get_color(u,v);
    let dU = kh * kn * (payload.texture.as_ref().unwrap().get_color(u+1.0/w,v).norm() - color_u_v.norm());
    let dV = kh * kn * (payload.texture.as_ref().unwrap().get_color(u,v+1.0/h).norm() - color_u_v.norm());

    let ln = Vector3::new(-dU, -dV, 1.0);
    let normal = (tbn * ln).normalize();
    

    let mut result_color = Vector3::zeros();
    result_color = normal;

    result_color * 255.0
}

pub fn displacement_fragment_shader(payload: &FragmentShaderPayload) -> V3f {
    let ka = Vector3::new(0.005, 0.005, 0.005);
    let kd = payload.color;
    let ks = Vector3::new(0.7937, 0.7937, 0.7937);

    let l1 = Light {
        position: Vector3::new(20.0, 20.0, 20.0),
        intensity: Vector3::new(500.0, 500.0, 500.0),
    };
    let l2 = Light {
        position: Vector3::new(-20.0, 20.0, 0.0),
        intensity: Vector3::new(500.0, 500.0, 500.0),
    };
    let lights = vec![l1, l2];
    let amb_light_intensity = Vector3::new(10.0, 10.0, 10.0);
    let eye_pos = Vector3::new(0.0, 0.0, 10.0);

    let p = 150.0;

    let normal = payload.normal;
    let point = payload.view_pos;
    let color = payload.color;

    let (kh, kn) = (0.2, 0.1);

    // LAB3 TODO: Implement displacement mapping here
    // Let n = normal = (x, y, z)
    // Vector t = (x*y/sqrt(x*x+z*z),sqrt(x*x+z*z),z*y/sqrt(x*x+z*z))
    // Vector b = n cross product t
    // Matrix TBN = [t b n]
    // dU = kh * kn * (h(u+1/w,v)-h(u,v))
    // dV = kh * kn * (h(u,v+1/h)-h(u,v))
    // Vector ln = (-dU, -dV, 1)
    // Position p = p + kn * n * h(u,v)
    // Normal n = normalize(TBN * ln)

    let (x,y,z) = (normal.x, normal.y, normal.z);
    let t = Vector3::new(x*y/(x*x+z*z).sqrt(),(x*x+z*z).sqrt(),z*y/(x*x+z*z).sqrt());
    let b = normal.cross(&t);

    let tbn = Matrix3::new(t.x, b.x, x,
                            t.y, b.y, y,
                            t.z, b.z, z);
    let w = payload.texture.as_ref().unwrap().width as f64;
    let h = payload.texture.as_ref().unwrap().height as f64;
    let u = payload.tex_coords[0];
    let v = payload.tex_coords[1];

    let color_u_v = payload.texture.as_ref().unwrap().get_color(u,v);
    let dU = kh * kn * (payload.texture.as_ref().unwrap().get_color(u+1.0/w,v).norm() - color_u_v.norm());
    let dV = kh * kn * (payload.texture.as_ref().unwrap().get_color(u,v+1.0/h).norm() - color_u_v.norm());

    let ln = Vector3::new(-dU, -dV, 1.0);
    let point = point + kn * normal * color_u_v.norm();
    let normal = (tbn * ln).normalize();

    let mut result_color = Vector3::zeros();
    for light in lights {
        // LAB3 TODO: For each light source in the code, calculate what the *ambient*, *diffuse*, and *specular* 
        // components are. Then, accumulate that result on the *result_color* object.
        let l = (light.position - point).normalize();
        let v = (eye_pos - point).normalize();
        let h = (l + v).normalize();
        let r = (light.position-point).norm();

        let amb = ka.component_mul(&amb_light_intensity);
        let diff = kd.component_mul(&((light.intensity /(r * r)) * normal.dot(&l).max(0.0)));
        let spec = ks.component_mul(&((light.intensity /(r * r)) * h.dot(&normal).max(0.0).powf(p)));
        result_color += amb + diff + spec;
        
    }

    result_color * 255.0
}
