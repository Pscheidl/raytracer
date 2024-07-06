#[derive(Copy, Clone, Debug)]
pub struct Projectile {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
}

impl Projectile {
    pub fn new(
        x: f64,
        y: f64,
        z: f64,
        dx: f64,
        dy: f64,
        dz: f64,
    ) -> Projectile {
        Projectile {
            x,
            y,
            z,
            dx,
            dy,
            dz,
        }
    }

    pub fn increment(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
        self.z += self.dz;
    }

    pub fn multi_increment(&mut self, multiplier: f64) {
        self.x += self.dx * multiplier;
        self.y += self.dy * multiplier;
        self.z += self.dz * multiplier;
    }
}