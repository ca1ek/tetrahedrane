use super::geometry::*;

fn rotate(x: f32, y: f32, around_x: f32, around_y: f32, angle: f32) -> (f32, f32) {
    use std::f32;
    
    let s = f32::sin(angle);
    let c = f32::cos(angle);
    
    let x = x - around_x;
    let y = y - around_y;
    
    let x = x * c - y * s;
    let y = x * s + y * c;
    
    (x + around_x, y + around_y)
}

pub trait Rotation {
    fn rotate_x_y(&mut self, x: f32, y: f32, angle: f32);
    fn rotate_x_z(&mut self, x: f32, y: f32, angle: f32);
    fn rotate_y_z(&mut self, x: f32, y: f32, angle: f32);
}

impl Rotation for Point {
    fn rotate_x_y(&mut self, x: f32, y: f32, angle: f32) {
        let (x, y) = rotate(self.x, self.y, x, y, angle);
        self.x = x;
        self.y = y;
    }
    
    fn rotate_x_z(&mut self, x: f32, z: f32, angle: f32) {
        let (x, z) = rotate(self.x, self.z, x, z, angle);
        self.x = x;
        self.z = z;
    }
    
    fn rotate_y_z(&mut self, y: f32, z: f32, angle: f32) {
        let (y, z) = rotate(self.y, self.z, y, z, angle);
        self.y = y;
        self.z = z;
    }
}

impl Rotation for Triangle {
    fn rotate_x_y(&mut self, x: f32, y: f32, angle: f32) {
        self.p1.rotate_x_y(x, y, angle);
        self.p2.rotate_x_y(x, y, angle);
        self.p3.rotate_x_y(x, y, angle);
    }
    
    fn rotate_x_z(&mut self, x: f32, z: f32, angle: f32) {
        self.p1.rotate_x_z(x, z, angle);
        self.p2.rotate_x_z(x, z, angle);
        self.p3.rotate_x_z(x, z, angle);
    }
    
    fn rotate_y_z(&mut self, y: f32, z: f32, angle: f32) {
        self.p1.rotate_y_z(y, z, angle);
        self.p2.rotate_y_z(y, z, angle);
        self.p3.rotate_y_z(y, z, angle);
    }
}