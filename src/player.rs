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
        is_shooting: bool, 
        projectiles: [[Projectile; 250]; 250],//Vec<Projectile>,
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
            is_shooting,
            projectiles,
            shooting_timer,
        }
    }
    pub fn spawn_new_rays(&mut self) {
        if self.is_moving_left {
            self.x -= 1.1;
        }

        if self.is_moving_right {
            self.x += 1.1;
        }

        if self.is_moving_up {
            self.z -= 1.1;
        }

        if self.is_moving_down {
            self.z += 1.1;
        }
        if self.is_moving_backward {
            self.y += 1.1;
        }
        if self.is_moving_forward {
            self.y -= 1.1;
        }

        for y in 0..250 {
            for x in 0..250 { //WINDOW_WIDTH
                self.projectiles[y][x] = Projectile::new(
                        256 as f64,
                        20.0,
                        0.0,
                        1.8+(x) as f64 / 250.0,
                        y as f64 / 250.0
                );
            }
        }
    }
}
