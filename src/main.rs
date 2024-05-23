use minifb::{Key, Window, WindowOptions};
use shapes::{circle::Circle, point::Point, rectangle::Rectangle, traits::{drawable::Drawable, dynamic_shape::DynamicShape}, triangle::Triangle};

mod shapes;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const BACKGROUND_COLOUR: u32 = 0xFFFFFFFF;
const FRAMES_PER_SECOND: usize = 60;
const TIME_STEP: f64 = 15.0 / FRAMES_PER_SECOND as f64;  

fn main() {
    let mut buffer = vec![0u32; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Sample Title (press ESC to close)", 
        WIDTH, 
        HEIGHT, 
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        }
    ).expect("Unable to open the window");
    
    window.set_target_fps( FRAMES_PER_SECOND );


    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(
            Triangle::new(
                Point { x:10, y:10},
                Point { x:200, y:540},
                Point { x:1000, y:600},
                0x0000F0BB
            )
        ),
        Box::new(
            Rectangle::new(
                Point { x: 0, y: 450 },
                WIDTH,
                HEIGHT - 450,
                0x0000AA88
            )   
        )
    ];
        
    let mut moving_shapes: Vec<Box<dyn DynamicShape>> = vec![
        Box::new(
            Circle::new(
                Point { x:900, y: 100},
                75,
                0xFFF15A22
            )
        )
    ];

    while window.is_open() && !window.is_key_down(Key::Escape) {


        // gets the users mouse clicks and spawns a circle there 
        if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
            if window.is_key_pressed(Key::C, minifb::KeyRepeat::No) { // c for circle
                moving_shapes.push(Box::new(
                    Circle::new(
                        Point { x: x as usize, y: y as usize },
                        30,
                        0xFF00FF
                    )
                ));
            }
        }



        // set background colour bc built in one doesnt work
        for pixel in buffer.iter_mut() {
            *pixel = BACKGROUND_COLOUR
        }

        // draw all the shapes
        for shape in &shapes {
            shape.draw_onto_buffer(&mut buffer, WIDTH);
        }

        // Move and draw the movable shapes
        for shape in &mut moving_shapes {
            shape.update(TIME_STEP, WIDTH, HEIGHT);
            shape.draw_onto_buffer(&mut buffer, WIDTH);
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}