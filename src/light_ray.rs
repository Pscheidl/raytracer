use std::{collections::HashSet, rc::Rc};

use crate::{enemy::Enemy, player, projectile::{self, Projectile}, room::Room};


// 1.6-1.8 FPS before usage of RAY state
macro_rules! impl_ray_state {
    ($($state:ident),+) => {
        $(
        impl LightRayState for $state {
            fn audit(&self) -> String {
                format!("{self:?}")
            }
        }
        )+
    };
}

impl_ray_state!(
    FindingColor,
    IntermediateCheckForLightSource,
    ColorFoundSearchingForLightSource
);

pub trait LightRayState {
    /// Information to be displayed when printing the audit.
    fn audit(&self) -> String;
}

#[derive(Copy, Clone, Debug)]
pub struct FindingColor;

#[derive(Copy, Clone, Debug)]
pub struct IntermediateCheckForLightSource {
    projectile_before_collision: Projectile
}

#[derive(Copy, Clone, Debug)]
pub struct ColorFoundSearchingForLightSource {
    pub option_wall_collision_vec: Option<[f64;3]>,
    pub option_first_object_collision_vec: Option<[f64;3]>,
}


#[derive(Clone, Debug)]
pub struct LightRay<S: LightRayState> {
    pub projectile: Projectile,
    pub state: Rc<S>,
    pub buffer_wall_color: [f32; 4],
}

impl<S: LightRayState> LightRay<S>  {

    fn transition<N: LightRayState + 'static>(self, next: N) -> LightRay<N> {

        let next = Rc::new(next);

        LightRay {
            projectile: self.projectile,
            buffer_wall_color: self.buffer_wall_color,
            state: next,
        }
    }
}

impl LightRay<ColorFoundSearchingForLightSource> {
    pub fn skip_shadows(self) -> [f32; 4]{
        return self.buffer_wall_color;
    }

    pub fn compute_shadows(self, room: &Room, objects: &Vec<Enemy>) -> [f32; 4]{
        let mut shadow_color = self.buffer_wall_color;
        let mut wall_shadow_count = 0;
        let mut object_shadow_count = 0;
        // wall shadows
        match self.state.option_wall_collision_vec  {
            Some(wall_collision_vec) => {
                wall_shadow_count = Self::trace_ray_towards_light(wall_collision_vec,  1, room, objects); // only one shadow maximum is cast from all objects
            }
            None => print!("wall was not detected, error"),
        }
        for _x in 0..wall_shadow_count {
            shadow_color[0] -= 0.15;
            shadow_color[1] -= 0.15;
            shadow_color[2] -= 0.15;
        }
        
        // object shadows
        match self.state.option_first_object_collision_vec  {
            Some(first_object_collision_vec) => {
                object_shadow_count = Self::trace_ray_towards_light(first_object_collision_vec, 4, room, objects);
            }
            None => {},
        }
        for _x in 0..object_shadow_count {
            shadow_color[0] -= 0.2;
            shadow_color[1] -= 0.2;
            shadow_color[2] -= 0.2;
        }
        return shadow_color;  
    }

    fn trace_ray_towards_light(start_vec: [f64; 3], max_objects: usize, room: &Room, objects: &Vec<Enemy>) -> usize {
        let light_vec = [room.x / 2.0, 0.0, room.z / 2.0]; // hard-coded light source
        let light_to_projectile_dx = light_vec[0] - start_vec[0];
        let light_to_projectile_dy = light_vec[1] - start_vec[1];
        let light_to_projectile_dz = light_vec[2] - start_vec[2];
        let len_light_to_projectile = (light_to_projectile_dx.powf(2.0) + light_to_projectile_dy.powf(2.0) + light_to_projectile_dz.powf(2.0)).sqrt();

        let delta_x = light_to_projectile_dx / len_light_to_projectile;
        let delta_y = light_to_projectile_dy / len_light_to_projectile;
        let delta_z = light_to_projectile_dz / len_light_to_projectile;

        let mut projectile = Projectile::new(
            start_vec[0],
            start_vec[1], 
            start_vec[2], 
            delta_x,
            delta_y,
            delta_z,
            1.0);

        let mut objects_from_object_towards_light: HashSet<usize> = HashSet::new();

        // move the projectile from collision point
        projectile.increment();

        'outer: for _x in 1..1000000 {
            projectile.increment();

            if room.is_outside(&projectile) {
                break;
            }
            
            for (enemy_id, enemy) in objects.iter().enumerate() {

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
                    if !objects_from_object_towards_light.contains(&enemy_id) {
                        objects_from_object_towards_light.insert(enemy_id);
                        if objects_from_object_towards_light.len()  >= max_objects {
                            break 'outer;
                        }                    
                    }
                }
            }            
        }

        return objects_from_object_towards_light.len();
    }
}

