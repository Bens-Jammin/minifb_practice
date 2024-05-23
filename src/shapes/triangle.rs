use std::isize;

use crate::shapes::traits::drawable::Drawable;

use super::point::Point;


pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
    colour: u32
}

// TODO: do colour gradients for triangles
// https://www.youtube.com/watch?v=t7Ztio8cwqM

impl Triangle {

    pub fn new( p1: Point, p2: Point, p3: Point, colour: u32 ) -> Self {
        Self {
            p1,
            p2,
            p3,
            colour
        }
    }


    /// the point given can be represented (relative to some point in the triangle 'A')
    /// as the sum of two vectors. We can then determine if the point is inside the triangle iff:
    /// 
    /// * w1 >= 0
    /// 
    /// * w2 >= 0
    /// 
    /// * (w1 + w2) <= 1
    /// 
    /// ### Further Explanation
    /// 
    /// https://www.youtube.com/watch?v=HYAgJN3x4GA
    fn calculate_vector_weights(&self, x: isize, y: isize) -> (f64, f64) {

        let (ax, ay) = (self.p1.x as isize, self.p1.y as isize);
        let (bx, by) = (self.p2.x as isize, self.p2.y as isize);
        let (cx, cy) = (self.p3.x as isize, self.p3.y as isize);

        let w1: f64 = ( ax*(cy-ay) + (y-ay)*(cx-ax) - x*(cy-ay) ) as f64 
                    / ( (by-ay)*(cx-ax) -(bx-ax)*(cy-ay) ) as f64; 

        let w2: f64 = ( (y as f64) - (ay as f64) - w1 * (by-ay) as f64  ) as f64
                    / ( cy - ay ) as f64;
    
        (w1, w2)
    }
    
}


impl Drawable for Triangle {
    
    fn draw_onto_buffer(&self, buffer: &mut [u32], screen_width: usize) {

        let ( (min_x, min_y), (max_x, max_y) ) = self.get_bounding_coordinates();

        for y in min_y..max_y {
            for x in min_x..max_x {
                let (w1, w2) = self.calculate_vector_weights( x as isize, y as isize );

                let pixel_is_inside_triangle =  ( w1 >= 0.0 ) 
                && ( w2 >= 0.0 ) 
                && ( w1 + w2 <= 1.0 ); 
                if pixel_is_inside_triangle {
                    buffer[y * screen_width + x ] = self.colour;
                }
            }
        }
        
    }
    
    fn get_bounding_coordinates(&self) -> ( (usize, usize), (usize, usize) ) {
    
         let mut x_points = vec![ self.p1.x, self.p2.x, self.p3.x ];
         let mut y_points = vec![self.p1.y, self.p2.y, self.p3.y ];
         
         x_points.sort();
         y_points.sort();
    
         ( (x_points[0], y_points[0]), (x_points[2], y_points[2]) )
     }
    
}