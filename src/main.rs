// main file for R-iero

mod world; // loads world.rs as module

use minifb::{Key, MouseButton, MouseMode, Scale, Window, WindowOptions};
use world::{Material, World};

const WIDTH: usize = 320;  // world width in cells
const HEIGHT: usize = 200; // world height i cells
const BRUSH: i32 = 3;      // radius of the sand brush in cells

fn main(){
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    // world simulation
    let mut world = World::new(WIDTH, HEIGHT);
    let mut window = Window::new(
        "R-iero",
	WIDTH,
	HEIGHT,
	WindowOptions{
	    scale: Scale::X4,
	    ..WindowOptions::default()
	},
    )
    .expect("Could not create window");
    
    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape){
        // input: pour sand (using lmb)
	if window.get_mouse_down(MouseButton::Left){
	    // mouse pos in world cells
	    // (none if outside of window range)
	    if let Some((mx, my)) = window.get_mouse_pos(MouseMode::Discard){
	        let cx = mx as i32;
		let cy = my as i32;
                for dy in -BRUSH..=BRUSH{
	            for dx in -BRUSH..=BRUSH{
		        let x = cx + dx;
		        let y = cy + dy;
		        // pythagoras
		        let inside_circle = dx * dx + dy * dy <= BRUSH * BRUSH;
		        // only ~1 in 4 of them are pouring
		        let sprinkle = fastrand::u8(0..4) == 0;

		        if inside_circle && sprinkle && world.is_empty(x, y){
		            world.set(x, y, Material::Sand);
		        }
		    }
	        }  
	    }
	}
     
	world.update();

	// drawing world -> buffer -> window
	world.draw(&mut buffer);
	window
	    .update_with_buffer(&buffer, WIDTH, HEIGHT)
	    .expect("Could not update window");
    }
}