impl LightRay<FindingColor> {
    pub fn new(projectile: Projectile) -> LightRay<FindingColor> {
        LightRay {
            projectile,
            state: Rc::new(FindingColor),
            buffer_wall_color: [0.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn find_wall_color(mut self, room: &Room, objects: &Vec<Enemy>) -> LightRay<ColorFoundSearchingForLightSource> {

        let mut option_first_object_collision_vec: Option<[f64;3]> = None;
        let mut option_wall_collision_vec: Option<[f64;3]> = None;
 
        'ray_travel: for _x in 1..100000 { // not using loop for debug in order to handle infinity errors
            
            // check walls
            if room.is_outside(&self.projectile) { // faster check for end of the room (+20 % FPS)
                if let Some(wall_color) = room.get_wall_color_at_projectile(&self.projectile) {
                
                    // add color from the wall
                    self.buffer_wall_color[0] += wall_color[0];
                    self.buffer_wall_color[1] += wall_color[1];
                    self.buffer_wall_color[2] += wall_color[2];
    
                    option_wall_collision_vec = Some([
                        self.projectile.x.clone(),
                        self.projectile.y.clone(),
                        self.projectile.z.clone()
                        ]);
    
                    break 'ray_travel // wall is the end of the room
                }
            }

            // check objects            
            for (enemy_id, enemy) in objects.iter().enumerate() {

                let object_size = enemy.size;
                let object_size_plus_error = object_size + 0.5;

                // Manhattan distance filter (+10 % FPS)
                let enemy_to_projectile_dx = enemy.x - self.projectile.x;
                let enemy_to_projectile_dy = enemy.y - self.projectile.y;
                let enemy_to_projectile_dz = enemy.z - self.projectile.z;

                if enemy_to_projectile_dx.abs() > object_size_plus_error || enemy_to_projectile_dy.abs() > object_size_plus_error || enemy_to_projectile_dz.abs() > object_size_plus_error {
                    continue;
                }
                // Compute expensive distance
                let len_projectile_to_core = ((enemy_to_projectile_dx).powf(2.0) + (enemy_to_projectile_dy).powf(2.0) + (enemy_to_projectile_dz).powf(2.0)).sqrt();

                if len_projectile_to_core + 0.5 >= object_size && len_projectile_to_core - 0.5 <= object_size {
                    // collision with an object when searching for a wall                
                    // while searching for a wall we can hit other objects
                        
                    let enemy_to_projectile_norm_x = enemy_to_projectile_dx / len_projectile_to_core;
                    let enemy_to_projectile_norm_y = enemy_to_projectile_dy / len_projectile_to_core;
                    let enemy_to_projectile_norm_z = enemy_to_projectile_dz / len_projectile_to_core;

                    // R=V−2N(V⋅N)
                    // R=RAY-2*NORMAL(RAY*NORMAL)
                    //                    ^-- dot product

                    let dot_x = self.projectile.dx + enemy_to_projectile_norm_x;
                    let dot_y = self.projectile.dy + enemy_to_projectile_norm_y;
                    let dot_z = self.projectile.dz + enemy_to_projectile_norm_z;
                    let dot_projectile_ball_norm = (dot_x.powf(2.0) + dot_y.powf(2.0) + dot_z.powf(2.0)).sqrt();
                    
                    let reflection_dx = self.projectile.dx - 2.0*enemy_to_projectile_norm_x*(dot_projectile_ball_norm);
                    let reflection_dy = self.projectile.dy - 2.0*enemy_to_projectile_norm_y*(dot_projectile_ball_norm);
                    let reflection_dz = self.projectile.dz - 2.0*enemy_to_projectile_norm_z*(dot_projectile_ball_norm);
                    let len_reflection_delta = (reflection_dx.powf(2.0) + reflection_dy.powf(2.0) + reflection_dz.powf(2.0)).sqrt();
                    
                    self.projectile.dx = reflection_dx / len_reflection_delta;
                    self.projectile.dy = reflection_dy / len_reflection_delta;
                    self.projectile.dz = reflection_dz / len_reflection_delta;

                    // save first object collision for optional shadow computation
                    if option_first_object_collision_vec.is_none() {
                        option_first_object_collision_vec = Some([
                            self.projectile.x,
                            self.projectile.y,
                            self.projectile.z,
                            ]);
                    }
                    
                    // avoid collision in next iteration
                    self.projectile.increment();
                }
            }
            self.projectile.increment();
        }
        self.transition(ColorFoundSearchingForLightSource { option_first_object_collision_vec, option_wall_collision_vec })
        
    }
}