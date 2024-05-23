pub struct Point {
    pub x: usize,
    pub y: usize
}

impl Point {
    pub fn update_position(&mut self, dy: isize, dx: isize) -> &mut Self {
        self.y = (self.y as isize + dy) as usize;
        self.x = (self.x as isize + dx) as usize;
        self
    }
}