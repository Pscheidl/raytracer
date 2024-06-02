use crate::WINDOW_WIDTH;
use crate::player::WIDTH;

#[derive(Copy, Clone, Debug)]
pub struct Projectile {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
    pub time_to_live: f64,
}

impl Projectile {
    pub fn new(
        x: f64,
        y: f64,
        z: f64,
        dx: f64,
        dy: f64,
        dz: f64,
        time_to_live: f64,
    ) -> Projectile {
        Projectile {
            x,
            y,
            z,
            dx,
            dy,
            dz,
            time_to_live,
        }
    }
}