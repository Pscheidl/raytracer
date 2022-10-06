use std::f64::consts::PI;

use super::WINDOW_HEIGHT;
use super::WINDOW_WIDTH;
use crate::enemy::Enemy;
use crate::player;
use crate::drawing::draw_rectange;
use crate::enemy;
use piston_window::color::{WHITE, RED, BLUE, GREEN, YELLOW, GRAY};

pub struct Game {
    // World buffers
    pub frame_buffer: [[bool; WINDOW_HEIGHT]; WINDOW_WIDTH],
    pub frame_buffer_next_tick: [[bool; WINDOW_HEIGHT]; WINDOW_WIDTH],
    pub player: player::Player,
    pub enemies: Vec<enemy::Enemy>,
    pub enemy_spawn_difficulty: usize,
    pub enemy_spawn_ticks: usize,
}

impl Game {
    pub fn new() -> Game {
        // randomize world
        let temp_world = [[false; WINDOW_HEIGHT]; WINDOW_WIDTH];
        let mut enemies =  Vec::new();
        enemies.push(enemy::Enemy::new(250.0, 500.0, 150.0, 10.0, 1000, enemy::EnemyType::Sphere));
        //enemies.push(enemy::Enemy::new(200.0, 500.0, 50.0, 10.0, 1000, enemy::EnemyType::Sphere)); 
        //enemies.push(enemy::Enemy::new(300.0, 500.0, 100.0, 10.0, 1000, enemy::EnemyType::Sphere)); 
        Game {
            frame_buffer: temp_world,
            frame_buffer_next_tick: [[false; WINDOW_HEIGHT]; WINDOW_WIDTH],
            player: player::Player::new(
            WINDOW_WIDTH as f64/ 2.0,
            WINDOW_HEIGHT as f64/ 2.0,
            0.0,
            10.0,
            10.0,
            false,
            false,
            false,
            false,
            false,
            Vec::new(),
            50,            
            ),
            enemies: enemies,
            enemy_spawn_ticks: 150,
            enemy_spawn_difficulty: 50,
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
            piston_window::Key::RCtrl => self.player.is_shooting = true,
            piston_window::Key::LCtrl => self.player.is_shooting = true,
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
            piston_window::Key::RCtrl => self.player.is_shooting = false,
            piston_window::Key::LCtrl => self.player.is_shooting = false,
            _ => {}
        };
    }  

    /// Draws entire world.
    pub fn compute_one_tick(&mut self, con: &piston_window::Context,
         g: &mut piston_window::G2d) -> Vec<usize> {

        let mut result = Vec::<usize>::new();
        // Iterate over the world

        // player - no need to draw
        //let mut player_trans = con.transform.trans(self.player.x, self.player.y);
        //draw_rectange(BLUE, self.player.x, self.player.y, 10, 3, con, g);
        
        // spawn_new_rays
        self.player.spawn_new_rays();

        // projectiles
        //let light_object: Enemy = enemy::Enemy::new(50.0, 50.0, 50.0, 1.0, 1000, enemy::EnemyType::Point);

        let object_size = 25.0;
        
        
        for (index, projectile) in self.player.projectiles.iter_mut().enumerate() {
            result.push(0);
            
            let mut is_enemy_found = false;
            
            for _ in 0..1000 { 
                                
                if !is_enemy_found {
                    //draw_rectange( GRAY, projectile.x , projectile.z, 1, 1, con, g); // DEBUG RAYS
                    let delta_z = 0.5;
                    let delta_x = projectile.yaw.tan() * delta_z;
                    projectile.x += delta_x;  //cos or sin
                    projectile.z += delta_z;
                } else {
                    if projectile.x <= 10.0 {
                        result[index] = 1;
                        break;
                    }
                    if projectile.x >= 200.0 {
                        result[index] = 2;
                        break;
                    }

                    if projectile.z <= 10.0 {
                        result[index] = 3;
                        break;
                    }
                    if projectile.z >= 200.0 {
                        result[index] = 4;
                        break;
                    }
                    projectile.x += projectile.yaw.cos();
                    projectile.z += projectile.yaw.sin();
                    draw_rectange( BLUE, projectile.x , projectile.z, 1, 1, con, g);
                    continue;           
                }
                
                for (index, enemy) in self.enemies.iter_mut().enumerate() {   
                    /*let enemy_x_from_player = enemy.x - self.player.x;
                    let enemy_z_from_player = enemy.z - self.player.z;

                    if enemy_x_from_player - enemy.size < projectile.x 
                    && enemy_x_from_player + enemy.size > projectile.x 
                    && enemy_z_from_player - enemy.size < projectile.z 
                    && enemy_z_from_player + enemy.size > projectile.z 
                    && !is_enemy_found {*/
                    let enemy_x_moved_tip = enemy.x - self.player.x - (PI/2.0 as f64).cos()*object_size;
                    let enemy_z_moved_tip = enemy.z - self.player.z - (PI/2.0 as f64).sin()*object_size;
                    let enemy_x_moved_core = enemy.x - self.player.x;
                    let enemy_z_moved_core = enemy.z - self.player.z;
                    let len_from_core = ((enemy_x_moved_core-projectile.x).powf(2.0) + (enemy_z_moved_core-projectile.z).powf(2.0)).sqrt();
                                        
                    if len_from_core + 0.5 >= object_size && len_from_core - 0.5 <= object_size && !is_enemy_found {
                                                                  
                        draw_rectange( RED, enemy_x_moved_core , enemy_z_moved_core, 1, 1, con, g); // center of circle
                        draw_rectange( YELLOW, enemy_x_moved_tip , enemy_z_moved_tip, 1, 1, con, g); // top point on the circle
                        
                        is_enemy_found = true;
                        projectile.yaw += PI / 2.0; // aim back

                        let len_from_tip = ((enemy_x_moved_tip-projectile.x).powf(2.0) + (enemy_z_moved_tip-projectile.z).powf(2.0)).sqrt();
                        // tilt rays based on side it comes from
                        if projectile.x > enemy_x_moved_core {
                            projectile.yaw += PI/object_size*len_from_tip/2.0;
                        } else {
                            projectile.yaw -= PI/object_size*len_from_tip/2.0;
                        }                        
                    } 
                }
            }        
        }
        result
    }
}
