use super::{point::Point, traits::drawable::Drawable};


pub struct Rectangle {
    top_left_corner: Point,
    width: usize,
    height: usize,
    colour: u32
}

impl Rectangle {
    pub fn new(top_left_corner: Point , width: usize, height: usize, colour: u32) -> Self {
        Rectangle { top_left_corner, width, height, colour }
    }
}


impl Drawable for Rectangle {
    fn draw_onto_buffer( &self, buffer: &mut [u32], screen_width: usize ) {

        let ((min_x, min_y),(max_x, max_y)) = self.get_bounding_coordinates();

        for y in min_y..max_y {
            for x in min_x..max_x {
                buffer[y * screen_width + x ] = self.colour;
            }
        } 
        
    }

    fn get_bounding_coordinates(&self) -> ( (usize, usize), (usize, usize) ) {
        ( 
            ( self.top_left_corner.x, self.top_left_corner.y ),
            ( self.top_left_corner.x + self.width, self.top_left_corner.y + self.height )
        )
    }
}