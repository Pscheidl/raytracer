use crate::projectile::Projectile;
use crate::math::Vector3D;

#[derive(Copy, Clone)]
pub struct LightSource {
    pub strenght: f64,
    pub delta_strenght: f64,
    pub position: Vector3D,
    pub delta_position: Vector3D,
}

impl LightSource {
    pub fn new(
        strenght: f64,
        delta_strenght: f64,
        position: Vector3D,
        delta_position: Vector3D,
    ) -> LightSource {
        LightSource {
            strenght,
            delta_strenght,
            position,
            delta_position,
        }
    }

    pub fn tick(&mut self) {
        self.position.0[0] += self.delta_position.0[0];
        self.position.0[1] += self.delta_position.0[1];
        self.position.0[2] += self.delta_position.0[2];
        if self.position.0[0] < 0.0 {
            self.delta_position.0[0] = - self.delta_position.0[0];
        }
        if self.position.0[1] < 0.0 {
            self.delta_position.0[1] = - self.delta_position.0[1];
        }
        if self.position.0[2] < 0.0 {
            self.delta_position.0[2] = - self.delta_position.0[2];
        }

        if self.position.0[0] > 150_f64 {
            self.delta_position.0[0] = - self.delta_position.0[0];
        }
        if self.position.0[1] > 150_f64 {
            self.delta_position.0[1] = - self.delta_position.0[1];
        }
        if self.position.0[2] > 150_f64 {
            self.delta_position.0[2] = - self.delta_position.0[2];
        }

        self.flicker_light();   
    }

    pub fn flicker_light(&mut self) {
        self.strenght += self.delta_strenght;
        if self.strenght > 75_f64 {
            self.delta_strenght = -2_f64;
        } else if self.strenght < 5_f64 {
            self.delta_strenght = 2_f64;
        }
    }
}


#[derive(Copy, Clone)]
pub struct Room {
    pub size: Vector3D,
    pub light_source: LightSource,
}

impl Room {
    pub fn new(
        size: Vector3D,
        light_source: LightSource,
    ) -> Room {
        Room {
            size,
            light_source,
        }
    }

    pub fn is_outside(self, projectile: &Projectile) -> bool {
        if projectile.x <= 0.0 {
            return true;
        }
        if projectile.x >= self.size.0[0] {
            return true;
        }

        if projectile.y <= 0.0 {
            return true;
        }
        if projectile.y >= self.size.0[1]  {
            return true;
        }

        if projectile.z <= 0.0 {
            return true;
        }
        if projectile.z >= self.size.0[2] {
            return true;
        }
        false
    }

    pub fn get_wall_color_at_projectile(self, projectile: &Projectile) -> Option<[f32; 4]> {
        let is_x_alternate = (projectile.x as i32/25) % 2 == 0;
        let is_y_alternate = (projectile.y as i32/25) % 2 == 0;
        let is_z_alternate = (projectile.z as i32/25) % 2 == 0;

        
        if projectile.x <= 0.0 { // left
            if is_y_alternate {
                return Some([
                    1.0, 
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
        if projectile.x >= self.size.0[0] { // right                        
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
        if projectile.z >= self.size.0[2] { // back
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
            if projectile.x + 10.0 > self.light_source.position.0[0] && projectile.x - 10.0 < self.light_source.position.0[0]
            && projectile.z + 10.0 > self.light_source.position.0[2] && projectile.z - 10.0 < self.light_source.position.0[2]  {
                return Some([
                    1.0,
                    1.0,
                    0.5,
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
        if projectile.y >= self.size.0[2] { // back
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
        None
    }
}