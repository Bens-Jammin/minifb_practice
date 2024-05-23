
pub trait Movable {
    fn update( &mut self, time_step: f64, screen_width: usize, screen_height: usize );

}