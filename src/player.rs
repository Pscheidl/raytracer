
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

        let delta_z: f64 = 2.0;
        self.projectiles.clear();

        let mut vector_len_coef = 5.0; // lower to increase FPS (1 is minimum, 5 for better quality)
        if self.is_low_detail_render {
            vector_len_coef = 1.0;
        }
        for ray_y in -250..250 { // WINDOW_HEIGHT
            let delta_y = ray_y as f64 / 100.0;
            let mut projectile_row: Vec<Projectile> = Vec::new();

            for ray_x in -250..250 { // WINDOW_WIDTH
                let delta_x: f64 = ray_x as f64 / 100.0;
                let vec_len = (delta_x.powf(2.0) + delta_y.powf(2.0) + delta_z.powf(2.0)).sqrt() * vector_len_coef;
                
                let norm_delta_x = delta_x / vec_len;
                let norm_delta_y = delta_y / vec_len;
                let norm_delta_z = delta_z / vec_len;

                let rot_x_delta_x = norm_delta_x;
                let rot_x_delta_y = norm_delta_y*self.angle_y.cos() - norm_delta_z*self.angle_y.sin();
                let rot_x_delta_z = norm_delta_y*self.angle_y.sin() + norm_delta_z*self.angle_y.cos();

                let rot_x_y_delta_x = rot_x_delta_x*self.angle_x.cos() - rot_x_delta_y*self.angle_x.sin();
                let rot_x_y_delta_y = rot_x_delta_x*self.angle_x.sin() + rot_x_delta_y*self.angle_x.cos();
                let rot_x_y_delta_z = rot_x_delta_z;

                let rot_x_y_z_delta_x = rot_x_y_delta_x*self.angle_z.cos() + rot_x_y_delta_z*self.angle_z.sin();
                let rot_x_y_z_delta_y = rot_x_y_delta_y;
                let rot_x_y_z_delta_z = -rot_x_y_delta_x*self.angle_z.sin() + rot_x_y_delta_z*self.angle_z.cos();

                projectile_row.push(Projectile::new(
                    self.x,
                    self.y,
                    self.z,
                    rot_x_y_z_delta_x,
                    rot_x_y_z_delta_y,
                    rot_x_y_z_delta_z,
                    1.0 // 1.0 max brightness 0.0 dead
                ));
            }
            self.projectiles.push(projectile_row);
        }
    }
}
