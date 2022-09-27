use crate::{WINDOW_WIDTH, WINDOW_HEIGHT, projectile::Projectile};

const MAX_VELOCITY_X:f64 = 1.0;
const MAX_VELOCITY_Y:f64 = 1.0;
pub const WIDTH:usize= 20*4;
pub const HEIGHT:usize= 20*4;


pub struct Player {
    pub x: f64,
    pub y: f64,
    pub move_speed_left: f64,
    pub move_speed_up: f64,
    pub is_moving_up: bool,
    //pub is_moving_backward: bool,
    pub is_turning_left: bool,
    pub is_turning_right: bool,
    pub is_facing_left: bool,
    pub is_shooting: bool,
    pub projectiles: Vec<Projectile>,
    pub shooting_timer: usize,
}

impl Player {
    pub fn new(
        x: f64,
        y: f64,
        move_speed_left: f64,
        move_speed_up: f64,
        is_moving_up: bool,
        //is_moving_backward: bool,
        is_turning_left: bool,
        is_turning_right: bool,
        is_facing_left: bool,   
        is_shooting: bool, 
        projectiles: Vec<Projectile>,
        shooting_timer: usize,
    ) -> Player {
        Player {
            x,
            y,
            move_speed_left,
            move_speed_up,
            is_moving_up,
            //is_moving_backward,
            is_turning_left,
            is_turning_right,
            is_facing_left,
            is_shooting,
            projectiles,
            shooting_timer,
        }
    }
    pub fn tick(&mut self) {
        if self.is_shooting && self.shooting_timer == 0 {
            let mut speed = 0.0;
            if self.is_facing_left {
                speed = -10.0;
            } else {
                speed = 10.0;
            }
            let mut starting_x = WIDTH as f64 + self.x;
            if self.is_facing_left {
                starting_x = -(WIDTH as f64) / 2.0 + self.x;
            }
            self.projectiles.push(Projectile::new(starting_x, self.y + HEIGHT as f64 / 2.0, speed, 50));
            self.shooting_timer = 50;
        }
        if self.shooting_timer > 0 {
            self.shooting_timer -= 1;
        }
        

        if self.is_moving_up {
            self.move_up();
        } else {
            // moving down is done by fake gravity
            self.move_down();
        }

        if self.is_turning_left {
            self.is_facing_left = true;
            self.turn_left();
        } else if self.is_turning_right {
            self.is_facing_left = false;
            self.turn_right();
        }
        self.x += self.move_speed_left;
        self.y += self.move_speed_up;

        //collision y
        if self.y > (WINDOW_HEIGHT - HEIGHT*2) as f64 {
            self.y = (WINDOW_HEIGHT - HEIGHT*2) as f64;
            self.move_speed_up = 0.0;
            // friction on x
            self.move_speed_left /= 1.05;
        } else if self.y < 0.0 {
            self.y = 0.0;
            self.move_speed_up = 0.0;
            // friction on x
            self.move_speed_left /= 1.05;
        }

        //collision x
        if self.x > WINDOW_WIDTH as f64 {
            self.x = 0.0;
            /*self.x = 1000.0;
            self.move_speed_left = 0.0;*/
            // friction on x
            //self.move_speed_up /= 1.05;
        } else if self.x < 0.0 {
            self.x = (WINDOW_WIDTH - WIDTH) as f64;
            /*self.x = 50.0;
            self.move_speed_left = 0.0;
            // friction on x
            self.move_speed_up /= 1.05;*/
        }
    }

    pub fn move_up(&mut self) {
        //self.x += self.move_angle.cos() * self.move_speed;
        
        if self.move_speed_up >= -MAX_VELOCITY_Y {
            self.move_speed_up -= 0.01;
        }
    }

    pub fn move_down(&mut self) {
        //self.x -= self.move_angle.cos() * self.move_speed;
        if self.move_speed_up <= MAX_VELOCITY_Y {
            self.move_speed_up += 0.01;
        }    
    }

    pub fn turn_left(&mut self) {
        if self.move_speed_left >= -MAX_VELOCITY_X {
            self.move_speed_left -= 0.01;
        }
    }

    pub fn turn_right(&mut self) {
        if self.move_speed_left <= MAX_VELOCITY_X {
            self.move_speed_left += 0.01;
        }
    }
}
