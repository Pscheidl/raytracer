
use crate::{projectile::{Projectile, self}};

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub angle_x: f64,
    pub angle_y: f64,
    pub angle_z: f64,
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
    pub is_low_detail_render: bool,
    pub projectiles: Vec::<Vec::<Projectile>>,
}

impl Player {
    pub fn new(
        x: f64,
        y: f64,
        z: f64,
        angle_x: f64,
        angle_y: f64,
        angle_z: f64,
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
        is_low_detail_render: bool,
        projectiles: Vec::<Vec::<Projectile>>,
    ) -> Player {
        Player {
            x,
            y,
            z,
            angle_x,
            angle_y,
            angle_z,
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
            is_low_detail_render,
            projectiles,
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
            self.y -= 3.1;
        }

        if self.is_moving_down {
            self.y += 3.1;
        }
        if self.is_moving_backward {
            self.z += 3.1;
        }
        if self.is_moving_forward {
            self.z -= 3.1;
        }

        if self.is_looking_up {
            self.angle_y += 0.1;
        }
        if self.is_looking_down {
            self.angle_y -= 0.1;
        }

        if self.is_roll_left {
            self.angle_x += 0.1;
        }
        if self.is_roll_right {
            self.angle_x -= 0.1;
        }

        if self.is_looking_left {
            self.angle_z -= 0.1;
        }
        if self.is_looking_right {
            self.angle_z += 0.1;
        }        
    }
}
