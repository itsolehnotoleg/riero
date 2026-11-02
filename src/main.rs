use minifb::{Key, Scale, Window, WindowOptions};

const WIDTH: usize = 320;
const HEIGHT: usize = 200;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
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
        for y in 0..HEIGHT{
	    for x in 0..WIDTH{
                let i = y * WIDTH + x;
		buffer[i] = if y > 140{0x8B5A2B} else {0x87CEEB};
	    }
	}
	window
	    .update_with_buffer(&buffer, WIDTH, HEIGHT)
	    .expect("Could not update window");
    }
}
