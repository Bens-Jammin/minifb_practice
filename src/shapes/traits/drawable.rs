
pub trait Drawable {

    /// given the pixel buffer for the window, this function fills the buffer 
    /// with the co lour of the given triange where the triangle exists      
    fn draw_onto_buffer( &self, buffer: &mut [u32], screen_width: usize );

    /// Since the draw_onto_buffer doesn't need to search all the possible points on the window, just the ones that might be on the triange,
    /// this function is to determine a 'window' that the triangle.
    /// 
    /// This function is also used to help determine the 'hit boxes' of the shapes, and allow overlap detection.
    fn get_bounding_coordinates(&self) -> ( (usize, usize), (usize, usize) );

}