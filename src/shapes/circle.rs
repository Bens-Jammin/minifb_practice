use crate::shapes::traits::drawable::Drawable;
use super::{point::Point, traits::movable::Movable};
use libm;

const ELASTICITY_COEFFICIENT: f64 = 0.75;
const GRAVITY: f64 = 10.0;


pub struct Circle {
    radius: usize,
    center: Point,
    colour: u32,
    x_velocity: f64,
    y_velocity: f64,
    mass: usize,
}


impl Circle {
    pub fn new(center: Point, radius: usize, colour: u32) -> Self {
        let set_mass = 100;
        Circle { 
            center, 
            radius, 
            colour, 
            x_velocity: 0.0, 
            y_velocity: 9.8,   // increase in y moves the ball down, axis is inverted 
            mass: set_mass,
        }
    }


    pub fn is_colliding_with(&self, other: Circle) -> bool {

        let x_component = self.center.x as f64 - other.center.x as f64;
        let y_component = self.center.y as f64 - other.center.y as f64;

        let d = y_component.powf(2.0) + x_component.powf(2.0);

        d < (self.radius + other.radius).pow(2) as f64
    }


    /// finds the angle between any two circles. Returns a value in radians between -π/2 and π/2
    /// 
    /// NOTE: implement with conservation of momentum: https://www.youtube.com/watch?v=9YRgHikdcqs
    pub fn find_angle_between(&self, other: Circle) -> f64 {

        // adjacent
        let x_component = self.center.x as f64 - other.center.x as f64;
        
        // opposite
        let y_component = self.center.y as f64 - other.center.y as f64;

        // NOTE: angle is in radians 
        let angle = libm::atan( y_component / x_component );

        angle
    }
}


impl Drawable for Circle {
    
    fn draw_onto_buffer( &self, buffer: &mut [u32], screen_width: usize ) {
        // to calculate if the pixel is inside the circle, we use pythagoreans theorem to find if
        // the shortest path from the center to the point is less than or equal to the radius.
        // if yes, then the point is inside the circle.

        let ( (min_x, min_y), (max_x, max_y) ) = self.get_bounding_coordinates();

        for y in min_y..max_y {
            for x in min_x..max_x {
                
                let dx = self.center.x as isize - x as isize;
                let dy = self.center.y as isize - y as isize;
                
                // min distance from point to center 
                // calculated using pythagorean theorem
                let dr = f32::sqrt( (dx.pow(2) + dy.pow(2)) as f32 );

                if dr as usize <= self.radius {
                    buffer[y * screen_width + x ] = self.colour;
                }
            }
        }

    }
    
    fn get_bounding_coordinates(&self) -> ( (usize, usize), (usize, usize) ) {
        ( 
            ( self.center.x - self.radius, self.center.y - self.radius) , 
            ( self.center.x + self.radius, self.center.y + self.radius) 
        )
    }
}

const VELOCITY_THRESHOLD: f64 = 0.1;

impl Movable for Circle {
    fn update(&mut self, time_step: f64, screen_width: usize, screen_height: usize) {
        
        // determine new velocities
        // yes x velocity isn't implemented yet i know
        self.y_velocity += GRAVITY * time_step * 0.95; 

        // Calculate the new positions
        let mut dy: isize = (self.y_velocity * time_step) as isize;
        let mut dx: isize = (self.x_velocity * time_step) as isize;

        let ((min_x, min_y), (max_x, max_y)) = self.get_bounding_coordinates();

        // Handle horizontal collision
        if min_x as isize - dx < 0 || max_x + dx.abs() as usize >= screen_width {
            self.x_velocity = -self.x_velocity * ELASTICITY_COEFFICIENT;
            dx = -dx;
        }

        // Handle vertical collision
        if min_y as isize - dy < 0 || max_y + dy.abs() as usize >= screen_height {
            self.y_velocity = -self.y_velocity * ELASTICITY_COEFFICIENT;
            dy = -dy; 
        }

        
        // man i just need the ball to stop bouncing
        if self.x_velocity.abs() < VELOCITY_THRESHOLD { self.x_velocity = 0.0; }
        if self.y_velocity.abs() < VELOCITY_THRESHOLD { self.y_velocity = 0.0;}


        self.center.update_position( dy , dx );
    }
}
