// main file for R-iero

mod world; // loads world.rs as module

use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Scale, Window, WindowOptions};
use world::{Material, World};

const WIDTH: usize = 320;  // world width in cells
const HEIGHT: usize = 200; // world height i cells

fn pour(world: &mut World, cx: i32, cy: i32, radius: i32, material: Material){
    for dy in -radius..=radius{
        for dx in -radius..=radius{
	    if dx * dx + dy * dy > radius * radius{
	        continue;
	    }
	    let x = cx + dx;
	    let y = cy + dy;

	    if material == Material::Air{
	        world.set(x, y, Material::Air);
	    }
	    else if world.is_empty(x, y){
                if !material.moves() || fastrand::u8(0..4) == 0{
                   world.set(x, y, material);
		}
	    }
	}
    }
}

// show the current material and brush size in window title
fn update_title(window: &mut Window, selected: Material, brush: i32){
    let title = format!(
         "R-iero | {} | brush {} | 1-3: material, [ ]: size, RMB: erase",
	 selected.name(),
	 brush
    );
    window.set_title(&title);
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

    let mut selected = Material::Sand;
    let mut brush: i32 = 3;

    update_title(&mut window, selected, brush);

    while window.is_open() && !window.is_key_down(Key::Escape){
	// keyboard
	let mut changed = false;
	let hotkeys = [
	    (Key::Key1, Material::Sand),
	    (Key::Key2, Material::Water),
	    (Key::Key3, Material::Dirt),
	];

	for(key, material) in hotkeys{
            if window.is_key_pressed(key, KeyRepeat::No){
	        selected = material;
		changed = true;
	    }
	}

	// change brush size
	if window.is_key_pressed(Key::RightBracket, KeyRepeat::Yes){
            brush = (brush + 1).min(20);
	    changed = true;
	}
	if window.is_key_pressed(Key::LeftBracket, KeyRepeat::Yes){
	    brush = (brush - 1).max(1);
	    changed = true;
	}
	if changed{
	    update_title(&mut window, selected, brush);
	}

        // mouse
	if let Some((mx, my)) = window.get_mouse_pos(MouseMode::Discard){
	    let cx = mx as i32;
	    let cy = my as i32;

	    if window.get_mouse_down(MouseButton::Left){
	        pour(&mut world, cx, cy, brush, selected);
	    }
	    if window.get_mouse_down(MouseButton::Right){
	        pour(&mut world, cx, cy, brush, Material::Air);
	    }
	}

        // simulation of world
	world.update();

	// drawing world -> buffer -> window
	world.draw(&mut buffer);
	window
	    .update_with_buffer(&buffer, WIDTH, HEIGHT)
	    .expect("Could not update window");
    }
}
