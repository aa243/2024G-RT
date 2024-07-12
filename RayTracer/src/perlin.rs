use crate::util::random_between;
use crate::util::random_double;
use crate::util::Point3;
use crate::Vec3;

pub struct Perlin {
    point_count: usize,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
    rand_vec: Vec<Vec3>,
}

impl Perlin {
    pub fn new() -> Self {
        let point_count = 256;
        let mut rand_vec: Vec<Vec3> = Vec::with_capacity(point_count);
        for _ in 0..point_count {
            rand_vec.push(Vec3::random_between(-1.0, 1.0).normalize())
        }
        let perm_x = Perlin::perlin_generate_perm();
        let perm_y = Perlin::perlin_generate_perm();
        let perm_z = Perlin::perlin_generate_perm();
        Self {
            point_count,
            perm_x,
            perm_y,
            perm_z,
            rand_vec,
        }
    }
    pub fn noise(&self, p: &Point3) -> f64 {
        // let i = (p.x * 4.0) as i32 & 255;
        // let j = (p.y * 4.0) as i32 & 255;
        // let k = (p.z * 4.0) as i32 & 255;

        // return self.randfloat[(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize];
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = [[[Vec3::zero(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di as usize][dj as usize][dk as usize] = self.rand_vec[(self.perm_x
                        [((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize])
                        as usize];
                }
            }
        }

        return Perlin::perlin_interpolate(c, u, v, w);
    }

    pub fn turb(&self, p: &Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;
        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }
        accum.abs()
    }
    fn perlin_generate_perm() -> Vec<i32> {
        let mut p = Vec::with_capacity(256);
        for i in 0..256 {
            p.push(i as i32);
        }
        Perlin::permute(&mut p, 256);
        p
    }
    fn permute(p: &mut Vec<i32>, n: usize) {
        for i in (1..n).rev() {
            let target = random_between(0.0, i as f64) as usize;
            p.swap(i, target);
        }
    }
    fn trilinear_interpolate(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f64 * u + (1.0 - i as f64) * (1.0 - u))
                        * (j as f64 * v + (1.0 - j as f64) * (1.0 - v))
                        * (k as f64 * w + (1.0 - k as f64) * (1.0 - w))
                        * c[i][j][k];
                }
            }
        }
        accum
    }
    fn perlin_interpolate(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                        * c[i][j][k].dot(&weight_v);
                }
            }
        }
        accum
    }
}
