
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

    pub fn transform(&mut self, vec: Vec3){
        self.mat[12] += vec.x;
        self.mat[13] += vec.y;
        self.mat[14] += vec.z;
    }

    //always rotates around the z-axis
    pub fn rotate(&mut self, angle_degrees: f32){
        let angle_radians = angle_degrees * (PI / 180.0);
        let angle_sin = angle_radians.sin();
        let angle_cos = angle_radians.cos();

        self.mat[0] = angle_cos;
        self.mat[1] = -angle_sin;
        self.mat[4] = angle_sin;
        self.mat[5] = angle_cos;
    }

    pub fn mul(&self, vec: Vec4) -> Vec4{
        Vec4::new(
            self.mat[0] * vec.x + self.mat[1] * vec.y + self.mat[2] * vec.z + self.mat[3] * vec.w,
            self.mat[4] * vec.x + self.mat[5] * vec.y + self.mat[6] * vec.z + self.mat[7] * vec.w,
            self.mat[8] * vec.x + self.mat[9] * vec.y + self.mat[10] * vec.z + self.mat[11] * vec.w,
            self.mat[12] * vec.x + self.mat[13] * vec.y + self.mat[14] * vec.z + self.mat[15] * vec.w,
        )
    }

}