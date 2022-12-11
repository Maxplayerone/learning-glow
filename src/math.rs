
const PI: f32 = 3.1415;

#[derive(Debug)]
pub struct Vec3{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3{
    pub fn new(x: f32, y: f32, z: f32) -> Self{
        Self{
            x, y, z
        }
    }
}

#[derive(Debug)]
pub struct Vec4{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4{
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self{
        Self{
            x, y, z, w
        }
    }
}

#[derive(Debug)]
pub struct Mat4{
    pub mat: [f32; 16],
}

pub enum RotationAxis{
    X,
    Y,
    Z,
}

impl Mat4{
    pub fn new(value: f32) -> Self{
        Self{
            mat: [
            value, 0.0, 0.0, 0.0,
            0.0, value, 0.0, 0.0,
            0.0, 0.0, value, 0.0,
            0.0, 0.0, 0.0, 1.0,
            ]
        }
    }

    pub fn scale(&mut self, scaling_value: f32){
        self.mat[0] *= scaling_value;
        self.mat[5] *= scaling_value;
        self.mat[10] *= scaling_value;
    }

    pub fn translate(&mut self, vec: Vec3){
        self.mat[3] += vec.x;
        self.mat[7] += vec.y;
        self.mat[11] += vec.z;
    }

    pub fn rotate(&mut self, angle_degrees: f32, axis: RotationAxis){
        let angle_radians = angle_degrees * (PI / 180.0);
        let angle_sin = angle_radians.sin();
        let angle_cos = angle_radians.cos();
        match axis{
            RotationAxis::X => {
            self.mat[5] = angle_cos;
            self.mat[6] = -angle_sin;
            self.mat[9] = angle_sin;
            self.mat[10] = angle_cos;
            },
            RotationAxis::Y => {
                self.mat[0] = angle_cos;
                self.mat[2] = angle_sin;
                self.mat[8] = -angle_sin;
                self.mat[10] = angle_cos; 
            },

            RotationAxis::Z => {
            self.mat[0] = angle_cos;
            self.mat[1] = -angle_sin;
            self.mat[4] = angle_sin;
            self.mat[5] = angle_cos;
            },
        }
    }

    pub fn perspective(&mut self, fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32){
        let frustum_depth = z_far - z_near;
        let inv_frustum_depth = 1.0 / frustum_depth;

        self.mat[5] = 1.0 / (0.5 * fov).tan();
        self.mat[0] = self.mat[5] / aspect_ratio;
        self.mat[10] = -z_far * inv_frustum_depth;
        self.mat[14] = -z_far * z_near * inv_frustum_depth * 2.0;
        self.mat[11] = -1.0;
        self.mat[15] = 0.0;
    }

    pub fn mul(&self, vec: Vec4) -> Vec4{
        Vec4::new(
            self.mat[0] * vec.x + self.mat[1] * vec.y + self.mat[2] * vec.z + self.mat[3] * vec.w,
            self.mat[4] * vec.x + self.mat[5] * vec.y + self.mat[6] * vec.z + self.mat[7] * vec.w,
            self.mat[8] * vec.x + self.mat[9] * vec.y + self.mat[10] * vec.z + self.mat[11] * vec.w,
            self.mat[12] * vec.x + self.mat[13] * vec.y + self.mat[14] * vec.z + self.mat[15] * vec.w,
        )
    }

    pub fn transpose(&mut self){
        let mut tmp = self.mat[1];
        self.mat[1] = self.mat[4];
        self.mat[4] = tmp;

        tmp = self.mat[2];
        self.mat[2] = self.mat[8];
        self.mat[8] = tmp;

        tmp = self.mat[3];
        self.mat[3] = self.mat[12];
        self.mat[12] = tmp;

        tmp = self.mat[6];
        self.mat[6] = self.mat[10];
        self.mat[10] = tmp;

        tmp = self.mat[7];
        self.mat[7] = self.mat[13];
        self.mat[13] = tmp;

        tmp = self.mat[11];
        self.mat[11] = self.mat[14];
        self.mat[14] = tmp;
    }

}