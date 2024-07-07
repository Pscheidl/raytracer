use crate::math::Vector3D;
use crate::player;
use crate::enemy;
use crate::light_ray::LightRay;
use crate::projectile::Projectile;
use crate::room;
use crate::room::LightSource;

use image::Rgba;
use image::RgbaImage;

use rayon::prelude::*;
use crate::{CANVAS_WIDTH, CANVAS_HEIGHT, CANVAS_WIDTH_HALF, CANVAS_HEIGHT_HALF};


pub struct Game {
    // World buffers
    pub player: player::Player,
    pub enemies: Vec<enemy::Enemy>,
    pub room: room::Room,
}

impl Game {
    pub fn new() -> Game {
        /*
        Space orientation:

           Y - | +down
           X - -  +right
           Z - / +far
        */

        let mut enemies =  Vec::new();
        
        const SPHERE_SIZE: f64 = 30.0;
        const SPHERE_SIZE_PLUS_MARGIN: f64 = SPHERE_SIZE + 10.0;
        enemies.push(enemy::Enemy::new(
            crate::ROOM_SIZE_X / 2.0,
            0.0 + SPHERE_SIZE_PLUS_MARGIN,
            crate::ROOM_SIZE_Z / 3.0 * 2.0,
            SPHERE_SIZE,
            enemy::EnemyType::Sphere,
            enemy::ENEMY_SPEED,
            0.0,
            0.0));   

        enemies.push(enemy::Enemy::new(
            SPHERE_SIZE_PLUS_MARGIN,
            crate::ROOM_SIZE_Y - SPHERE_SIZE_PLUS_MARGIN,
            crate::ROOM_SIZE_Z / 2.0,
            SPHERE_SIZE,
            enemy::EnemyType::Sphere,
            0.0,
            0.0,
            -enemy::ENEMY_SPEED));

        enemies.push(enemy::Enemy::new(
            crate::ROOM_SIZE_X - SPHERE_SIZE_PLUS_MARGIN,
            crate::ROOM_SIZE_Y - SPHERE_SIZE_PLUS_MARGIN,
            crate::ROOM_SIZE_Z / 2.0,
            SPHERE_SIZE,
            enemy::EnemyType::Sphere,
            0.0,
            0.0,
            enemy::ENEMY_SPEED));



        Game {
            player: player::Player::new(
                crate::ROOM_SIZE_X/2.0_f64,
                crate::ROOM_SIZE_Y/3.0_f64,
                1.0,
                0.0,
                0.0,
                0.0,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
            ),
            enemies,
            room: room::Room::new(
                Vector3D([crate::ROOM_SIZE_X, crate::ROOM_SIZE_Y, crate::ROOM_SIZE_Z]),
                LightSource::new(75_f64,
                    2_f64,
                    Vector3D([crate::ROOM_SIZE_X / 2.0, 0.0, crate::ROOM_SIZE_Z / 2.0]),
                    Vector3D([1.0,0.0,1.0])),
                )
        }
    }
    pub fn key_pressed(&mut self, key: piston_window::Key) {
        match key {
            piston_window::Key::Up => self.player.is_moving_up = true,
            piston_window::Key::Down => self.player.is_moving_down = true,
            piston_window::Key::W => self.player.is_moving_up = true,
            piston_window::Key::S => self.player.is_moving_down = true,
            piston_window::Key::Left => self.player.is_moving_left = true,
            piston_window::Key::A => self.player.is_moving_left = true,
            piston_window::Key::Right => self.player.is_moving_right = true,
            piston_window::Key::D => self.player.is_moving_right = true,
            piston_window::Key::Q => self.player.is_moving_forward = true,
            piston_window::Key::E => self.player.is_moving_backward = true,
            piston_window::Key::L => self.player.is_low_detail_render = true,
            piston_window::Key::R => self.player.is_looking_up = true,
            piston_window::Key::F => self.player.is_looking_down = true,
            piston_window::Key::T => self.player.is_roll_left = true,
            piston_window::Key::G => self.player.is_roll_right = true,
            piston_window::Key::C => self.player.is_looking_left = true,
            piston_window::Key::V => self.player.is_looking_right = true,
            _ => {}
        };
    }
    pub fn key_released(&mut self, key: piston_window::Key) {
        match key {
            piston_window::Key::Up => self.player.is_moving_up = false,
            piston_window::Key::Down => self.player.is_moving_down = false,
            piston_window::Key::W => self.player.is_moving_up = false,
            piston_window::Key::S => self.player.is_moving_down = false,
            piston_window::Key::Left => self.player.is_moving_left = false,
            piston_window::Key::A => self.player.is_moving_left = false,
            piston_window::Key::Right => self.player.is_moving_right = false,
            piston_window::Key::D => self.player.is_moving_right = false,
            piston_window::Key::Q => self.player.is_moving_forward = false,
            piston_window::Key::E => self.player.is_moving_backward = false,
            piston_window::Key::H => self.player.is_low_detail_render = false,
            piston_window::Key::R => self.player.is_looking_up = false,
            piston_window::Key::F => self.player.is_looking_down = false,
            piston_window::Key::T => self.player.is_roll_left = false,
            piston_window::Key::G => self.player.is_roll_right = false,
            piston_window::Key::C => self.player.is_looking_left = false,
            piston_window::Key::V => self.player.is_looking_right = false,
            _ => {}
        };
    }  

