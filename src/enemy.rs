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
    pub moving_left_speed: f64,
}

impl Enemy {
    pub fn new(
        x: f64,
        y: f64,
        z: f64,
        size: f64,
        time_to_live: usize,
        enemy_type: EnemyType,
        moving_left_speed: f64,
    ) -> Enemy {
        Enemy {
            x,
            y,
            z,
            size: size,
            time_to_live,
            enemy_type,
            moving_left_speed,
        }
    }

    pub fn move_enemy(&mut self, room_size: f64) {
        self.x += self.moving_left_speed;
        // move enemy
        if self.x + self.size  >= room_size {
            self.moving_left_speed = -5.0;
        } else if self.x - self.size  <= 0.0 {
            self.moving_left_speed = 5.0;
        }
    }
}