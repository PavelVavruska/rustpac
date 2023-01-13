use crate::{WINDOW_WIDTH, WINDOW_HEIGHT, projectile::Projectile, map};

const MAX_VELOCITY_X:f64 = 1.0;
const MAX_VELOCITY_Y:f64 = 1.0;
pub const WIDTH:f64= 75.0;
pub const HEIGHT:f64= 150.0;


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
    pub fn tick(&mut self, map: &Vec<map::Ground>) {
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
            self.projectiles.push(Projectile::new(starting_x, self.y + HEIGHT as f64 / 2.0, speed, 150));
            self.shooting_timer = 25;
        }
        if self.shooting_timer > 0 {
            self.shooting_timer -= 1;
        }
        
        let mut is_colliding_x = false;
        let mut is_colliding_y = false;

        if self.is_moving_up {
            self.move_up();
            is_colliding_y = self.is_colliding(map);
            if is_colliding_y {
                self.move_down();
            }
        } else {
            // moving down is done by fake gravity
            self.move_down();
            is_colliding_y = self.is_colliding(map);
            if is_colliding_y {
                self.move_up();
            }
        }

        if self.is_turning_left {
            self.is_facing_left = true;
            self.move_left();
            is_colliding_x = self.is_colliding(map);
            if is_colliding_x {
                self.move_right();
            }
        } else if self.is_turning_right {
            self.is_facing_left = false;
            self.move_right();
            is_colliding_x = self.is_colliding(map);
            if is_colliding_x {
                self.move_left();
            }
        }
        
        self.x += self.move_speed_left;
        if self.is_colliding(map) {
            self.x -= self.move_speed_left;
            self.move_speed_left = 0.0;
        }
        self.y += self.move_speed_up;
        if self.is_colliding(map) {            
            self.y -= self.move_speed_up;            
            self.move_speed_up = 0.0;
        }

        //collision y
        if self.y > WINDOW_HEIGHT as f64 - HEIGHT*2.0 {
            self.y = WINDOW_HEIGHT as f64 - HEIGHT*2.0;
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
            self.x = WINDOW_WIDTH as f64 - WIDTH;
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

    pub fn move_left(&mut self) {
        if self.move_speed_left >= -MAX_VELOCITY_X {
            self.move_speed_left -= 0.01;
        }
    }

    pub fn move_right(&mut self) {
        if self.move_speed_left <= MAX_VELOCITY_X {
            self.move_speed_left += 0.01;
        }
    }

    pub fn is_colliding(&self, map: &Vec<map::Ground>) -> bool {
        let player_low_left_x = self.x + WIDTH;
        let player_low_left_y = self.y + HEIGHT;
        for map_object in map {
            if player_low_left_x > map_object.x && player_low_left_x - WIDTH - map_object.width < map_object.x 
            && player_low_left_y > map_object.y && player_low_left_y - HEIGHT - map_object.height < map_object.y {
                print!("COLLISION");
                return true;
            }
        }
        false
    }
}