    /// Draws entire world.
    pub fn compute_one_tick(&mut self) -> image::ImageBuffer<Rgba<u8>, Vec<u8>> {

        // setup world
        for enemy in self.enemies.iter_mut() {
            enemy.move_enemy(self.room.size.0[0], self.room.size.0[1], self.room.size.0[2]);
        }

        self.room.light_source.tick();
        
        let delta_z: f64 = 3.0;

        let mut vector_len_coef = 5.0; // lower to increase FPS (1 is minimum, 5 for better quality)
        if self.player.is_low_detail_render {
            vector_len_coef = 1.0;
        }
        
        let mut img: image::ImageBuffer<Rgba<u8>, Vec<u8>> = RgbaImage::new(CANVAS_WIDTH as u32, CANVAS_HEIGHT as u32);

        // iterate over all projectiles 
        img.par_enumerate_pixels_mut().for_each(|(pixel_pos_y, pixel_pos_x, pixel)| {

            let delta_y: f64 = (-(CANVAS_WIDTH_HALF as f64) + (pixel_pos_x as f64)) / 100.0;            
            let delta_x: f64 = (-(CANVAS_HEIGHT_HALF as f64) + (pixel_pos_y as f64)) / 100.0;
            let vec_len = (delta_x.powf(2.0) + delta_y.powf(2.0) + delta_z.powf(2.0)).sqrt() * vector_len_coef;
            
            let norm_delta_x = delta_x / vec_len;
            let norm_delta_y = delta_y / vec_len;
            let norm_delta_z = delta_z / vec_len;

            let rot_x_delta_x = norm_delta_x;
            let rot_x_delta_y = norm_delta_y*self.player.angle_y.cos() - norm_delta_z*self.player.angle_y.sin();
            let rot_x_delta_z = norm_delta_y*self.player.angle_y.sin() + norm_delta_z*self.player.angle_y.cos();

            let rot_x_y_delta_x = rot_x_delta_x*self.player.angle_x.cos() - rot_x_delta_y*self.player.angle_x.sin();
            let rot_x_y_delta_y = rot_x_delta_x*self.player.angle_x.sin() + rot_x_delta_y*self.player.angle_x.cos();
            let rot_x_y_delta_z = rot_x_delta_z;

            let rot_x_y_z_delta_x = rot_x_y_delta_x*self.player.angle_z.cos() + rot_x_y_delta_z*self.player.angle_z.sin();
            let rot_x_y_z_delta_y = rot_x_y_delta_y;
            let rot_x_y_z_delta_z = -rot_x_y_delta_x*self.player.angle_z.sin() + rot_x_y_delta_z*self.player.angle_z.cos();

            let projectile = Projectile::new(
                self.player.x,
                self.player.y,
                self.player.z,
                rot_x_y_z_delta_x,
                rot_x_y_z_delta_y,
                rot_x_y_z_delta_z,
            );
            let current_ray = LightRay::new(projectile);
            let current_ray = current_ray.find_wall_color(&self.room, &self.enemies);
            
            if self.player.is_low_detail_render {
                pixel.0 = current_ray.skip_shadows();
            } else {                    
                pixel.0 = current_ray.compute_shadows(&self.room, &self.enemies);
            }
        });
        img
    }
}