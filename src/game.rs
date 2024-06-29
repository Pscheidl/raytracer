use crate::player;
use crate::enemy;
use crate::light_ray::LightRay;
use crate::room;

use piston_window::types::Color;
use rayon::prelude::*;

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
        enemies.push(enemy::Enemy::new(250.0, 45.0, 220.0, 30.0, 1000, enemy::EnemyType::Sphere, -5.0, 0.0, 0.0));
        enemies.push(enemy::Enemy::new(250.0, 45.0, 50.0, 30.0, 1000, enemy::EnemyType::Sphere, -5.0, 0.0, 0.0));
        enemies.push(enemy::Enemy::new(55.0, 35.0, 250.0, 30.0, 1000, enemy::EnemyType::Sphere, 5.0, 0.0, 0.0));

        Game {
            player: player::Player::new(
                140 as f64,
                60 as f64,
                155.0,
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
            _ => {}
        };
    }  

    /// Draws entire world.
    pub fn compute_one_tick(&mut self) -> Vec<Vec<Color>> {

        // setup world
        for enemy in self.enemies.iter_mut() {
            enemy.move_enemy(self.room.x, self.room.y, self.room.z);
        }
        
        // iterate over all projectiles 
        let canvas_vec: Vec<Vec<Color>> = self.player.projectiles.par_iter_mut().map(|projectile_row| {
            let mut canvas_line: Vec<Color> = [[0.0, 0.0, 0.0, 0.0]; 500].to_vec();
            for (index_column, projectile) in projectile_row.iter_mut().enumerate() {
                
                // helper vars for each projectile
                let current_ray = LightRay::new(*projectile);
                let current_ray = current_ray.find_wall_color(&self.room, &self.enemies);
                
                if true { //TODO high detail option
                    canvas_line[index_column] = current_ray.compute_shadows(&self.room, &self.enemies);
                } else {
                    canvas_line[index_column] = current_ray.skip_shadows();
                }
            }
            canvas_line
        }).collect();
        canvas_vec
    }
}