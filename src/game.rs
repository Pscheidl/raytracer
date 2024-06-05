use crate::player;

use crate::enemy;

use piston_window::types::Color;
use crate::projectile::Projectile;
use rayon::prelude::*;



pub struct Game {
    // World buffers
    pub player: player::Player,
    pub enemies: Vec<enemy::Enemy>,
    pub enemy_spawn_difficulty: usize,
    pub enemy_spawn_ticks: usize,
}

impl Game {
    pub fn new() -> Game {

        let mut enemies =  Vec::new();
        /*
        E N E M Y
        */

        /* Y - | +down
           X - -  +right
           Z - / +far*/
        //enemies.push(enemy::Enemy::new(250.0, 200.0, 300.0, 100.0, 1000, enemy::EnemyType::Sphere, -5.0));
        enemies.push(enemy::Enemy::new(110.0, 110.0, 300.0, 100.0, 1000, enemy::EnemyType::Sphere, 0.0, 5.0));
        enemies.push(enemy::Enemy::new(290.0, 290.0, 100.0, 100.0, 1000, enemy::EnemyType::Sphere, 0.0, 0.0));

        Game {
            player: player::Player::new(
                140 as f64,
                200 as f64,
                5.0,
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
    pub fn compute_one_tick(&mut self) -> Vec<Vec<Color>> {

        const ROOM_SIZE_X:f64 = 400.0;
        const ROOM_SIZE_Y:f64 = 450.0;
        const ROOM_SIZE_Z:f64 = 400.0;

        for enemy in self.enemies.iter_mut() {
            enemy.move_enemy(ROOM_SIZE_X, ROOM_SIZE_Y);
        }


        let canvas_vec: Vec<Vec<Color>> = self.player.projectiles.par_iter_mut().map(|projectile_row| {
            let mut canvas_line: Vec<Color> = [[0.0, 0.0, 0.0, 0.0]; 500].to_vec();
            for (index_column, projectile) in projectile_row.iter_mut().enumerate() {
                
                let mut last_ball_bounce = 255;
                
                for _x in 1..10000 { // not using loop for debug
                    let is_x_alternate = (projectile.x as i32/25) % 2 == 0;
                    let is_y_alternate = (projectile.y as i32/25) % 2 == 0;
                    let is_z_alternate = (projectile.z as i32/25) % 2 == 0;

                    if projectile.x <= 0.0 { // right
                        if is_y_alternate {
                            canvas_line[index_column] = [projectile.time_to_live as f32, 0.0, 0.0, 1.0]; // red    
                        } else if is_z_alternate {
                            canvas_line[index_column] = [projectile.time_to_live as f32, 0.2, 0.2, 1.0]; // light red
                        } else {
                            canvas_line[index_column] = [projectile.time_to_live as f32, 0.4, 0.4, 1.0]; // lighter red
                        }
                        
                        break;   
                    }
                    if projectile.x >= ROOM_SIZE_X { // left                        
                        if is_y_alternate {
                            canvas_line[index_column] = [0.0, projectile.time_to_live as f32, 0.0, 1.0]; // green
                        } else if is_z_alternate {
                            canvas_line[index_column] = [0.2, projectile.time_to_live as f32, 0.2, 1.0]; // light green
                        } else {
                            canvas_line[index_column] = [0.4, projectile.time_to_live as f32, 0.4, 1.0]; // light green
                        }
                        break;                         
                    }

                    if projectile.z <= 0.0 { // top                        
                        if is_x_alternate {
                            canvas_line[index_column] = [0.0, projectile.time_to_live as f32, projectile.time_to_live as f32, 1.0];  // cyan  
                        } else if is_y_alternate {
                            canvas_line[index_column] = [0.2, projectile.time_to_live as f32, projectile.time_to_live as f32, 1.0];  // light cyan  
                        } else {
                            canvas_line[index_column] = [0.4, projectile.time_to_live as f32, projectile.time_to_live as f32, 1.0];  // lighter cyan  
                        }                                              
                        break;                        
                    }
                    if projectile.z >= ROOM_SIZE_Z { // bottom
                        if is_x_alternate {
                            canvas_line[index_column] = [projectile.time_to_live as f32, projectile.time_to_live as f32, 0.0, 1.0];  // yellow
                        } else if is_y_alternate {
                            canvas_line[index_column] = [projectile.time_to_live as f32, projectile.time_to_live as f32, 0.2, 1.0];  // light yellow
                        } else {
                            canvas_line[index_column] = [projectile.time_to_live as f32, projectile.time_to_live as f32, 0.4, 1.0];  // lighter yellow
                        }                        
                        break;                            
                    }
                    if projectile.y <= 0.0 { // front                       
                        if is_x_alternate {
                            canvas_line[index_column] = [projectile.time_to_live as f32, 0.0, projectile.time_to_live as f32, 1.0];  // pink
                        } else if is_z_alternate {
                            canvas_line[index_column] = [projectile.time_to_live as f32, 0.2, projectile.time_to_live as f32, 1.0];  // light pink
                        } else {
                            canvas_line[index_column] = [projectile.time_to_live as f32, 0.4, projectile.time_to_live as f32, 1.0];  // lighter pink
                        }                
                        break;                            
                    }
                    if projectile.y >= ROOM_SIZE_Y { // back
                        if is_x_alternate {
                            canvas_line[index_column] = [0.0, 0.0, projectile.time_to_live as f32, 1.0]; // blue
                        } else if is_z_alternate {
                            canvas_line[index_column] = [0.2, 0.2, projectile.time_to_live as f32, 1.0]; // light blue    
                        } else {
                            canvas_line[index_column] = [0.4, 0.4, projectile.time_to_live as f32, 1.0]; // lighter blue    
                        }      
                        break;                            
                    }                    

                    projectile.x += projectile.dx;  //cos or sin
                    projectile.y += projectile.dy;
                    projectile.z += projectile.dz;

                    if projectile.time_to_live > 0.5 {
                        projectile.time_to_live -= 0.0004; // add fake shadow effect
                    }
                    
                    
                    /*if is_enemy_found {
                        continue;
                    }*/

                    for (ball_index, enemy) in self.enemies.iter().enumerate() {
                        if last_ball_bounce == ball_index {
                            continue;
                        }
                        let object_size = enemy.size;
                        let object_size_plus_error = object_size + 0.4;

                        // Manhattan distance filter
                        let dx = enemy.x - projectile.x;
                        let dy = enemy.y - projectile.y;
                        let dz = enemy.z - projectile.z;

                        if dx.abs() > object_size_plus_error || dy.abs() > object_size_plus_error || dz.abs() > object_size_plus_error {
                            continue;
                        }

                        // Compute expensive distance
                        let len_from_core = ((dx).powf(2.0) + (dy).powf(2.0) + (dz).powf(2.0)).sqrt();

                        if len_from_core + 0.5 >= object_size && len_from_core - 0.5 <= object_size {

                            last_ball_bounce = ball_index;

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
}