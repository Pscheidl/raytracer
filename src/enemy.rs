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
    pub moving_x_speed: f64,
    pub moving_y_speed: f64,
    pub moving_z_speed: f64,
}

impl Enemy {
    pub fn new(
        x: f64,
        y: f64,
        z: f64,
        size: f64,
        time_to_live: usize,
        enemy_type: EnemyType,
        moving_x_speed: f64,
        moving_y_speed: f64,
        moving_z_speed: f64,
    ) -> Enemy {
        Enemy {
            x,
            y,
            z,
            size: size,
            time_to_live,
            enemy_type,
            moving_x_speed,
            moving_y_speed,
            moving_z_speed,
        }
    }

    pub fn move_enemy(&mut self, room_size_x: f64,room_size_y: f64,room_size_z: f64) {
        self.x += self.moving_x_speed;
        // move enemy on x axis
        if self.x + self.size  >= room_size_x {
            self.moving_x_speed = -5.0;
        } else if self.x - self.size  <= 0.0 {
            self.moving_x_speed = 5.0;
        }

        self.y += self.moving_y_speed;
        // move enemy on y axis
        if self.y + self.size  >= room_size_y {
            self.moving_y_speed = -5.0;
        } else if self.y - self.size  <= 0.0 {
            self.moving_y_speed = 5.0;
        }

        self.z += self.moving_z_speed;
        // move enemy on z axis
        if self.z + self.size  >= room_size_z {
            self.moving_z_speed = -5.0;
        } else if self.z - self.size  <= 0.0 {
            self.moving_z_speed = 5.0;
        }
    }
}