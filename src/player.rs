
use crate::{projectile::{Projectile, self}};

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
    pub is_low_detail_render: bool,
    pub projectiles: Vec::<Vec::<Projectile>>,
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
        is_low_detail_render: bool,
        projectiles: Vec::<Vec::<Projectile>>,
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
            self.y += 3.1;
        }
        if self.is_looking_down {
            self.y -= 3.1;
        }

        let delta_z: f64 = 2.0;
        self.projectiles.clear();

        let mut vector_len_coef = 5.0; // lower to increase FPS (1 is minimum, 5 for better quality)
        if self.is_low_detail_render {
            vector_len_coef = 1.0;
        }
        for ray_y in 0..500 { // WINDOW_HEIGHT
            let delta_y = -3.0 + 6.0 / 500.0 * ray_y as f64;   
            let mut projectile_row: Vec<Projectile> = Vec::new();

            for ray_x in 0..500 { // WINDOW_WIDTH
                let delta_x: f64 = -3.0 + 6.0 / 500.0 * ray_x as f64;                
                let vec_len = (delta_x.powf(2.0) + delta_y.powf(2.0) + delta_z.powf(2.0)).sqrt() * vector_len_coef;                
                
                projectile_row.push(Projectile::new(
                    self.x,
                    self.y,
                    self.z,
                    delta_x / vec_len,
                    delta_y / vec_len,
                    delta_z / vec_len,
                    1.0 // 1.0 max brightness 0.0 dead
                ));
            }
            self.projectiles.push(projectile_row);
        }
    }
}
