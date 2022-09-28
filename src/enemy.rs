use crate::WINDOW_WIDTH;

pub const WIDTH: usize = 100;
pub const HEIGHT: usize = 150;

#[derive(Copy, Clone)]
pub struct Enemy {
    pub x: f64,
    pub y: f64,
    pub speed_x: f64,
    pub time_to_live: usize,
    pub enemy_type: usize,
}

impl Enemy {
    pub fn new(
        x: f64,
        y: f64,
        speed_x: f64,
        time_to_live: usize,
        enemy_type: usize,
    ) -> Enemy {
        Enemy {
            x,
            y,
            speed_x,
            time_to_live,
            enemy_type,     
        }
    }
    pub fn tick(&mut self) {
        self.x += self.speed_x;

        //collision x
        if self.x > WINDOW_WIDTH as f64 {
            self.x = 0.0;            
        } else if self.x < 0.0 {
            self.x = (WINDOW_WIDTH - WIDTH) as f64;            
        }
    }
}