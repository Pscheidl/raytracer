use crate::WINDOW_WIDTH;
use crate::player::WIDTH;

#[derive(Copy, Clone, Debug)]
pub struct Projectile {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f64,
    pub pitch: f64,
}

impl Projectile {
    pub fn new(
        x: f64,
        y: f64,
        z: f64,
        yaw: f64,
        pitch: f64,
    ) -> Projectile {
        Projectile {
            x,
            y,
            z,
            yaw,
            pitch,            
        }
    }
}