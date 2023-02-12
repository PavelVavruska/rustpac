use crate::{WINDOW_WIDTH, WINDOW_HEIGHT, projectile::Projectile, map};

const MAX_VELOCITY_X:f64 = 1.0;
const MAX_VELOCITY_Y:f64 = 1.0;
pub const WIDTH:f64 = 76.0/2.0;
pub const HEIGHT:f64 = 150.0/2.0;

pub struct Movement {
    pub move_speed_left: f64,
    pub move_speed_up: f64,
    pub is_moving_up: bool,
    //pub is_moving_backward: bool,
    pub is_turning_left: bool,
    pub is_turning_right: bool,
    pub is_facing_left: bool,
}

impl Movement {
    pub fn new(
        move_speed_left: f64,
        move_speed_up: f64,
        is_moving_up: bool,
        //is_moving_backward: bool,
        is_turning_left: bool,
        is_turning_right: bool,
        is_facing_left: bool,
    ) -> Movement {
        Movement { move_speed_left,
            move_speed_up,
            is_moving_up,
            //is_moving_backward,
            is_turning_left,
            is_turning_right,
            is_facing_left, }

    }
}

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub movement: Movement,
    pub is_shooting: bool,
    pub projectiles: Vec<Projectile>,
    pub shooting_timer: usize,
}

impl Player {
    pub fn new(
        x: f64,
        y: f64,
        movement: Movement,        
        is_shooting: bool, 
        projectiles: Vec<Projectile>,
        shooting_timer: usize,
    ) -> Player {
        Player {
            x,
            y,
            movement,
            is_shooting,
            projectiles,
            shooting_timer,
        }
    }
    pub fn tick(&mut self, map: &Vec<map::Ground>) {
        if self.is_shooting && self.shooting_timer == 0 {
            let speed = if self.movement.is_facing_left {
                -10.0
            } else {
                10.0
            };
            let mut starting_x = WIDTH + self.x;
            if self.movement.is_facing_left {
                starting_x = -WIDTH / 2.0 + self.x;
            }
            self.projectiles.push(Projectile::new(starting_x, self.y + HEIGHT / 2.0, speed, 150));
            self.shooting_timer = 25;
        }
        if self.shooting_timer > 0 {
            self.shooting_timer -= 1;
        }
        
        let is_colliding_x;
        let is_colliding_y;

        if self.movement.is_moving_up {
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

        if self.movement.is_turning_left {
            self.movement.is_facing_left = true;
            self.move_left();
            is_colliding_x = self.is_colliding(map);
            if is_colliding_x {
                self.move_right();
            }
        } else if self.movement.is_turning_right {
            self.movement.is_facing_left = false;
            self.move_right();
            is_colliding_x = self.is_colliding(map);
            if is_colliding_x {
                self.move_left();
            }
        }
        
        // collision 
        self.x += self.movement.move_speed_left;
        if self.is_colliding(map) {
            self.x -= self.movement.move_speed_left;
            self.movement.move_speed_left = 0.0;
        }
        self.y += self.movement.move_speed_up;
        if self.is_colliding(map) {            
            self.y -= self.movement.move_speed_up;            
            self.movement.move_speed_up = 0.0;
        }

        //collision y
        if self.y > WINDOW_HEIGHT as f64 - HEIGHT*2.0 {
            self.y = WINDOW_HEIGHT as f64 - HEIGHT*2.0;
            self.movement.move_speed_up = 0.0;
            // friction on x
            self.movement.move_speed_left /= 1.05;
        } else if self.y < 0.0 {
            self.y = 0.0;
            self.movement.move_speed_up = 0.0;
            // friction on x
            self.movement.move_speed_left /= 1.05;
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
        
        if self.movement.move_speed_up >= -MAX_VELOCITY_Y {
            self.movement.move_speed_up -= 0.01;
        }
    }

    pub fn move_down(&mut self) {
        //self.x -= self.move_angle.cos() * self.move_speed;
        if self.movement.move_speed_up <= MAX_VELOCITY_Y {
            self.movement.move_speed_up += 0.01;
        }    
    }

    pub fn move_left(&mut self) {
        if self.movement.move_speed_left >= -MAX_VELOCITY_X {
            self.movement.move_speed_left -= 0.01;
        }
    }

    pub fn move_right(&mut self) {
        if self.movement.move_speed_left <= MAX_VELOCITY_X {
            self.movement.move_speed_left += 0.01;
        }
    }

    pub fn is_colliding(&self, map: &Vec<map::Ground>) -> bool {
        let player_x = self.x;
        let player_y = self.y;
        for map_object in map {
            if player_x + WIDTH > map_object.x && player_x - map_object.width < map_object.x 
            && player_y + HEIGHT > map_object.y && player_y - map_object.height < map_object.y {
                return true;
            }
        }
        false
    }
}


#[cfg(test)]
mod tests {
    use crate::{map::Ground, player::Movement};
    use super::Player;

    #[test]
    fn test_colision_x_before_map() {
        let player = Player::new(0.0,
            0.0, 
            Movement::new(1.0,
                0.0,
                false,
                false,
                true,             
                false),            
            false,
            Vec::new(),
            1);
        
        let mut map_item_list = Vec::new();
        let ground1 = Ground::new(1.0 + super::WIDTH, 0.0, 2.0, 10.0);
        map_item_list.push(ground1);
        
        assert_eq!(0.0, player.x);
        assert_eq!(false, player.is_colliding(&map_item_list));
    }

    #[test]
    fn test_colision_x_inside_map() {
        let player = Player::new(2.0,
            0.0, 
            Movement::new(1.0,
                0.0,
                false,
                false,
                true,             
                false),            
            false,
            Vec::new(),
            1);

        let mut map_item_list = Vec::new();
        let ground1 = Ground::new(1.0 + super::WIDTH, 0.0, 2.0, 10.0);
        map_item_list.push(ground1);

        assert_eq!(2.0, player.x);
        assert_eq!(true, player.is_colliding(&map_item_list));
    }

    #[test]
    fn test_colision_x_after_map() {        
        let player = Player::new(super::WIDTH + 2.0,
            0.0, 
            Movement::new(1.0,
            0.0,
            false,
            false,
            true,             
            false),
            false,
            Vec::new(),
            1);

        let mut map_item_list = Vec::new();
        let ground1 = Ground::new(1.0 + super::WIDTH, 0.0, 2.0, 10.0);
        map_item_list.push(ground1);
        
        assert_eq!(super::WIDTH + 2.0, player.x);
        assert_eq!(true, player.is_colliding(&map_item_list));
    }
}