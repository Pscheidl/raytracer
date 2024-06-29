use crate::{projectile::{Projectile, self}};

#[derive(Copy, Clone)]
pub struct Room {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Room {
    pub fn new(
        x: f64,
        y: f64,
        z: f64,
    ) -> Room {
        Room {
            x,
            y,
            z,
        }
    }

    pub fn is_outside(self, projectile: &Projectile) -> bool {
        if projectile.x < 0.0 {
            return true;
        }
        if projectile.x > self.x {
            return true;
        }

        if projectile.y < 0.0 {
            return true;
        }
        if projectile.y > self.y {
            return true;
        }

        if projectile.z < 0.0 {
            return true;
        }
        if projectile.z > self.z {
            return true;
        }
        return false;
    }

    pub fn get_wall_color_at_projectile(self, projectile: &Projectile) -> Option<[f32; 4]> {
        let is_x_alternate = (projectile.x as i32/25) % 2 == 0;
        let is_y_alternate = (projectile.y as i32/25) % 2 == 0;
        let is_z_alternate = (projectile.z as i32/25) % 2 == 0;

        
        if projectile.x <= 0.0 { // left
            if is_y_alternate {
                return Some([
                    1.0 as f32, 
                    0.0, 
                    0.0, 
                    1.0
                ]) // red    
            } else if is_z_alternate {
                return Some([
                    1.0, 
                    0.15, 
                    0.15, 
                    1.0
                ]) // light red
            } else {
                return Some([
                    1.0, 
                    0.25, 
                    0.25, 
                    1.0
                ]) // lighter red
            }
        }
        if projectile.x >= self.x { // right                        
            if is_y_alternate {
                return Some([
                    0.0, 
                    1.0, 
                    0.0, 
                    1.0
                ]) // green
            } else if is_z_alternate {
                return Some([
                    0.2, 
                    1.0, 
                    0.2, 
                    1.0
                ]) // light green
            } else {
                return Some([
                    0.4, 
                    1.0, 
                    0.4, 
                    1.0
                ]) // light green
            }
        }
        if projectile.z <= 0.0 { // front                        
            if is_x_alternate {
                
                return Some([
                    0.0, 
                    1.0,
                    1.0,
                    1.0
                ])  // cyan  
            } else if is_y_alternate {
                return Some([
                    0.2, 
                    1.0, 
                    1.0, 
                    1.0
                ])  // light cyan  
            } else {
                return Some([
                    0.4, 
                    1.0, 
                    1.0, 
                    1.0
                ])  // lighter cyan  
            }
        }
        if projectile.z >= self.z { // back
            if is_x_alternate {
                return Some([
                    1.0, 
                    1.0, 
                    0.0, 
                    1.0
                ])  // yellow
            } else if is_y_alternate {
                return Some([
                    1.0, 
                    1.0, 
                    0.2, 
                    1.0
                ])  // light yellow
            } else {
                return Some([
                    1.0, 
                    1.0, 
                    0.4, 
                    1.0
                ])  // lighter yellow
            }
        }
        if projectile.y <= 0.0 { // up    
            if projectile.x + 10.0 > self.x / 2.0 && projectile.x - 10.0 < self.x / 2.0
            && projectile.z + 10.0 > self.z / 2.0 && projectile.z - 10.0 < self.z / 2.0  {
                return Some([
                    1.0,
                    1.0,
                    1.0,
                    1.0
                ]) // LIGHT
            }  else if is_x_alternate {
                return Some([
                    1.0, 
                    0.0, 
                    1.0, 
                    1.0
                ])  // pink
            } else if is_z_alternate {
                return Some([
                    1.0, 
                    0.15, 
                    1.0, 
                    1.0
                ])  // light pink
            } else {
                return Some([
                    1.0, 
                    0.25, 
                    1.0, 
                    1.0
                ])  // lighter pink
            }
        }
        if projectile.y >= self.y { // back
            if is_x_alternate {
                return Some([
                    0.0, 
                    0.0, 
                    1.0, 
                    1.0
                ]) // blue
            } else if is_z_alternate {
                return Some([
                    0.15,
                    0.15,
                    1.0, 
                    1.0
                    ]) // light blue
            } else {
                return Some([
                    0.25,
                    0.25, 
                    1.0, 
                    1.0
                ]) // lighter blue    
            }
        }
        return None;
    }

    pub fn is_projectile_near_light(&self, projectile: &Projectile) -> bool {
        const LIGHT_SIZE: f64 = 0.8;
        let room_to_projectile_dx = self.x / 2.0 - projectile.x;
        let room_to_projectile_dy = - projectile.y;
        let room_to_projectile_dz = self.z / 2.0 - projectile.z;

        if room_to_projectile_dx.abs() < LIGHT_SIZE && room_to_projectile_dy.abs() < LIGHT_SIZE && room_to_projectile_dz.abs() < LIGHT_SIZE { 
            return true
        }
        return false
    }
}