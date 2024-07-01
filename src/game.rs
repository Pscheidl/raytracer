use std::ops::Deref;

use crate::player;
use crate::enemy;
use crate::light_ray::LightRay;
use crate::projectile::Projectile;
use crate::room;

use smallvec::{SmallVec, smallvec};
use piston_window::types::Color;
use rayon::prelude::*;

const ARRAY_SIZE: usize = 512;

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
        enemies.push(enemy::Enemy::new(255.0, 100.0, 220.0, 30.0, 1000, enemy::EnemyType::Sphere, 5.0, 0.0, 0.0));
        enemies.push(enemy::Enemy::new(250.0, 45.0, 200.0, 30.0, 1000, enemy::EnemyType::Sphere, -5.0, 0.0, 0.0));
        enemies.push(enemy::Enemy::new(250.0, 45.0, 50.0, 30.0, 1000, enemy::EnemyType::Sphere, -5.0, 0.0, 0.0));
        enemies.push(enemy::Enemy::new(55.0, 35.0, 280.0, 30.0, 1000, enemy::EnemyType::Sphere, 5.0, 0.0, 0.0));

        Game {
            player: player::Player::new(
                140 as f64,
                60 as f64,
                155.0,
                0.0,
                0.0,
                0.0,
                10.0,
                10.0,
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
                Vec::new(),
            ),
            enemies: enemies,
            room: room::Room::new(300.0,150.0,400.0)
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
    pub fn compute_one_tick(&mut self) -> SmallVec<[Box<SmallVec<[Color;ARRAY_SIZE]>>;ARRAY_SIZE]> {

        // setup world
        for enemy in self.enemies.iter_mut() {
            enemy.move_enemy(self.room.x, self.room.y, self.room.z);
        }
        
        let delta_z: f64 = 2.0;
        //self.projectiles.clear();S

        let mut vector_len_coef = 5.0; // lower to increase FPS (1 is minimum, 5 for better quality)
        if self.player.is_low_detail_render {
            vector_len_coef = 1.0;
        }

        let mut canvas_vec: SmallVec<[Box<SmallVec<[Color;ARRAY_SIZE]>>;ARRAY_SIZE]> = smallvec![Box::new(smallvec![[0.0, 0.0, 0.0, 0.0];ARRAY_SIZE]); ARRAY_SIZE];
        // iterate over all projectiles 
        canvas_vec.par_iter_mut().enumerate().for_each(|(idx, canvas_line)| {

            let ray_y:i32 = idx as i32 - 256;
            let delta_y = ray_y as f64 / 100.0;
            let canvas_line = canvas_line.as_mut();

            for (index_column, ray_x) in (-256..256).enumerate() { // WINDOW_WIDTH
                let delta_x: f64 = ray_x as f64 / 100.0;
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
                    1.0 // 1.0 max brightness 0.0 dead
                );
                let current_ray = LightRay::new(projectile);
                let current_ray = current_ray.find_wall_color(&self.room, &self.enemies);
                
                if self.player.is_low_detail_render {
                    canvas_line[index_column] = current_ray.skip_shadows();
                } else {
                    canvas_line[index_column] = current_ray.compute_shadows(&self.room, &self.enemies);
                }
            }
        });
        canvas_vec
    }
}