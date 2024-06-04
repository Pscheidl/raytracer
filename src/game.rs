use std::f64::consts::PI;

use super::WINDOW_HEIGHT;
use super::WINDOW_WIDTH;
use crate::enemy::Enemy;
use crate::player;
use crate::drawing::draw_rectange;
use crate::enemy;
use crate::player::Player;
use crate::projectile;
use piston_window::color::{WHITE, RED, BLUE, GREEN, YELLOW, GRAY};
use piston_window::types::Color;
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
        //enemies.push(enemy::Enemy::new(250.0, 200.0, 300.0, 100.0, 1000, enemy::EnemyType::Sphere, -5.0));
        //enemies.push(enemy::Enemy::new(250.0, 200.0, 300.0, 100.0, 1000, enemy::EnemyType::Sphere, -3.0));
        enemies.push(enemy::Enemy::new(200.0, 200.0, 300.0, 100.0, 1000, enemy::EnemyType::Sphere, 5.0));

        Game {
            frame_buffer: temp_world,
            frame_buffer_next_tick: [[false; FRAME_BUFFER_Y]; FRAME_BUFFER_X],
            player: player::Player::new(
                200 as f64,
                200 as f64,
                 2.0, // ROOM SIZE - 1
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
                [[Projectile::new(0.0,0.0,0.0,0.0,0.0,0.0, 1.0); 250]; 250],
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
         g: &mut piston_window::G2d) -> Vec<Vec<Color>> {

        
        for (index, enemy) in self.enemies.iter_mut().enumerate() {
            enemy.move_enemy(ROOM_SIZE);
        }

        const ROOM_SIZE:f64 = 400.0;

        let canvas_vec: Vec<Vec<Color>> = self.player.projectiles.par_iter_mut().enumerate().map(|(index_row, projectile_row)| {
            let mut canvas_line: Vec<Color> = [[0.0, 0.0, 0.0, 0.0]; 250].to_vec();
            for (index_column, projectile) in projectile_row.iter_mut().enumerate() {
                
                let mut is_enemy_found = false;
                
                for _ in 0..20000 { 

                    if projectile.x <= 0.0 { // right
                        if ((projectile.y as i32/25) % 2 == 0) {
                            canvas_line[index_column] = [projectile.time_to_live as f32, 0.0, 0.0, 1.0]; // red
                        } else if ((projectile.z as i32/25) % 2 == 0) {
                            canvas_line[index_column] = [projectile.time_to_live as f32, 0.2, 0.2, 1.0]; // light red
                        } else {
                            canvas_line[index_column] = [projectile.time_to_live as f32, 0.4, 0.4, 1.0]; // lighter red
                        }
                        
                        break;   
                    }
                    if projectile.x >= ROOM_SIZE { // left
                        
                        if ((projectile.y as i32/25) % 2 == 0) {
                            canvas_line[index_column] = [0.0, projectile.time_to_live as f32, 0.0, 1.0]; // green
                        } else if ((projectile.z as i32/25) % 2 == 0) {
                            canvas_line[index_column] = [0.2, projectile.time_to_live as f32, 0.2, 1.0]; // light green
                        } else {
                            canvas_line[index_column] = [0.4, projectile.time_to_live as f32, 0.4, 1.0]; // light green
                        }
                        break;                         
                    }

                    if projectile.z <= 0.0 { // top                        
                        if ((projectile.x as i32/25) % 2 == 0) {
                            canvas_line[index_column] = [0.0, projectile.time_to_live as f32, projectile.time_to_live as f32, 1.0];  // cyan  
                        } else if ((projectile.y as i32/25) % 2 == 0) {
                            canvas_line[index_column] = [0.2, projectile.time_to_live as f32, projectile.time_to_live as f32, 1.0];  // light cyan  
                        } else {
                            canvas_line[index_column] = [0.4, projectile.time_to_live as f32, projectile.time_to_live as f32, 1.0];  // lighter cyan  
                        }
                                              
                        break;                        
                    }
                    if projectile.z >= ROOM_SIZE { // bottom
                        if ((projectile.x as i32/25) % 2 == 0) {
                            canvas_line[index_column] = [projectile.time_to_live as f32, projectile.time_to_live as f32, 0.0, 1.0];  // yellow
                        } else if ((projectile.y as i32/25) % 2 == 0) {
                            canvas_line[index_column] = [projectile.time_to_live as f32, projectile.time_to_live as f32, 0.2, 1.0];  // light yellow
                        } else {
                            canvas_line[index_column] = [projectile.time_to_live as f32, projectile.time_to_live as f32, 0.4, 1.0];  // lighter yellow
                        }
                        
                        break;                            
                    }
                    if projectile.y <= 0.0 { // front
                       
                        if ((projectile.x as i32/25) % 2 == 0) {
                            canvas_line[index_column] = [projectile.time_to_live as f32, 0.0, projectile.time_to_live as f32, 1.0];  // pink
                        } else if ((projectile.z as i32/25) % 2 == 0) {
                            canvas_line[index_column] = [projectile.time_to_live as f32, 0.2, projectile.time_to_live as f32, 1.0];  // light pink
                        } else {
                            canvas_line[index_column] = [projectile.time_to_live as f32, 0.4, projectile.time_to_live as f32, 1.0];  // lighter pink
                        }
                        
                                                
                        break;                            
                    }
                    if projectile.y >= ROOM_SIZE { // back
                        if ((projectile.x as i32/25) % 2 == 0) {
                            canvas_line[index_column] = [0.0, 0.0, projectile.time_to_live as f32, 1.0]; // blue
                        } else if ((projectile.z as i32/25) % 2 == 0) {
                            canvas_line[index_column] = [0.2, 0.2, projectile.time_to_live as f32, 1.0]; // light blue    
                        } else {
                            canvas_line[index_column] = [0.4, 0.4, projectile.time_to_live as f32, 1.0]; // lighter blue    
                        }      
                        break;                            
                    }                    

                    projectile.x += projectile.dx;  //cos or sin
                    projectile.y += projectile.dy;
                    projectile.z += projectile.dz;

                    projectile.time_to_live -= 0.0002;
                    
                    if (is_enemy_found) {
                        continue;
                    }

                    for (index, enemy) in self.enemies.iter().enumerate() {                        

                        

                        //print!("{}", enemy.x);
                        let object_size = enemy.size;
                        
                        let enemy_x_moved_core = enemy.x;          
                        let enemy_y_moved_core = enemy.y;
                        let enemy_z_moved_core = enemy.z;
                        
                        let len_from_core = ((enemy_x_moved_core-projectile.x).powf(2.0) + (enemy_y_moved_core-projectile.y).powf(2.0) + (enemy_z_moved_core-projectile.z).powf(2.0)).sqrt();
                        
                        
                        if len_from_core + 0.3 >= object_size && len_from_core - 0.3 <= object_size
                        && !is_enemy_found {

                            is_enemy_found = true;

                            let ball_vec_x = enemy.x - projectile.x;
                            let ball_vec_y = enemy.y - projectile.y;
                            let ball_vec_z = enemy.z - projectile.z;

                            let ball_vec_len = (ball_vec_x.powf(2.0) + ball_vec_y.powf(2.0) + ball_vec_z.powf(2.0)).sqrt();
                            
                            let ball_vec_norm_x = ball_vec_x / ball_vec_len;
                            let ball_vec_norm_y = ball_vec_y / ball_vec_len;
                            let ball_vec_norm_z = ball_vec_z / ball_vec_len;
    
                            // R=V−2N(V⋅N)
                            // R=RAY-2*NORMAL(RAY*NORMAL)
                            projectile.dx = projectile.dx - 2.0*ball_vec_norm_x*(projectile.dx*ball_vec_norm_x);
                            projectile.dy = projectile.dy - 2.0*ball_vec_norm_y*(projectile.dy*ball_vec_norm_y);
                            projectile.dz = projectile.dz - 2.0*ball_vec_norm_z*(projectile.dz*ball_vec_norm_z);

                        }
                        
                    }
                }
            }
            canvas_line
        }).collect();
        canvas_vec
    }

    pub fn getPitchAndYawBounceFromBall(projectile: Projectile, ball: Enemy, len_from_core: f64) -> (f64,f64) {
        /*
        TODO
        Ball can be simplified as a circle for collision purposes.
         */
        let core_ball_x = ball.x;
        let core_ball_y = ball.y;
        let core_ball_z = ball.z;

        let hit_ball_x = projectile.x;
        let hit_ball_y = projectile.y;
        let hit_ball_z = projectile.z;
        
        (0.0, 0.0)
    }    
}

#[cfg(test)]
mod tests {
    use crate::{Game, projectile::Projectile, enemy::{Enemy}};

    #[test]
    fn collision_test() {
        let projectile = Projectile{x: 1.0, y: 1.0, z: 1.0, dx:0.0, dy:0.0, dz:0.0, time_to_live: 1.0};
        let enemy = Enemy{ x: 0.0, y: 0.0, z: 0.0, size: 2.0, time_to_live: 1000, enemy_type: crate::enemy::EnemyType::Sphere, moving_left_speed: 0.1};
        let result = Game::getPitchAndYawBounceFromBall(projectile, enemy, 1.0);
        assert_eq!(result, (1.0, 1.0));
    }
}
