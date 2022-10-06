use crate::WINDOW_WIDTH;

pub const WIDTH: usize = 150;
pub const HEIGHT: usize = 150;


#[derive(Copy, Clone)]
pub enum EnemyType {
    Point,
    Cube,
    Sphere,
}

#[derive(Copy, Clone)]
pub struct Enemy {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub size: f64,
    pub time_to_live: usize,
    pub enemy_type: EnemyType,
}

impl Enemy {
    pub fn new(
        x: f64,
        y: f64,
        z: f64,
        size: f64,
        time_to_live: usize,
        enemy_type: EnemyType,
    ) -> Enemy {
        Enemy {
            x,
            y,
            z,
            size: size,
            time_to_live,
            enemy_type,     
        }
    }
    pub fn tick(&mut self) {
        self.x += self.size;

        //collision x
        if self.x > WINDOW_WIDTH as f64 {
            self.x = 0.0;            
        } else if self.x < 0.0 {
            self.x = (WINDOW_WIDTH - WIDTH) as f64;            
        }
    }
}