// main file for R-iero

mod world; // loads world.rs as module

use minifb::{Key, MouseButton, MouseMode, Scale, Window, WindowOptions};
use world::{Material, World};

const WIDTH: usize = 320;  // world width in cells
const HEIGHT: usize = 200; // world height i cells
const BRUSH: i32 = 3;      // radius of the sand brush in cells

fn pour(world: &mut World, cx: i32, cy: i32, material: Material){
    for dy in -BRUSH..=BRUSH{
        for dx in -BRUSH..=BRUSH{
	    let x = cx + dx;
	    let y = cy + dy;
	    let inside_circle = dx * dx + dy * dy <= BRUSH * BRUSH;
	    let sprinkle = fastrand::u8(0..4) == 0;

	    if inside_circle && sprinkle && world.is_empty(x, y){
	        world.set(x, y, material);
	    }
	}
    }
}

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
	// lmb = sand, rmb = water
	if let Some((mx, my)) = window.get_mouse_pos(MouseMode::Discard){
	    let cx = mx as i32;
	    let cy = my as i32;

	    if window.get_mouse_down(MouseButton::Left){
	        pour(&mut world, cx, cy, Material::Sand);
	    }
	    if window.get_mouse_down(MouseButton::Right){
	        pour(&mut world, cx, cy, Material::Water);
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
