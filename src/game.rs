use std::collections::HashSet;

use crate::player;

use crate::enemy;
use crate::projectile::Projectile;
use crate::room;

use piston_window::types::Color;
use rayon::prelude::*;

#[derive(PartialEq, Debug)]
enum LightTracing {
    FindingWall,
    WallFoundSearchingForLightSource,
    IntermediateSearchingForLightSource,
}


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

                let mut light_tracer = LightTracing::FindingWall;         
                let mut buffer_wall_color = [
                    0.0, 
                    0.0, 
                    0.0, 
                    1.0];
                let mut intermediate_projectile = Projectile::new(0.0,0.0,0.0,0.0,0.0,0.0,1.0);  // X Y Z
                let mut objects_from_wall_towards_light: HashSet<usize> = HashSet::new();
                let mut objects_from_object_towards_light: HashSet<usize> = HashSet::new();
                
                'ray_travel: for _x in 1..100000 { // not using loop for debug in order to handle infinity errors

                    match light_tracer {
                        LightTracing::FindingWall => {

                            // finding wall, if found start moving towards light source                            
                            if let Some(wall_color) = self.room.get_wall_color_at_projectile(&projectile) {
                                buffer_wall_color[0] += wall_color[0];
                                buffer_wall_color[1] += wall_color[1];
                                buffer_wall_color[2] += wall_color[2];
                                let light_source_to_projectile_dx= self.room.x / 2.0 - projectile.x;
                                let light_source_to_projectile_dy = 0.0 - projectile.y;
                                let light_source_to_projectile_dz = self.room.z / 2.0 - projectile.z;
                                let len_light_source_to_projectile = (light_source_to_projectile_dx.powf(2.0) + light_source_to_projectile_dy.powf(2.0) + light_source_to_projectile_dz.powf(2.0)).sqrt();

                                projectile.dx = light_source_to_projectile_dx / len_light_source_to_projectile;
                                projectile.dy = light_source_to_projectile_dy / len_light_source_to_projectile;
                                projectile.dz = light_source_to_projectile_dz / len_light_source_to_projectile;
                                light_tracer = LightTracing::WallFoundSearchingForLightSource;
                            }
                        }
                        LightTracing::WallFoundSearchingForLightSource => {

                            // check if the projectile arrived at the light source, if yes write current color and exit current ray processing

                            if self.room.is_projectile_near_light(&projectile) {
                                canvas_line[index_column] = buffer_wall_color;
                                break 'ray_travel // is light
                            }
                        }
                        LightTracing::IntermediateSearchingForLightSource => {

                            // check if the projectile arrived at the light source, if yes continue moving ray towards wall (to get color)
                            if self.room.is_projectile_near_light(&projectile) {
                                // we found a light source - keep the brightness as it is
                                
                                // reset the projectile back to the reflection point of the object
                                projectile.x = intermediate_projectile.x;
                                projectile.y = intermediate_projectile.y;
                                projectile.z = intermediate_projectile.z;
                                projectile.dx = intermediate_projectile.dx;
                                projectile.dy = intermediate_projectile.dy;
                                projectile.dz = intermediate_projectile.dz;

                                light_tracer = LightTracing::FindingWall;
                            }                            
                        },
                    }

                    for (enemy_id, enemy) in self.enemies.iter().enumerate() {

                        let object_size = enemy.size;
                        let object_size_plus_error = object_size + 0.5;

                        // Manhattan distance filter (+10 % FPS)
                        let enemy_to_projectile_dx = enemy.x - projectile.x;
                        let enemy_to_projectile_dy = enemy.y - projectile.y;
                        let enemy_to_projectile_dz = enemy.z - projectile.z;

                        if enemy_to_projectile_dx.abs() > object_size_plus_error || enemy_to_projectile_dy.abs() > object_size_plus_error || enemy_to_projectile_dz.abs() > object_size_plus_error {
                            continue;
                        }

                        // Compute expensive distance
                        let len_projectile_to_core = ((enemy_to_projectile_dx).powf(2.0) + (enemy_to_projectile_dy).powf(2.0) + (enemy_to_projectile_dz).powf(2.0)).sqrt();
                        

                        if len_projectile_to_core + 0.5 >= object_size && len_projectile_to_core - 0.5 <= object_size {
                            // collision with an object when searching for a wall

                            match light_tracer {
                                LightTracing::FindingWall => {
                                    // while searching for a wall we can hit other objects
                        
                                    let enemy_to_projectile_norm_x = enemy_to_projectile_dx / len_projectile_to_core;
                                    let enemy_to_projectile_norm_y = enemy_to_projectile_dy / len_projectile_to_core;
                                    let enemy_to_projectile_norm_z = enemy_to_projectile_dz / len_projectile_to_core;
            
                                    // R=V−2N(V⋅N)
                                    // R=RAY-2*NORMAL(RAY*NORMAL)
                                    //                    ^-- dot product
        
                                    let dot_x = projectile.dx + enemy_to_projectile_norm_x;
                                    let dot_y = projectile.dy + enemy_to_projectile_norm_y;
                                    let dot_z = projectile.dz + enemy_to_projectile_norm_z;
                                    let dot_projectile_ball_norm = (dot_x.powf(2.0) + dot_y.powf(2.0) + dot_z.powf(2.0)).sqrt();
                                    
                                    let reflection_dx = projectile.dx - 2.0*enemy_to_projectile_norm_x*(dot_projectile_ball_norm);
                                    let reflection_dy = projectile.dy - 2.0*enemy_to_projectile_norm_y*(dot_projectile_ball_norm);
                                    let reflection_dz = projectile.dz - 2.0*enemy_to_projectile_norm_z*(dot_projectile_ball_norm);
                                    let len_reflection_delta = (reflection_dx.powf(2.0) + reflection_dy.powf(2.0) + reflection_dz.powf(2.0)).sqrt();
                                    
                                    projectile.dx = reflection_dx / len_reflection_delta;
                                    projectile.dy = reflection_dy / len_reflection_delta;
                                    projectile.dz = reflection_dz / len_reflection_delta;

                                    // save last projectile state (Add delta to skip collision in the next iteration)
                                    intermediate_projectile.x = projectile.x + projectile.dx;
                                    intermediate_projectile.y = projectile.y + projectile.dy;
                                    intermediate_projectile.z = projectile.z + projectile.dz;
                                    intermediate_projectile.dx = projectile.dx;
                                    intermediate_projectile.dy = projectile.dy;
                                    intermediate_projectile.dz = projectile.dz;

                                    // start moving towards light
                                    let light_to_projectile_dx = self.room.x / 2.0 - projectile.x;
                                    let light_to_projectile_dy = 0.0 - projectile.y;
                                    let light_to_projectile_dz = self.room.z / 2.0 - projectile.z;
                                    let len_light_to_projectile = (light_to_projectile_dx.powf(2.0) + light_to_projectile_dy.powf(2.0) + light_to_projectile_dz.powf(2.0)).sqrt();
        
                                    projectile.dx = light_to_projectile_dx / len_light_to_projectile;
                                    projectile.dy = light_to_projectile_dy / len_light_to_projectile;
                                    projectile.dz = light_to_projectile_dz / len_light_to_projectile;

                                    light_tracer = LightTracing::IntermediateSearchingForLightSource;
                                },
                            
                                LightTracing::WallFoundSearchingForLightSource => {

                                    // we hit an object when searching for a light source                                    
                                    if !objects_from_wall_towards_light.contains(&enemy_id) {
                                        objects_from_wall_towards_light.insert(enemy_id);
                                        buffer_wall_color[0] -= 0.2;
                                        buffer_wall_color[1] -= 0.2;
                                        buffer_wall_color[2] -= 0.2;
                                    }                                    
                                    //break 'ray_travel // is shadow
                                }
                                LightTracing::IntermediateSearchingForLightSource => {

                                    // we hit an object during search for a light source = shadow
                                    if !objects_from_object_towards_light.contains(&enemy_id) {
                                        objects_from_object_towards_light.insert(enemy_id);
                                        buffer_wall_color[0] -= 0.2;
                                        buffer_wall_color[1] -= 0.2;
                                        buffer_wall_color[2] -= 0.2;
                                    }
                                },
                            }
                        }
                    }
                    // at the end - move the projectile
                    projectile.x = projectile.x + projectile.dx;
                    projectile.y = projectile.y + projectile.dy;
                    projectile.z = projectile.z + projectile.dz;
                }
            }
            canvas_line
        }).collect();
        canvas_vec
    }
}