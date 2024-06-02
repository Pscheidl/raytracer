use std::f64::consts::PI;
use std::collections::btree_set::Range;

use crate::{WINDOW_WIDTH, WINDOW_HEIGHT, projectile::{Projectile, self}};

const MAX_VELOCITY_X:f64 = 1.0;
const MAX_VELOCITY_Y:f64 = 1.0;
pub const WIDTH:usize= 20*4;
pub const HEIGHT:usize= 20*4;


pub struct Player {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub move_speed_left: f64,
    pub move_speed_up: f64,
    pub is_moving_up: bool,
    pub is_moving_down: bool,
    pub is_moving_left: bool,
    pub is_moving_right: bool,
    pub is_moving_forward: bool,
    pub is_moving_backward: bool,
    pub is_looking_up: bool,
    pub is_looking_down: bool,
    pub is_looking_left: bool,
    pub is_looking_right: bool,
    pub is_roll_left: bool,
    pub is_roll_right: bool,
    pub is_shooting: bool,
    pub projectiles: [[Projectile; 250]; 250],//Vec<Projectile>,
    pub shooting_timer: usize,
}

impl Player {
    pub fn new(
        x: f64,
        y: f64,
        z: f64,
        move_speed_left: f64,
        move_speed_up: f64,
        is_moving_up: bool,
        is_moving_down: bool,
        is_moving_left: bool,
        is_moving_right: bool,
        is_moving_forward: bool,
        is_moving_backward: bool,
        is_looking_up: bool,
        is_looking_down: bool,
        is_looking_left: bool,
        is_looking_right: bool,
        is_roll_left: bool,
        is_roll_right: bool,
        is_shooting: bool, 
        projectiles: [[Projectile; 250]; 250], //Vec<Projectile>,
        shooting_timer: usize,
    ) -> Player {
        Player {
            x,
            y,
            z,
            move_speed_left,
            move_speed_up,
            is_moving_up,
            is_moving_down,
            is_moving_left,
            is_moving_right,    
            is_moving_forward,
            is_moving_backward,
            is_looking_up,
            is_looking_down,
            is_looking_left,
            is_looking_right,
            is_roll_left,
            is_roll_right,
            is_shooting,
            projectiles,
            shooting_timer,
        }
    }
    pub fn spawn_new_rays(&mut self) {
        if self.is_moving_left {
            self.x -= 3.1;
        }

        if self.is_moving_right {
            self.x += 3.1;
        }

        if self.is_moving_up {
            self.z -= 3.1;
        }

        if self.is_moving_down {
            self.z += 3.1;
        }
        if self.is_moving_backward {
            self.y += 3.1;
        }
        if self.is_moving_forward {
            self.y -= 3.1;
        }

        if self.is_looking_up {
            self.y += 3.1;
        }
        if self.is_looking_down {
            self.y -= 3.1;
        }

        for y in 0..250 {
            for x in 0..250 { //WINDOW_WIDTH
                let mut delta_x = (-3.0 + 6.0 / 250.0 * x as f64);
                let mut delta_y = (-3.0 + 6.0 / 250.0 * y as f64);
                let mut delta_z: f64 = 1.0;
                let vec_len = (delta_x.powf(2.0) + delta_y.powf(2.0) + delta_z.powf(2.0)).sqrt();
                
                delta_x /= vec_len;
                delta_y /= vec_len;
                delta_z /= vec_len;

                self.projectiles[y][x] = Projectile::new(
                    self.x,
                    self.y,
                    self.z,
                    delta_x,
                    delta_y,
                    delta_z,
                    //(-0.8 + 1.8 / 250.0 * x as f64).atan(),   // -46 to 46 LEFT RIGHT
                    //(-0.8 + 1.8 / 250.0 * y as f64).atan(), // -46 to 46 UP DOWN
                    1.0 // 1.0 max brightness 0.0 dead
                );
            }
        }
    }
}
