use std::f64::consts::PI;

use super::WINDOW_HEIGHT;
use super::WINDOW_WIDTH;
use crate::enemy::Enemy;
use crate::player;
use crate::drawing::draw_rectange;
use crate::enemy;
use crate::projectile;
use piston_window::color::{WHITE, RED, BLUE, GREEN, YELLOW, GRAY};
use crate::projectile::Projectile;
use rayon::iter::Enumerate;
use rayon::prelude::*;

const WINDOW_X_OFFSET: f64 = 950.0; //256;
const WINDOW_Y_OFFSET: f64 = 300.0; //192;
const FRAME_BUFFER_X: usize = 2; 
const FRAME_BUFFER_Y: usize = 2; 

pub struct Game {
    // World buffers
    pub frame_buffer: [[bool; FRAME_BUFFER_Y]; FRAME_BUFFER_X],
    pub frame_buffer_next_tick: [[bool; FRAME_BUFFER_Y]; FRAME_BUFFER_X],
    pub player: player::Player,
    pub enemies: Vec<enemy::Enemy>,
    pub enemy_spawn_difficulty: usize,
    pub enemy_spawn_ticks: usize,
}

impl Game {
    pub fn new() -> Game {
        // randomize world
        let temp_world = [[false; FRAME_BUFFER_Y]; FRAME_BUFFER_X];
        let mut enemies =  Vec::new();
        /*
        E N E M Y
        */

        /* Y - | +down
           X - -  +right
           Z - / +far*/
        enemies.push(enemy::Enemy::new(-50.0, -50.0, 0.0, 100.0, 1000, enemy::EnemyType::Sphere));
        enemies.push(enemy::Enemy::new(0.0, 0.0, 0.0, 50.0, 1000, enemy::EnemyType::Sphere));
        enemies.push(enemy::Enemy::new(50.0,50.0, 0.0, 75.0, 1000, enemy::EnemyType::Sphere));
        Game {
            frame_buffer: temp_world,
            frame_buffer_next_tick: [[false; FRAME_BUFFER_Y]; FRAME_BUFFER_X],
            player: player::Player::new(
                0 as f64,
                0 as f64,
                -59.0, // ROOM SIZE - 1
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
                [[Projectile::new(0.0,0.0,0.0,0.0,0.0); 250]; 250],
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
            piston_window::Key::Q => self.player.is_moving_forward = true,
            piston_window::Key::E => self.player.is_moving_backward = true,
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
            piston_window::Key::Q => self.player.is_moving_forward = false,
            piston_window::Key::E => self.player.is_moving_backward = false,
            piston_window::Key::RCtrl => self.player.is_shooting = false,
            piston_window::Key::LCtrl => self.player.is_shooting = false,
            _ => {}
        };
    }  

    /// Draws entire world.
    pub fn compute_one_tick(&mut self, con: &piston_window::Context,
         g: &mut piston_window::G2d) -> Vec<Vec<usize>> {

        //let mut result = [[0; 250]; 250];
        // Iterate over the world

        // player - no need to draw
        //let mut player_trans = con.transform.trans(self.player.x, self.player.y);
        //draw_rectange(BLUE, self.player.x, self.player.y, 10, 3, con, g);
        
        // spawn_new_rays
        //self.player.spawn_new_rays();

        // projectiles
        //let light_object: Enemy = enemy::Enemy::new(50.0, 50.0, 50.0, 1.0, 1000, enemy::EnemyType::Point);

        let object_size = 25.0;
        const room_size:f64 = 60.0;

        let canvas_vec: Vec<Vec<usize>> = self.player.projectiles.par_iter_mut().enumerate().map(|(index_row, projectile_row)| {
            let mut canvas_line: Vec<usize> = [0; 250].to_vec();
            for (index_column, projectile) in projectile_row.iter_mut().enumerate() {
                
                let mut is_enemy_found = false;
                
                for _ in 0..2000 { 

                    /*0 => GRAY,
                        1 => GREEN,
                        2 => RED,
                        3 => BLUE,
                        4 => YELLOW,
                        5 => CYAN,
                        6 => MAGENTA,
                        _ => WHITE, */
                        if projectile.x <= -room_size {
                            canvas_line[index_column] = 1;
                            break;   
                        }
                        if projectile.x >= room_size {
                            canvas_line[index_column] = 2;
                            break;                         
                        }

                        if projectile.z <= -room_size {
                            canvas_line[index_column] = 3;
                            break;                        
                        }
                        if projectile.z >= room_size {
                            canvas_line[index_column] = 4;
                            break;                            
                        }
                        if projectile.y <= -room_size {
                            canvas_line[index_column] = 5;
                            break;                            
                        }
                        if projectile.y >= room_size {
                            canvas_line[index_column] = 6;
                            break;                            
                        }


                        if !is_enemy_found {
                            //draw_rectange( GRAY, projectile.x , projectile.z, 1, 1, con, g); // DEBUG RAYS
    
                            let mut delta_z = 0.5;
                            let mut delta_x = projectile.yaw.tan() * delta_z;
                            let mut delta_y = projectile.pitch.tan() * delta_z;
    
                            let vec_len = (delta_x.powf(2.0) + delta_y.powf(2.0) + delta_z.powf(2.0)).sqrt();
    
                            delta_x /= vec_len;
                            delta_y /= vec_len;
                            delta_z /= vec_len;
    
                            projectile.x += delta_x;  //cos or sin
                            projectile.z += delta_z;
                            projectile.y += delta_y;
                        } else {
                            let mut delta_x = projectile.yaw.cos();
                            let mut delta_y = projectile.pitch.sin();
                            let mut delta_z = projectile.yaw.sin();                        
                            
                            let vec_len = (delta_x.powf(2.0) + delta_y.powf(2.0) + delta_z.powf(2.0)).sqrt();
    
                            delta_x /= vec_len;
                            delta_y /= vec_len;
                            delta_z /= vec_len;
    
                            projectile.x += delta_x;
                            projectile.y += delta_y;
                            projectile.z += delta_z;
                            
                            if index_row == 25 {
                                // X cor reflected rays
                                //draw_rectange( BLUE, projectile.x + WINDOW_X_OFFSET, projectile.z, 1, 1, con, g);
                            }
                            if index_column == 20 {
                                // Y cor reflected rays
                                //draw_rectange( YELLOW, projectile.y + WINDOW_X_OFFSET + WINDOW_Y_OFFSET, projectile.z, 1, 1, con, g);
                            }
                            continue;           
                        }
                        
                        for (index, enemy) in self.enemies.iter().enumerate() {   
                                                    
                            let enemy_x_moved_tip = enemy.x - self.player.x - (PI/2.0 as f64).cos()*object_size;
                            let enemy_z_moved_tip = enemy.z - self.player.z - (PI/2.0 as f64).sin()*object_size;
                            let enemy_y_moved_tip = enemy.y - self.player.y - (PI/2.0 as f64).cos()*object_size;
                            let enemy_x_moved_core = enemy.x - self.player.x;
                            let enemy_z_moved_core = enemy.z - self.player.z;
                            let enemy_y_moved_core = enemy.y - self.player.y;
                           
                            let len_from_core = ((enemy_x_moved_core-projectile.x).powf(2.0) + (enemy_y_moved_core-projectile.y).powf(2.0) + (enemy_z_moved_core-projectile.z).powf(2.0)).sqrt();
    
                            if index_row == 25 {                     
                                //draw_rectange( YELLOW, enemy_x_moved_core + WINDOW_X_OFFSET, enemy_z_moved_core, 1, 1, con, g); // center of circle
                                //draw_rectange( YELLOW, enemy_x_moved_tip + WINDOW_X_OFFSET , enemy_z_moved_tip, 1, 1, con, g); // top point on the circle
                            }
                            if index_column == 25 {
                                //draw_rectange( YELLOW, enemy_y_moved_core + WINDOW_X_OFFSET + WINDOW_Y_OFFSET, enemy_z_moved_core, 1, 1, con, g); // center of circle
                                //draw_rectange( YELLOW, enemy_y_moved_tip + WINDOW_X_OFFSET + WINDOW_Y_OFFSET, enemy_z_moved_tip, 1, 1, con, g); // top point on the circle
                            }
                            
                            if len_from_core + 0.5 >= object_size && len_from_core - 0.5 <= object_size
                            && !is_enemy_found {
                                if index_row == 25 {                     
                                    //draw_rectange( RED, enemy_x_moved_core + WINDOW_X_OFFSET, enemy_z_moved_core, 2, 2, con, g); // center of circle
                                    //draw_rectange( RED, enemy_x_moved_tip + WINDOW_X_OFFSET , enemy_z_moved_tip, 2, 2, con, g); // top point on the circle
                                }
                                if index_column == 25 {
                                    //draw_rectange( RED, enemy_y_moved_core + WINDOW_X_OFFSET + WINDOW_Y_OFFSET, enemy_z_moved_core, 2, 2, con, g); // center of circle
                                    //draw_rectange( RED, enemy_y_moved_tip + WINDOW_X_OFFSET + WINDOW_Y_OFFSET, enemy_z_moved_tip, 2, 2, con, g); // top point on the circle
                                }
                                
                                is_enemy_found = true;
                                projectile.yaw += PI / 2.0; // aim back
                                projectile.pitch += PI / 2.0; // aim back
                                
                                let len_from_tip = ((enemy_x_moved_tip-projectile.x).powf(2.0) + (enemy_y_moved_tip-projectile.y).powf(2.0) + (enemy_z_moved_tip-projectile.z).powf(2.0)).sqrt();
                                // tilt rays based on side it comes from
                                if projectile.x > enemy_x_moved_core {
                                    projectile.yaw += PI/object_size*len_from_tip/2.0;
                                } else {
                                    projectile.yaw -= PI/object_size*len_from_tip/2.0;
                                }
                                if projectile.y > enemy_y_moved_core {
                                    projectile.pitch += PI/object_size*len_from_tip/2.0;
                                } else {
                                    projectile.pitch -= PI/object_size*len_from_tip/2.0;
                                }    
                            }
                        }
                }
            }
            canvas_line
        }).collect();
        canvas_vec
    }
}
